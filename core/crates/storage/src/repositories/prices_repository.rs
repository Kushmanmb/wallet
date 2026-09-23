use std::collections::{HashMap, HashSet};
use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::{Nullable, SingleValue, SqlType};
use diesel::upsert::excluded;
use primitives::{AssetId, ChartTimeframe, Price, PriceId, PriceProvider};

use crate::error::ResourceName;
use crate::models::min_max::MinMax;
use crate::models::{ChartRow, PriceAssetRow, PriceProviderConfigRow, PriceRow, price::NewPriceRow, price::PriceAssetDataRow, price::PricesChangeset};
use crate::repositories::assets_repository::{all_asset_ids, asset_ids_updated_since, asset_rows};
use crate::repositories::charts_repository::{ChartsRepository, chart_extremes};
use crate::repositories::prices_providers_repository::PricesProvidersRepository;
use crate::sql_types::PriceProviderRow;
use crate::{DatabaseClient, DatabaseError, DieselResultExt};

diesel::define_sql_function! {
    fn coalesce<T: SqlType + SingleValue>(a: Nullable<T>, b: Nullable<T>) -> Nullable<T>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetsWithPricesFilter {
    Ids(Vec<String>),
    UpdatedSince(NaiveDateTime),
}

#[derive(Debug, Clone)]
pub enum PriceFilter {
    Provider(PriceProvider),
    UpdatedBefore(NaiveDateTime),
    UpdatedAfter(NaiveDateTime),
    Ids(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum PriceUpdate {
    AllTimeHigh { value: f64, date: Option<NaiveDateTime> },
    AllTimeLow { value: f64, date: Option<NaiveDateTime> },
    PriceChangePercentage24h(f64),
}

pub trait PricesRepository {
    fn add_prices(&mut self, values: Vec<NewPriceRow>) -> Result<usize, DatabaseError>;
    fn set_prices(&mut self, prices: Vec<PriceRow>) -> Result<Vec<AssetId>, DatabaseError>;
    fn set_prices_assets(&mut self, values: Vec<PriceAssetRow>) -> Result<usize, DatabaseError>;
    fn get_prices_by_filter(&mut self, filters: Vec<PriceFilter>) -> Result<Vec<PriceRow>, DatabaseError>;
    fn get_prices_assets(&mut self) -> Result<Vec<PriceAssetRow>, DatabaseError>;
    fn get_prices_asset_ids(&mut self) -> Result<Vec<AssetId>, DatabaseError>;
    fn get_prices_assets_by_provider(&mut self, provider: PriceProvider) -> Result<Vec<PriceAssetRow>, DatabaseError>;
    fn get_primary_price_key(&mut self, asset_id: &AssetId, max_age: Duration) -> Result<PriceId, DatabaseError>;
    fn get_primary_prices(&mut self, asset_ids: &[AssetId], max_age: Duration) -> Result<Vec<(AssetId, PriceRow)>, DatabaseError>;
    fn get_price_by_id(&mut self, price_id: &str) -> Result<Price, DatabaseError>;
    fn get_prices_for_asset(&mut self, asset_id: &AssetId) -> Result<Vec<PriceRow>, DatabaseError>;
    fn get_prices_assets_for_price_ids(&mut self, ids: Vec<String>) -> Result<Vec<PriceAssetRow>, DatabaseError>;
    fn delete_prices(&mut self, ids: Vec<String>) -> Result<usize, DatabaseError>;
    fn get_assets_with_prices(&mut self, filters: Vec<AssetsWithPricesFilter>, max_age: Duration) -> Result<Vec<PriceAssetDataRow>, DatabaseError>;
    fn update_prices(&mut self, price_ids: Vec<String>, updates: Vec<PriceUpdate>) -> Result<usize, DatabaseError>;
    fn update_extremes_for_price(&mut self, price_id: &str) -> Result<usize, DatabaseError>;
}

fn prices_for_asset_ids(client: &mut DatabaseClient, asset_ids: &[String]) -> Result<Vec<(String, PriceRow)>, diesel::result::Error> {
    use crate::schema::{prices, prices_assets};

    prices_assets::table
        .inner_join(prices::table.on(prices_assets::price_id.eq(prices::id)))
        .filter(prices_assets::asset_id.eq_any(asset_ids))
        .select((prices_assets::asset_id, PriceRow::as_select()))
        .load::<(crate::sql_types::AssetId, PriceRow)>(&mut client.connection)
        .map(|rows| rows.into_iter().map(|(id, row)| (id.0.to_string(), row)).collect())
}

fn price_row(client: &mut DatabaseClient, price_id: &str) -> Result<PriceRow, diesel::result::Error> {
    use crate::schema::prices::dsl::*;
    prices.filter(id.eq(price_id)).select(PriceRow::as_select()).first(&mut client.connection)
}

fn price_asset_ids_updated_since(client: &mut DatabaseClient, since: NaiveDateTime) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::{prices, prices_assets};

    prices_assets::table
        .inner_join(prices::table.on(prices_assets::price_id.eq(prices::id)))
        .filter(prices::last_updated_at.gt(since))
        .select(prices_assets::asset_id)
        .load::<crate::sql_types::AssetId>(&mut client.connection)
        .map(|ids| ids.into_iter().map(|id| id.0.to_string()).collect())
}

fn upsert_prices(client: &mut DatabaseClient, values: Vec<PriceRow>) -> Result<usize, diesel::result::Error> {
    use crate::schema::prices::dsl::*;
    if values.is_empty() {
        return Ok(0);
    }
    diesel::insert_into(prices)
        .values(&values)
        .on_conflict(id)
        .do_update()
        .set((
            price.eq(excluded(price)),
            price_change_percentage_24h.eq(coalesce(excluded(price_change_percentage_24h), price_change_percentage_24h)),
            market_cap_rank.eq(excluded(market_cap_rank)),
            total_volume.eq(excluded(total_volume)),
            last_updated_at.eq(excluded(last_updated_at)),
        ))
        .execute(&mut client.connection)
}

impl PricesRepository for DatabaseClient {
    fn add_prices(&mut self, values: Vec<NewPriceRow>) -> Result<usize, DatabaseError> {
        use crate::schema::prices::dsl::*;
        if values.is_empty() {
            return Ok(0);
        }
        Ok(diesel::insert_into(prices).values(&values).on_conflict_do_nothing().execute(&mut self.connection)?)
    }

    fn set_prices_assets(&mut self, values: Vec<PriceAssetRow>) -> Result<usize, DatabaseError> {
        use crate::schema::prices_assets::dsl::*;
        if values.is_empty() {
            return Ok(0);
        }
        Ok(diesel::insert_into(prices_assets)
            .values(&values)
            .on_conflict((asset_id, provider))
            .do_update()
            .set(price_id.eq(excluded(price_id)))
            .execute(&mut self.connection)?)
    }

    fn get_prices_by_filter(&mut self, filters: Vec<PriceFilter>) -> Result<Vec<PriceRow>, DatabaseError> {
        use crate::schema::prices::dsl::*;
        let query = filters.into_iter().fold(prices.into_boxed(), |q, filter| match filter {
            PriceFilter::Provider(p) => q.filter(provider.eq(PriceProviderRow::from(p))),
            PriceFilter::UpdatedBefore(time) => q.filter(last_updated_at.lt(time).or(last_updated_at.is_null())),
            PriceFilter::UpdatedAfter(time) => q.filter(last_updated_at.ge(time)),
            PriceFilter::Ids(ids) => q.filter(id.eq_any(ids)),
        });
        Ok(query.order(market_cap_rank.asc().nulls_last()).select(PriceRow::as_select()).load(&mut self.connection)?)
    }

    fn get_prices_assets(&mut self) -> Result<Vec<PriceAssetRow>, DatabaseError> {
        use crate::schema::prices_assets::dsl::*;
        Ok(prices_assets.select(PriceAssetRow::as_select()).load(&mut self.connection)?)
    }

    fn get_prices_asset_ids(&mut self) -> Result<Vec<AssetId>, DatabaseError> {
        use crate::schema::prices_assets::dsl::*;
        let ids: Vec<String> = prices_assets.select(asset_id).load(&mut self.connection)?;
        Ok(ids.into_iter().filter_map(|value| AssetId::new(&value)).collect())
    }

    fn get_prices_assets_by_provider(&mut self, price_provider: PriceProvider) -> Result<Vec<PriceAssetRow>, DatabaseError> {
        use crate::schema::prices_assets::dsl::*;
        Ok(prices_assets.filter(provider.eq(PriceProviderRow::from(price_provider))).select(PriceAssetRow::as_select()).load(&mut self.connection)?)
    }

    fn get_primary_price_key(&mut self, asset_id: &AssetId, max_age: Duration) -> Result<PriceId, DatabaseError> {
        let providers = self.get_prices_providers()?;
        let rows = prices_for_asset_ids(self, &[asset_id.to_string()])?.into_iter().map(|(_, row)| row).collect::<Vec<_>>();
        Ok(primary_price(&providers, &rows, max_age).ok_or_else(|| DatabaseError::not_found(PriceRow::RESOURCE_NAME, asset_id.to_string()))?.id.0.clone())
    }

    fn get_primary_prices(&mut self, asset_ids: &[AssetId], max_age: Duration) -> Result<Vec<(AssetId, PriceRow)>, DatabaseError> {
        if asset_ids.is_empty() {
            return Ok(vec![]);
        }
        let providers = self.get_prices_providers()?;
        let string_ids: Vec<String> = asset_ids.iter().map(|id| id.to_string()).collect();
        let mut rows_by_asset: HashMap<String, Vec<PriceRow>> = prices_for_asset_ids(self, &string_ids)?.into_iter().fold(HashMap::new(), |mut acc, (id, row)| {
            acc.entry(id).or_default().push(row);
            acc
        });
        Ok(asset_ids
            .iter()
            .filter_map(|asset_id| {
                let rows = rows_by_asset.remove(&asset_id.to_string())?;
                let row = primary_price(&providers, &rows, max_age)?.clone();
                Some((asset_id.clone(), row))
            })
            .collect())
    }

    fn get_price_by_id(&mut self, price_id: &str) -> Result<Price, DatabaseError> {
        Ok(price_row(self, price_id).or_not_found(price_id.to_string())?.as_primitive())
    }

    fn get_prices_for_asset(&mut self, asset_id: &AssetId) -> Result<Vec<PriceRow>, DatabaseError> {
        Ok(prices_for_asset_ids(self, &[asset_id.to_string()])?.into_iter().map(|(_, row)| row).collect())
    }

    fn get_prices_assets_for_price_ids(&mut self, ids: Vec<String>) -> Result<Vec<PriceAssetRow>, DatabaseError> {
        use crate::schema::prices_assets::dsl::*;
        Ok(prices_assets.filter(price_id.eq_any(ids)).select(PriceAssetRow::as_select()).load(&mut self.connection)?)
    }

    fn delete_prices(&mut self, ids: Vec<String>) -> Result<usize, DatabaseError> {
        use crate::schema::prices::dsl::*;
        if ids.is_empty() {
            return Ok(0);
        }
        Ok(diesel::delete(prices.filter(id.eq_any(ids))).execute(&mut self.connection)?)
    }

    fn update_prices(&mut self, price_ids: Vec<String>, updates: Vec<PriceUpdate>) -> Result<usize, DatabaseError> {
        if updates.is_empty() {
            return Ok(0);
        }
        let changeset = PricesChangeset::from_updates(updates);
        use crate::schema::prices::dsl::*;
        if price_ids.is_empty() {
            return Ok(0);
        }
        Ok(diesel::update(prices.filter(id.eq_any(&price_ids))).set(&changeset).execute(&mut self.connection)?)
    }

    fn update_extremes_for_price(&mut self, price_id: &str) -> Result<usize, DatabaseError> {
        let row = price_row(self, price_id).or_not_found(price_id.to_string())?;
        let timeframes = [ChartTimeframe::Raw, ChartTimeframe::Hourly, ChartTimeframe::Daily];
        let extremes: Vec<MinMax<f64>> = timeframes.into_iter().map(|tf| chart_extremes(self, price_id, tf)).collect::<Result<_, _>>()?;
        let combined = MinMax {
            max: extremes.iter().filter_map(|e| e.max).max_by(|a, b| a.value.total_cmp(&b.value)),
            min: extremes.iter().filter_map(|e| e.min).min_by(|a, b| a.value.total_cmp(&b.value)),
        };
        let updates = row.merge_extremes_from_charts(combined);
        self.update_prices(vec![price_id.to_string()], updates)
    }

    fn set_prices(&mut self, prices: Vec<PriceRow>) -> Result<Vec<AssetId>, DatabaseError> {
        if prices.is_empty() {
            return Ok(vec![]);
        }
        let price_ids: Vec<String> = prices.iter().map(|p| p.id.to_string()).collect();
        let mappings = self.get_prices_assets_for_price_ids(price_ids)?;
        let mapped_ids: HashSet<String> = mappings.iter().map(|m| m.price_id.to_string()).collect();
        let to_store: Vec<PriceRow> = prices.into_iter().filter(|p| mapped_ids.contains(&p.id.to_string())).collect();
        if to_store.is_empty() {
            return Ok(vec![]);
        }
        let ids: Vec<String> = to_store.iter().map(|p| p.id.to_string()).collect();
        let incoming_by_id: HashMap<String, PriceRow> = to_store.iter().cloned().map(|p| (p.id.to_string(), p)).collect();
        upsert_prices(self, to_store)?;

        let current_prices = self.get_prices_by_filter(vec![PriceFilter::Ids(ids)])?;
        let extreme_updates: Vec<(String, Vec<PriceUpdate>)> = current_prices
            .iter()
            .filter_map(|price| {
                let id = price.id.to_string();
                let updates = price.merge_extremes(incoming_by_id.get(&id));
                (!updates.is_empty()).then_some((id, updates))
            })
            .collect();
        for (id, updates) in extreme_updates {
            self.update_prices(vec![id], updates)?;
        }

        let charts: Vec<ChartRow> = current_prices.iter().cloned().map(ChartRow::from_price).collect();
        self.add_charts(ChartTimeframe::Raw, charts)?;

        Ok(mappings.into_iter().map(|m| m.asset_id.0).collect::<HashSet<_>>().into_iter().collect())
    }

    fn get_assets_with_prices(&mut self, filters: Vec<AssetsWithPricesFilter>, max_age: Duration) -> Result<Vec<PriceAssetDataRow>, DatabaseError> {
        let mut ids = None;
        let mut since = None;
        for filter in filters {
            match filter {
                AssetsWithPricesFilter::Ids(values) => ids = Some(values),
                AssetsWithPricesFilter::UpdatedSince(value) => since = Some(value),
            }
        }

        let asset_ids = match since {
            Some(value) => {
                let mut asset_ids = asset_ids_updated_since(self, value)?;
                asset_ids.extend(price_asset_ids_updated_since(self, value)?);
                asset_ids.sort();
                asset_ids.dedup();
                if let Some(ids) = ids {
                    let ids = ids.into_iter().collect::<HashSet<_>>();
                    asset_ids.retain(|id| ids.contains(id));
                }
                asset_ids
            }
            None => match ids {
                Some(ids) => ids,
                None => all_asset_ids(self)?,
            },
        };

        if asset_ids.is_empty() {
            return Ok(vec![]);
        }

        let providers = self.get_prices_providers()?;
        let assets = asset_rows(self, asset_ids)?;
        let mut prices_by_asset: HashMap<String, Vec<PriceRow>> = prices_for_asset_ids(self, &assets.iter().map(|a| a.id.clone()).collect::<Vec<_>>())?
            .into_iter()
            .fold(HashMap::new(), |mut acc, (asset_id, row)| {
                acc.entry(asset_id).or_default().push(row);
                acc
            });

        Ok(assets
            .into_iter()
            .map(|asset| {
                let rows = prices_by_asset.remove(&asset.id).unwrap_or_default();
                let price = primary_price(&providers, &rows, max_age).cloned();
                PriceAssetDataRow { asset, price }
            })
            .collect())
    }
}

fn primary_price<'a>(providers: &[PriceProviderConfigRow], rows: &'a [PriceRow], max_age: Duration) -> Option<&'a PriceRow> {
    let cutoff = (Utc::now() - chrono::Duration::from_std(max_age).ok()?).naive_utc();
    let mut candidates: Vec<(&PriceProviderConfigRow, &PriceRow)> = providers
        .iter()
        .filter(|p| p.enabled)
        .filter_map(|p| rows.iter().find(|row| row.provider.0 == p.id.0).map(|row| (p, row)))
        .filter(|(_, row)| row.last_updated_at >= cutoff)
        .collect();
    candidates.sort_by_key(|(p, _)| p.priority);
    candidates.first().map(|(_, row)| *row)
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::HOUR;

    #[test]
    fn test_primary_price() {
        let providers = vec![
            PriceProviderConfigRow::new(PriceProvider::Coingecko, true),
            PriceProviderConfigRow::new(PriceProvider::Pyth, true),
            PriceProviderConfigRow::new(PriceProvider::Jupiter, false),
        ];
        let max_age = HOUR;

        let fresh = vec![PriceRow::mock_with_age(PriceProvider::Coingecko, 60), PriceRow::mock_with_age(PriceProvider::Pyth, 60)];
        assert_eq!(primary_price(&providers, &fresh, max_age).unwrap().provider.0, PriceProvider::Coingecko);

        let stale_primary = vec![PriceRow::mock_with_age(PriceProvider::Coingecko, 7200), PriceRow::mock_with_age(PriceProvider::Pyth, 60)];
        assert_eq!(primary_price(&providers, &stale_primary, max_age).unwrap().provider.0, PriceProvider::Pyth);

        let only_disabled = vec![PriceRow::mock_with_age(PriceProvider::Jupiter, 60)];
        assert!(primary_price(&providers, &only_disabled, max_age).is_none());

        assert!(primary_price(&providers, &[], max_age).is_none());
    }
}
