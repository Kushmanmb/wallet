use chrono::NaiveDateTime;
use diesel::prelude::*;
use primitives::{Chain, Diff};

use crate::models::{NewNftAssetAssociationRow, NewNftAssetRow, NewNftCollectionRow, NftAssetRow, NftCollectionRow, nft_link::NftLinkRow, nft_report::NewNftReportRow};
use crate::sql_types::ChainRow;
use crate::{DatabaseClient, DatabaseError, DieselResultExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NftCollectionFilter {
    UpdatedSince(NaiveDateTime),
    Ids(Vec<i32>),
    Identifiers(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NftAssetFilter {
    Identifiers(Vec<String>),
    AddressId(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NftAssetAssociationFilter {
    AddressId(i32),
    Chains(Vec<Chain>),
}

pub trait NftRepository {
    fn get_nft_assets_by_filter(&mut self, filters: Vec<NftAssetFilter>) -> Result<Vec<NftAssetRow>, DatabaseError>;
    fn get_nft_asset(&mut self, identifier: &str) -> Result<NftAssetRow, DatabaseError>;
    fn add_nft_assets(&mut self, values: Vec<NewNftAssetRow>) -> Result<usize, DatabaseError>;
    fn upsert_nft_asset(&mut self, value: NewNftAssetRow) -> Result<NftAssetRow, DatabaseError>;
    fn get_nft_collections_by_filter(&mut self, filters: Vec<NftCollectionFilter>) -> Result<Vec<NftCollectionRow>, DatabaseError>;
    fn get_nft_collection(&mut self, identifier: &str) -> Result<NftCollectionRow, DatabaseError>;
    fn get_nft_collection_links(&mut self, collection_id: i32) -> Result<Vec<NftLinkRow>, DatabaseError>;
    fn add_nft_collections(&mut self, values: Vec<NewNftCollectionRow>) -> Result<usize, DatabaseError>;
    fn upsert_nft_collection(&mut self, value: NewNftCollectionRow) -> Result<NftCollectionRow, DatabaseError>;
    fn add_nft_collections_links(&mut self, values: Vec<NftLinkRow>) -> Result<usize, DatabaseError>;
    fn set_nft_collection_links(&mut self, collection_id: i32, values: Vec<NftLinkRow>) -> Result<usize, DatabaseError>;
    fn add_nft_report(&mut self, report: NewNftReportRow) -> Result<usize, DatabaseError>;
    fn set_nft_asset_associations(&mut self, address_id: i32, chains: Vec<Chain>, asset_ids: Vec<i32>) -> Result<(), DatabaseError>;
    fn count_nft_assets_by_address_ids(&mut self, address_ids: Vec<i32>, chains: Vec<Chain>) -> Result<i64, DatabaseError>;
}

fn get_nft_asset(client: &mut DatabaseClient, _identifier: &str) -> Result<NftAssetRow, diesel::result::Error> {
    use crate::schema::nft_assets::dsl::*;
    nft_assets.filter(identifier.eq(_identifier)).select(NftAssetRow::as_select()).first(&mut client.connection)
}

fn get_nft_collection(client: &mut DatabaseClient, _identifier: &str) -> Result<NftCollectionRow, diesel::result::Error> {
    use crate::schema::nft_collections::dsl::*;
    nft_collections.filter(identifier.eq(_identifier)).select(NftCollectionRow::as_select()).first(&mut client.connection)
}

fn get_nft_asset_association_ids_by_filter(client: &mut DatabaseClient, filters: Vec<NftAssetAssociationFilter>) -> Result<Vec<i32>, diesel::result::Error> {
    use crate::schema::nft_assets::dsl::{chain as asset_chain, id as asset_pk, nft_assets};
    use crate::schema::nft_assets_associations::dsl::*;
    let mut query = nft_assets_associations.inner_join(nft_assets.on(asset_pk.eq(asset_id))).into_boxed();
    for filter in filters {
        match filter {
            NftAssetAssociationFilter::AddressId(value) => query = query.filter(address_id.eq(value)),
            NftAssetAssociationFilter::Chains(values) => query = query.filter(asset_chain.eq_any(values.into_iter().map(ChainRow::from).collect::<Vec<_>>())),
        }
    }
    query.select(asset_id).load(&mut client.connection)
}

fn add_nft_asset_associations(client: &mut DatabaseClient, values: Vec<NewNftAssetAssociationRow>) -> Result<usize, diesel::result::Error> {
    use crate::schema::nft_assets_associations::dsl::*;
    diesel::insert_into(nft_assets_associations).values(values).on_conflict((address_id, asset_id)).do_nothing().execute(&mut client.connection)
}

fn delete_nft_asset_associations(client: &mut DatabaseClient, _address_id: i32, asset_ids: Vec<i32>) -> Result<usize, diesel::result::Error> {
    use crate::schema::nft_assets_associations::dsl::*;
    diesel::delete(nft_assets_associations.filter(address_id.eq(_address_id)).filter(asset_id.eq_any(asset_ids))).execute(&mut client.connection)
}

impl NftRepository for DatabaseClient {
    fn get_nft_assets_by_filter(&mut self, filters: Vec<NftAssetFilter>) -> Result<Vec<NftAssetRow>, DatabaseError> {
        use crate::schema::nft_assets::dsl::*;
        use crate::schema::nft_assets_associations;

        let mut query = nft_assets.into_boxed();
        for filter in filters {
            match filter {
                NftAssetFilter::Identifiers(values) => query = query.filter(identifier.eq_any(values)),
                NftAssetFilter::AddressId(value) => {
                    query = query.filter(id.eq_any(nft_assets_associations::table.filter(nft_assets_associations::address_id.eq(value)).select(nft_assets_associations::asset_id)));
                }
            }
        }
        Ok(query.select(NftAssetRow::as_select()).load(&mut self.connection)?)
    }

    fn get_nft_asset(&mut self, identifier: &str) -> Result<NftAssetRow, DatabaseError> {
        get_nft_asset(self, identifier).or_not_found(identifier.to_string())
    }

    fn add_nft_assets(&mut self, values: Vec<NewNftAssetRow>) -> Result<usize, DatabaseError> {
        use crate::schema::nft_assets::dsl::*;
        Ok(diesel::insert_into(nft_assets).values(values).on_conflict_do_nothing().execute(&mut self.connection)?)
    }

    fn upsert_nft_asset(&mut self, value: NewNftAssetRow) -> Result<NftAssetRow, DatabaseError> {
        use crate::schema::nft_assets::dsl::*;
        Ok(diesel::insert_into(nft_assets)
            .values(&value)
            .on_conflict(identifier)
            .do_update()
            .set(&value)
            .returning(NftAssetRow::as_returning())
            .get_result(&mut self.connection)?)
    }

    fn get_nft_collections_by_filter(&mut self, filters: Vec<NftCollectionFilter>) -> Result<Vec<NftCollectionRow>, DatabaseError> {
        use crate::schema::nft_collections::dsl::*;
        let mut query = nft_collections.into_boxed();
        for filter in filters {
            match filter {
                NftCollectionFilter::UpdatedSince(value) => query = query.filter(updated_at.gt(value)),
                NftCollectionFilter::Ids(values) => query = query.filter(id.eq_any(values)),
                NftCollectionFilter::Identifiers(values) => query = query.filter(identifier.eq_any(values)),
            }
        }
        Ok(query.select(NftCollectionRow::as_select()).load(&mut self.connection)?)
    }

    fn get_nft_collection(&mut self, identifier: &str) -> Result<NftCollectionRow, DatabaseError> {
        get_nft_collection(self, identifier).or_not_found(identifier.to_string())
    }

    fn get_nft_collection_links(&mut self, _collection_id: i32) -> Result<Vec<NftLinkRow>, DatabaseError> {
        use crate::schema::nft_collections_links::dsl::*;
        Ok(nft_collections_links.filter(collection_id.eq(_collection_id)).select(NftLinkRow::as_select()).load(&mut self.connection)?)
    }

    fn add_nft_collections(&mut self, values: Vec<NewNftCollectionRow>) -> Result<usize, DatabaseError> {
        use crate::schema::nft_collections::dsl::*;
        Ok(diesel::insert_into(nft_collections).values(values).on_conflict_do_nothing().execute(&mut self.connection)?)
    }

    fn upsert_nft_collection(&mut self, value: NewNftCollectionRow) -> Result<NftCollectionRow, DatabaseError> {
        use crate::schema::nft_collections::dsl::*;
        Ok(diesel::insert_into(nft_collections)
            .values(&value)
            .on_conflict(identifier)
            .do_update()
            .set(&value)
            .returning(NftCollectionRow::as_returning())
            .get_result(&mut self.connection)?)
    }

    fn add_nft_collections_links(&mut self, values: Vec<NftLinkRow>) -> Result<usize, DatabaseError> {
        use crate::schema::nft_collections_links::dsl::*;
        Ok(diesel::insert_into(nft_collections_links).values(values).on_conflict((collection_id, link_type)).do_nothing().execute(&mut self.connection)?)
    }

    fn set_nft_collection_links(&mut self, collection_row_id: i32, values: Vec<NftLinkRow>) -> Result<usize, DatabaseError> {
        use crate::schema::nft_collections_links::dsl::*;
        diesel::delete(nft_collections_links.filter(collection_id.eq(collection_row_id))).execute(&mut self.connection)?;
        if values.is_empty() {
            return Ok(0);
        }
        self.add_nft_collections_links(values)
    }

    fn add_nft_report(&mut self, report: NewNftReportRow) -> Result<usize, DatabaseError> {
        use crate::schema::nft_reports::dsl::*;
        Ok(diesel::insert_into(nft_reports).values(report).on_conflict_do_nothing().execute(&mut self.connection)?)
    }

    fn set_nft_asset_associations(&mut self, address_id: i32, chains: Vec<Chain>, asset_ids: Vec<i32>) -> Result<(), DatabaseError> {
        let existing = get_nft_asset_association_ids_by_filter(self, vec![NftAssetAssociationFilter::AddressId(address_id), NftAssetAssociationFilter::Chains(chains)])?;
        let diff = Diff::compare(asset_ids, existing);

        let to_insert: Vec<NewNftAssetAssociationRow> = diff.different.into_iter().map(|asset_id| NewNftAssetAssociationRow { address_id, asset_id }).collect();

        if !to_insert.is_empty() {
            add_nft_asset_associations(self, to_insert)?;
        }
        if !diff.missing.is_empty() {
            delete_nft_asset_associations(self, address_id, diff.missing)?;
        }
        Ok(())
    }

    fn count_nft_assets_by_address_ids(&mut self, address_ids: Vec<i32>, chains: Vec<Chain>) -> Result<i64, DatabaseError> {
        use crate::schema::nft_assets::dsl::{chain as asset_chain, id as asset_pk, nft_assets};
        use crate::schema::nft_assets_associations::dsl::*;

        if address_ids.is_empty() || chains.is_empty() {
            return Ok(0);
        }

        Ok(nft_assets_associations
            .inner_join(nft_assets.on(asset_pk.eq(asset_id)))
            .filter(address_id.eq_any(address_ids))
            .filter(asset_chain.eq_any(chains.into_iter().map(ChainRow::from).collect::<Vec<_>>()))
            .select(diesel::dsl::count(asset_id).aggregate_distinct())
            .first(&mut self.connection)?)
    }
}
