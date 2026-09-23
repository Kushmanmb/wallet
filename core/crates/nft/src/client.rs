use std::collections::{HashMap, HashSet};
use std::error::Error;

use primitives::nft::NFTAssetData;
use primitives::{AssetId, Chain, ImageFormatter, NFTAsset, NFTAssetId, NFTCollection, NFTCollectionId, NFTData};
use storage::database::devices::DevicesStore;
use storage::database::nft::{NftAssetFilter, NftCollectionFilter};
use storage::models::{NewNftAssetRow, NewNftCollectionRow, NewNftReportRow, NftCollectionRow, NftLinkRow};
use storage::{Database, DatabaseClient, DatabaseError, NftRepository, WalletsRepository};

use crate::NFTProviderConfig;
use crate::mapper::map_nft_data;
use crate::provider_client::NFTProviderClient;

pub struct NFTClient {
    database: Database,
    provider_client: NFTProviderClient,
    assets_url: String,
}

impl NFTClient {
    pub fn new(database: Database, provider_client: NFTProviderClient, assets_url: String) -> Self {
        Self { database, provider_client, assets_url }
    }

    pub fn from_config(database: Database, config: NFTProviderConfig, assets_url: String) -> Self {
        Self::new(database, NFTProviderClient::new(config), assets_url)
    }

    pub async fn update_collection(&self, collection_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        self.refresh_collection(collection_id.parse()?).await?;
        Ok(true)
    }

    pub async fn refresh_collection(&self, collection_id: NFTCollectionId) -> Result<NFTCollection, Box<dyn Error + Send + Sync>> {
        let collection = self.provider_client.get_nft_collection(collection_id).await?;
        self.upsert_collection(collection.clone()).await?;
        Ok(self.with_urls_collection(collection))
    }

    pub async fn refresh_asset(&self, asset_id: NFTAssetId) -> Result<(), Box<dyn Error + Send + Sync>> {
        let collection = self.provider_client.get_nft_collection(asset_id.get_collection_id()).await?;
        let collection_row = self.upsert_collection(collection).await?;
        let asset = self.provider_client.get_nft_asset(asset_id).await?;
        self.database.run(move |client| client.upsert_nft_asset(NewNftAssetRow::from_primitive(asset, collection_row.id))).await?;
        Ok(())
    }

    pub async fn get_wallet_assets(&self, device_id: i32, wallet_id: i32) -> Result<Vec<NFTData>, Box<dyn Error + Send + Sync>> {
        let subscriptions = self.database.run(move |client| client.get_subscriptions_by_wallet_id(device_id, wallet_id)).await?;

        let mut asset_ids: HashSet<NFTAssetId> = HashSet::new();
        for (sub, addr) in subscriptions {
            let chain = sub.chain.0;
            let ids = match self.provider_client.get_nft_asset_ids(chain, &addr.address).await {
                Ok(ids) => ids,
                Err(_) => self.database.run(move |client| Self::get_cached_asset_ids(client, addr.id, chain)).await?,
            };
            asset_ids.extend(ids);
        }

        self.preload(asset_ids.into_iter().collect()).await
    }

    pub async fn get_nft_asset_data(&self, asset_id: NFTAssetId) -> Result<NFTAssetData, Box<dyn Error + Send + Sync>> {
        let asset_id = asset_id.to_string();
        let (asset, collection) = self
            .database
            .run(move |client| -> Result<_, DatabaseError> {
                let asset = Self::load_nft_asset_with(client, &asset_id)?;
                let collection = Self::load_nft_collection_with(client, &asset.collection_id.to_string())?;
                Ok((asset, collection))
            })
            .await?;

        Ok(NFTAssetData {
            collection: self.with_urls_collection(collection),
            asset: self.with_urls_asset(asset),
        })
    }

    fn get_cached_asset_ids(client: &mut DatabaseClient, address_id: i32, chain: Chain) -> Result<Vec<NFTAssetId>, DatabaseError> {
        Ok(client
            .get_nft_assets_by_filter(vec![NftAssetFilter::AddressId(address_id)])?
            .into_iter()
            .filter(|row| row.chain.0 == chain)
            .map(|row| row.identifier.0)
            .collect())
    }

