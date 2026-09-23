use std::error::Error;

use crate::model::AssetAddressChanges;
use async_trait::async_trait;
use cacher::{CacheKey, CacherClient};
use chain_providers::ChainProviders;
use storage::AssetsAddressesRepository;
use storage::{Database, DatabaseError};
use streamer::{ChainAddressPayload, consumer::MessageConsumer};

pub struct FetchCoinAddressesConsumer {
    pub provider: ChainProviders,
    pub database: Database,
    pub cacher: CacherClient,
}

impl FetchCoinAddressesConsumer {
    pub fn new(provider: ChainProviders, database: Database, cacher: CacherClient) -> Self {
        Self { provider, database, cacher }
    }
}

#[async_trait]
impl MessageConsumer<ChainAddressPayload, String> for FetchCoinAddressesConsumer {
    async fn should_process(&self, payload: &ChainAddressPayload) -> Result<bool, Box<dyn Error + Send + Sync>> {
        self.cacher.can_process_cached(CacheKey::FetchCoinAddresses(payload.value.chain.as_ref(), &payload.value.address)).await
    }

    async fn process(&self, payload: ChainAddressPayload) -> Result<String, Box<dyn Error + Send + Sync>> {
        let chain_address = payload.value;
        let balance = self.provider.get_balance_coin(chain_address.chain, chain_address.address.clone()).await?;
        let balance_value = balance.balance.available.to_string();
        let changes = AssetAddressChanges::from_coin_balance(&chain_address, balance);

        self.database
            .run(move |client| -> Result<_, DatabaseError> {
                client.delete_assets_addresses(changes.addresses_to_delete)?;
                client.add_assets_addresses(changes.addresses_to_add)
            })
            .await?;

        Ok(balance_value)
    }
}