    fn with_urls_asset(&self, asset: NFTAsset) -> NFTAsset {
        let id = asset.id.to_string();
        let preview_url = ImageFormatter::get_nft_asset_url(&self.assets_url, &id);
        let resource_url = ImageFormatter::get_nft_asset_resource_url(&self.assets_url, &id);
        asset.with_urls(preview_url, resource_url)
    }

    fn with_urls_collection(&self, collection: NFTCollection) -> NFTCollection {
        let preview_url = ImageFormatter::get_nft_collection_url(&self.assets_url, &collection.id.to_string());
        collection.with_preview_url(preview_url)
    }

    fn with_urls_data(&self, data: NFTData) -> NFTData {
        NFTData {
            collection: self.with_urls_collection(data.collection),
            assets: data.assets.into_iter().map(|a| self.with_urls_asset(a)).collect(),
        }
    }

    async fn upsert_collection(&self, collection: NFTCollection) -> Result<NftCollectionRow, Box<dyn Error + Send + Sync>> {
        Ok(self
            .database
            .run(move |client| -> Result<_, DatabaseError> {
                let row = client.upsert_nft_collection(NewNftCollectionRow::from_primitive(collection.clone()))?;
                let links: Vec<NftLinkRow> = collection.links.into_iter().filter(|link| !link.url.is_empty()).filter_map(|link| NftLinkRow::from_primitive(row.id, link)).collect();
                client.set_nft_collection_links(row.id, links)?;
                Ok(row)
            })
            .await?)
    }

    pub async fn preload(&self, assets: Vec<NFTAssetId>) -> Result<Vec<NFTData>, Box<dyn Error + Send + Sync>> {
        let collection_ids: HashSet<NFTCollectionId> = assets.iter().map(|x| x.get_collection_id()).collect();
        let collection_id_map = self.preload_collections(collection_ids.into_iter().collect()).await?;
        self.preload_assets(assets.clone(), &collection_id_map).await?;
        self.get_nfts(assets).await
    }

    pub async fn preload_collections(&self, collection_ids: Vec<NFTCollectionId>) -> Result<HashMap<String, i32>, Box<dyn Error + Send + Sync>> {
        let identifiers: Vec<String> = collection_ids.iter().map(|x| x.to_string()).collect();
        let lookup = identifiers.clone();
        let existing = self.database.run(move |client| Self::get_nft_collection_id_map(client, &lookup)).await?;

        let mut new_collections: Vec<NFTCollection> = Vec::new();
        for id in collection_ids.into_iter().filter(|id| !existing.contains_key(&id.to_string())) {
            if let Ok(collection) = self.provider_client.get_nft_collection(id).await {
                new_collections.push(collection);
            }
        }

        if new_collections.is_empty() {
            return Ok(existing);
        }

        Ok(self
            .database
            .run(move |client| -> Result<_, DatabaseError> {
                let rows = new_collections.iter().cloned().map(NewNftCollectionRow::from_primitive).collect();
                client.add_nft_collections(rows)?;

                let map = Self::get_nft_collection_id_map(client, &identifiers)?;

                let links: Vec<NftLinkRow> = new_collections
                    .into_iter()
                    .flat_map(|collection| {
                        let pk = map.get(&collection.id.to_string()).copied();
                        collection.links.into_iter().filter(|link| !link.url.is_empty()).filter_map(move |link| pk.and_then(|pk| NftLinkRow::from_primitive(pk, link)))
                    })
                    .collect();
                client.add_nft_collections_links(links)?;

                Ok(map)
            })
            .await?)
    }

    pub async fn preload_assets(&self, asset_ids: Vec<NFTAssetId>, collection_id_map: &HashMap<String, i32>) -> Result<HashMap<String, i32>, Box<dyn Error + Send + Sync>> {
        let identifiers: Vec<String> = asset_ids.iter().map(|x| x.to_string()).collect();
        let lookup = identifiers.clone();
        let existing = self.database.run(move |client| Self::get_nft_asset_id_map(client, &lookup)).await?;

        let mut new_assets: Vec<NFTAsset> = Vec::new();
        for id in asset_ids.into_iter().filter(|id| !existing.contains_key(&id.to_string())) {
            if let Ok(asset) = self.provider_client.get_nft_asset(id).await {
                new_assets.push(asset);
            }
        }

        let rows: Vec<NewNftAssetRow> = new_assets
            .into_iter()
            .filter_map(|asset| collection_id_map.get(&asset.collection_id.to_string()).map(|&pk| NewNftAssetRow::from_primitive(asset, pk)))
            .collect();

        if rows.is_empty() {
            return Ok(existing);
        }

        Ok(self
            .database
            .run(move |client| -> Result<_, DatabaseError> {
                client.add_nft_assets(rows)?;
                Self::get_nft_asset_id_map(client, &identifiers)
            })
            .await?)
    }

    fn get_nft_collection_id_map(client: &mut DatabaseClient, identifiers: &[String]) -> Result<HashMap<String, i32>, DatabaseError> {
        Ok(client
            .get_nft_collections_by_filter(vec![NftCollectionFilter::Identifiers(identifiers.to_vec())])?
            .into_iter()
            .map(|c| (c.identifier.to_string(), c.id))
            .collect())
    }

    fn get_nft_asset_id_map(client: &mut DatabaseClient, identifiers: &[String]) -> Result<HashMap<String, i32>, DatabaseError> {
        Ok(client
            .get_nft_assets_by_filter(vec![NftAssetFilter::Identifiers(identifiers.to_vec())])?
            .into_iter()
            .map(|a| (a.identifier.to_string(), a.id))
            .collect())
    }

    fn load_nft_assets(client: &mut DatabaseClient, asset_identifiers: Vec<String>) -> Result<Vec<NFTAsset>, DatabaseError> {
        let assets = client.get_nft_assets_by_filter(vec![NftAssetFilter::Identifiers(asset_identifiers)])?;
        let collection_ids: Vec<i32> = assets.iter().map(|a| a.collection_id).collect::<HashSet<_>>().into_iter().collect();
        let collection_identifiers: HashMap<i32, NFTCollectionId> = client.get_nft_collections_by_filter(vec![NftCollectionFilter::Ids(collection_ids)])?.into_iter().map(|c| (c.id, c.identifier.0)).collect();
        Ok(assets
            .into_iter()
            .filter_map(|row| {
                let identifier = collection_identifiers.get(&row.collection_id).cloned()?;
                Some(row.as_primitive(identifier.into()))
            })
            .collect())
    }

    fn load_nfts(client: &mut DatabaseClient, asset_identifiers: Vec<String>) -> Result<Vec<NFTData>, DatabaseError> {
        let assets = Self::load_nft_assets(client, asset_identifiers)?;
        let collection_ids = assets.iter().map(|asset| asset.collection_id.to_string()).collect::<HashSet<_>>().into_iter().collect();
        let collections = client
            .get_nft_collections_by_filter(vec![NftCollectionFilter::Identifiers(collection_ids)])?
            .into_iter()
            .map(|row| {
                let links = client.get_nft_collection_links(row.id)?.into_iter().map(|link| link.as_primitive()).collect();
                Ok(row.as_primitive(links))
            })
            .collect::<Result<Vec<_>, DatabaseError>>()?;
        Ok(map_nft_data(assets, collections))
    }

    async fn get_nfts(&self, assets: Vec<NFTAssetId>) -> Result<Vec<NFTData>, Box<dyn Error + Send + Sync>> {
        let identifiers = assets.into_iter().map(|asset| asset.to_string()).collect();
        let nfts = self.database.run(move |client| Self::load_nfts(client, identifiers)).await?;
        Ok(nfts.into_iter().map(|data| self.with_urls_data(data)).collect())
    }

    fn load_nft_asset_with(client: &mut DatabaseClient, asset_id: &str) -> Result<NFTAsset, DatabaseError> {
        Self::load_nft_assets(client, vec![asset_id.to_string()])?.into_iter().next().ok_or_else(|| DatabaseError::not_found("NftAsset", asset_id))
    }

    fn load_nft_collection_with(client: &mut DatabaseClient, collection_id: &str) -> Result<NFTCollection, DatabaseError> {
        let row = client.get_nft_collection(collection_id)?;
        let links = client.get_nft_collection_links(row.id)?.into_iter().map(|x| x.as_primitive()).collect();
        Ok(row.as_primitive(links))
    }

    pub async fn load_nft_asset(&self, asset_id: &str) -> Result<NFTAsset, Box<dyn Error + Send + Sync>> {
        let asset_id = asset_id.to_string();
        Ok(self.database.run(move |client| Self::load_nft_asset_with(client, &asset_id)).await?)
    }

    pub async fn load_nft_collection(&self, collection_id: &str) -> Result<NFTCollection, Box<dyn Error + Send + Sync>> {
        let collection_id = collection_id.to_string();
        Ok(self.database.run(move |client| Self::load_nft_collection_with(client, &collection_id)).await?)
    }

    pub async fn update_assets_for_addresses(&self, addresses: HashMap<Chain, String>) -> Result<Vec<NFTData>, Box<dyn Error + Send + Sync>> {
        let address_values: Vec<String> = addresses.values().cloned().collect();
        let address_id_map: HashMap<String, i32> = self.database.run(move |client| client.get_addresses(address_values)).await?.into_iter().map(|row| (row.address, row.id)).collect();

        let mut all_asset_ids: HashSet<NFTAssetId> = HashSet::new();
        let mut owned_by_address: HashMap<i32, HashSet<NFTAssetId>> = HashMap::new();
        let mut chains_by_address: HashMap<i32, HashSet<Chain>> = HashMap::new();
        for (chain, address) in addresses {
            let Ok(ids) = self.provider_client.get_nft_asset_ids(chain, &address).await else {
                continue;
            };
            if let Some(&address_id) = address_id_map.get(&address) {
                chains_by_address.entry(address_id).or_default().insert(chain);
                owned_by_address.entry(address_id).or_default().extend(ids.iter().cloned());
            }
            all_asset_ids.extend(ids);
        }

        let asset_ids: Vec<NFTAssetId> = all_asset_ids.into_iter().collect();
        let collection_ids: Vec<NFTCollectionId> = asset_ids.iter().map(|x| x.get_collection_id()).collect::<HashSet<_>>().into_iter().collect();
        let collection_id_map = self.preload_collections(collection_ids).await?;
        let asset_id_map = self.preload_assets(asset_ids.clone(), &collection_id_map).await?;

        let associations: Vec<(i32, Vec<Chain>, Vec<i32>)> = owned_by_address
            .into_iter()
            .map(|(address_id, owned)| {
                let current_asset_ids: Vec<i32> = owned.into_iter().filter_map(|id| asset_id_map.get(&id.to_string()).copied()).collect();
                let chains: Vec<Chain> = chains_by_address.remove(&address_id).unwrap_or_default().into_iter().collect();
                (address_id, chains, current_asset_ids)
            })
            .collect();
        self.database
            .run(move |client| -> Result<(), DatabaseError> {
                for (address_id, chains, current_asset_ids) in associations {
                    client.set_nft_asset_associations(address_id, chains, current_asset_ids)?;
                }
                Ok(())
            })
            .await?;

        self.get_nfts(asset_ids).await
    }

    pub async fn report_nft(&self, device_id: &str, collection_id: String, asset_id: Option<AssetId>, reason: Option<String>) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let device_id = device_id.to_string();
        self.database
            .run(move |client| -> Result<(), Box<dyn Error + Send + Sync>> {
                let device = DevicesStore::get_device(client, &device_id)?;
                let collection_pk = client.get_nft_collection(&collection_id)?.id;
                let asset_pk = asset_id.and_then(|id| client.get_nft_asset(&id.to_string()).ok().map(|row| row.id));

                client.add_nft_report(NewNftReportRow {
                    device_id: device.id,
                    collection_id: collection_pk,
                    asset_id: asset_pk,
                    reason,
                })?;
                Ok(())
            })
            .await?;
        Ok(true)
    }
}
