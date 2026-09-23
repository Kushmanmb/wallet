use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel::upsert::excluded;
use primitives::currency::Currency;
use primitives::{AssetId, FiatProviderCountry, FiatProviderName, FiatRate, FiatTransaction, FiatTransactionUpdate};

use crate::models::*;
use crate::schema::fiat_providers;
use crate::sql_types::{AssetId as AssetIdRow, Currency as CurrencyRow, FiatProviderNameRow};
use crate::{DatabaseClient, DatabaseError, DieselResultExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiatAssetFilter {
    HasAssetId,
    Provider(FiatProviderName),
    IsEnabled(bool),
    IsEnabledByProvider(bool),
    IsBuyEnabled(bool),
    IsSellEnabled(bool),
    ProviderEnabled(bool),
    ProviderBuyEnabled(bool),
    ProviderSellEnabled(bool),
}

#[derive(Debug, Clone)]
pub enum FiatAssetUpdate {
    IsEnabled(bool),
    IsEnabledByProvider(bool),
    IsBuyEnabled(bool),
    IsSellEnabled(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiatProviderCountryFilter {
    Provider(FiatProviderName),
    IsAllowed(bool),
}

#[derive(Debug, Clone)]
pub enum FiatProviderCountryUpdate {
    IsAllowed(bool),
}

pub trait FiatRepository {
    fn add_fiat_assets(&mut self, values: Vec<crate::models::FiatAssetRow>) -> Result<usize, DatabaseError>;
    fn update_fiat_assets(&mut self, asset_ids: Vec<String>, updates: Vec<FiatAssetUpdate>) -> Result<usize, DatabaseError>;
    fn add_fiat_providers(&mut self, values: Vec<crate::models::FiatProviderRow>) -> Result<usize, DatabaseError>;
    fn add_fiat_providers_countries(&mut self, values: Vec<crate::models::FiatProviderCountryRow>) -> Result<usize, DatabaseError>;
    fn update_fiat_providers_countries(&mut self, country_ids: Vec<String>, updates: Vec<FiatProviderCountryUpdate>) -> Result<usize, DatabaseError>;
    fn get_fiat_providers_countries(&mut self) -> Result<Vec<FiatProviderCountry>, DatabaseError>;
    fn get_fiat_providers_countries_by_filter(&mut self, filters: Vec<FiatProviderCountryFilter>) -> Result<Vec<crate::models::FiatProviderCountryRow>, DatabaseError>;
    fn get_fiat_transactions_by_device_id(&mut self, device_id: i32) -> Result<Vec<FiatTransaction>, DatabaseError>;
    fn get_fiat_transactions_by_device_and_wallet_id(&mut self, device_id: i32, wallet_id: i32) -> Result<Vec<FiatTransaction>, DatabaseError>;
    fn count_fiat_transactions_by_device_and_wallet_id(&mut self, device_id: i32, wallet_id: i32) -> Result<i64, DatabaseError>;
    fn get_fiat_assets_by_filter(&mut self, filters: Vec<FiatAssetFilter>) -> Result<Vec<crate::models::FiatAssetRow>, DatabaseError>;
    fn get_fiat_assets_popular(&mut self, from: NaiveDateTime, limit: i64) -> Result<Vec<AssetId>, DatabaseError>;
    fn get_fiat_assets_for_asset_id(&mut self, asset_id: &AssetId) -> Result<Vec<crate::models::FiatAssetRow>, DatabaseError>;
    fn set_fiat_rates(&mut self, rates: Vec<crate::models::FiatRateRow>) -> Result<usize, DatabaseError>;
    fn get_fiat_rates(&mut self) -> Result<Vec<FiatRate>, DatabaseError>;
    fn get_fiat_rate(&mut self, currency: &Currency) -> Result<FiatRate, DatabaseError>;
    fn get_fiat_providers(&mut self) -> Result<Vec<crate::models::FiatProviderRow>, DatabaseError>;
    fn update_fiat_provider_payment_methods(&mut self, provider_id: FiatProviderName, values: serde_json::Value) -> Result<usize, DatabaseError>;
    fn update_fiat_transaction(&mut self, provider: FiatProviderName, update: FiatTransactionUpdate) -> Result<FiatTransactionRow, DatabaseError>;
    fn get_fiat_transaction(&mut self, provider: FiatProviderName, transaction_id: &str) -> Result<Option<FiatTransactionRow>, DatabaseError>;
    fn add_fiat_transaction(&mut self, transaction: NewFiatTransactionRow) -> Result<usize, DatabaseError>;
}

fn add_fiat_assets(client: &mut DatabaseClient, values: Vec<FiatAssetRow>) -> Result<usize, diesel::result::Error> {
    use crate::schema::fiat_assets::dsl::*;
    diesel::insert_into(fiat_assets)
        .values(values)
        .on_conflict(id)
        .do_update()
        .set((
            asset_id.eq(excluded(asset_id)),
            symbol.eq(excluded(symbol)),
            network.eq(excluded(network)),
            token_id.eq(excluded(token_id)),
            unsupported_countries.eq(excluded(unsupported_countries)),
            buy_limits.eq(excluded(buy_limits)),
            sell_limits.eq(excluded(sell_limits)),
            is_buy_enabled.eq(excluded(is_buy_enabled)),
            is_sell_enabled.eq(excluded(is_sell_enabled)),
            is_enabled_by_provider.eq(excluded(is_enabled_by_provider)),
        ))
        .execute(&mut client.connection)
}

fn add_fiat_providers(client: &mut DatabaseClient, values: Vec<FiatProviderRow>) -> Result<usize, diesel::result::Error> {
    use crate::schema::fiat_providers::dsl::*;
    diesel::insert_into(fiat_providers).values(values).on_conflict_do_nothing().execute(&mut client.connection)
}

fn add_fiat_providers_countries(client: &mut DatabaseClient, values: Vec<FiatProviderCountryRow>) -> Result<usize, diesel::result::Error> {
    use crate::schema::fiat_providers_countries::dsl::*;
    diesel::insert_into(fiat_providers_countries)
        .values(values)
        .on_conflict(id)
        .do_update()
        .set((alpha2.eq(excluded(alpha2)), is_allowed.eq(excluded(is_allowed))))
        .execute(&mut client.connection)
}

fn get_fiat_providers_countries(client: &mut DatabaseClient) -> Result<Vec<FiatProviderCountryRow>, diesel::result::Error> {
    use crate::schema::fiat_providers_countries::dsl::*;
    fiat_providers_countries.select(FiatProviderCountryRow::as_select()).load(&mut client.connection)
}

fn get_fiat_transactions_by_device_and_wallet_id(client: &mut DatabaseClient, device_id_value: i32, wallet_id_value: i32) -> Result<Vec<FiatTransactionRow>, diesel::result::Error> {
    use crate::schema::fiat_transactions;

    fiat_transactions::table
        .filter(fiat_transactions::device_id.eq(device_id_value))
        .filter(fiat_transactions::wallet_id.eq(wallet_id_value))
        .order(fiat_transactions::created_at.desc())
        .select(FiatTransactionRow::as_select())
        .load(&mut client.connection)
}

fn get_fiat_transactions_by_device_id(client: &mut DatabaseClient, device_id_value: i32) -> Result<Vec<FiatTransactionRow>, diesel::result::Error> {
    use crate::schema::fiat_transactions;

    fiat_transactions::table
        .filter(fiat_transactions::device_id.eq(device_id_value))
        .order(fiat_transactions::created_at.desc())
        .select(FiatTransactionRow::as_select())
        .load(&mut client.connection)
}

fn get_fiat_assets_popular(client: &mut DatabaseClient, from: NaiveDateTime, limit: i64) -> Result<Vec<AssetIdRow>, diesel::result::Error> {
    use crate::schema::fiat_transactions::dsl::*;

    fiat_transactions
        .filter(created_at.gt(from))
        .select(asset_id)
        .group_by(asset_id)
        .order_by(count_star().desc())
        .limit(limit)
        .load::<AssetIdRow>(&mut client.connection)
}

fn get_fiat_assets_for_asset_id(client: &mut DatabaseClient, requested_asset_id: &str) -> Result<Vec<FiatAssetRow>, diesel::result::Error> {
    use crate::schema::fiat_assets::dsl::*;
    fiat_assets::table()
        .inner_join(fiat_providers::table)
        .filter(fiat_providers::enabled.eq(true))
        .filter(asset_id.eq(requested_asset_id))
        .select(FiatAssetRow::as_select())
        .load(&mut client.connection)
}

fn set_fiat_rates(client: &mut DatabaseClient, rates: Vec<FiatRateRow>) -> Result<usize, diesel::result::Error> {
    use crate::schema::fiat_rates::dsl::*;
    use diesel::query_dsl::methods::FilterDsl;
    let query = diesel::insert_into(fiat_rates).values(&rates).on_conflict(id).do_update().set(rate.eq(excluded(rate)));
    query.filter(provider.eq(excluded(provider))).execute(&mut client.connection)
}

fn get_fiat_rates(client: &mut DatabaseClient) -> Result<Vec<FiatRateRow>, diesel::result::Error> {
    use crate::schema::fiat_rates::dsl::*;
    fiat_rates.filter(is_enabled.eq(true)).select(FiatRateRow::as_select()).load(&mut client.connection)
}

fn get_fiat_rate(client: &mut DatabaseClient, currency: &Currency) -> Result<FiatRateRow, diesel::result::Error> {
    use crate::schema::fiat_rates::dsl::*;
    fiat_rates.find(CurrencyRow(currency.clone())).filter(is_enabled.eq(true)).select(FiatRateRow::as_select()).first(&mut client.connection)
}

fn update_by_provider_transaction_id(client: &mut DatabaseClient, provider: &FiatProviderNameRow, provider_transaction_id_value: &str, changeset: &UpdateFiatTransactionRow) -> Result<Option<FiatTransactionRow>, diesel::result::Error> {
    use crate::schema::fiat_transactions::dsl::*;

    diesel::update(fiat_transactions.filter(provider_id.eq(provider)).filter(provider_transaction_id.eq(provider_transaction_id_value)))
        .set(changeset)
        .returning(FiatTransactionRow::as_returning())
        .get_result(&mut client.connection)
        .optional()
}

fn update_by_quote_id(
    client: &mut DatabaseClient,
    provider: &FiatProviderNameRow,
    target_quote_id: &str,
    provider_transaction_id_value: &str,
    changeset: &UpdateFiatTransactionRow,
) -> Result<Option<FiatTransactionRow>, diesel::result::Error> {
    use crate::schema::fiat_transactions::dsl::*;

    diesel::update(fiat_transactions.filter(provider_id.eq(provider)).filter(quote_id.eq(target_quote_id)).filter(provider_transaction_id.is_null()))
        .set((provider_transaction_id.eq(provider_transaction_id_value), changeset))
        .returning(FiatTransactionRow::as_returning())
        .get_result(&mut client.connection)
        .optional()
}

fn get_fiat_transaction_for_quote(client: &mut DatabaseClient, provider: &FiatProviderNameRow, target_quote_id: &str) -> Result<Option<FiatTransactionRow>, diesel::result::Error> {
    use crate::schema::fiat_transactions::dsl::*;

    fiat_transactions
        .filter(provider_id.eq(provider))
        .filter(quote_id.eq(target_quote_id))
        .order((created_at.desc(), id.desc()))
        .select(FiatTransactionRow::as_select())
        .first(&mut client.connection)
        .optional()
}

fn update_fiat_transaction_by_id(client: &mut DatabaseClient, transaction_id: i32, changeset: UpdateFiatTransactionRow) -> Result<FiatTransactionRow, diesel::result::Error> {
    use crate::schema::fiat_transactions::dsl::*;

    diesel::update(fiat_transactions.find(transaction_id))
        .set(changeset)
        .returning(FiatTransactionRow::as_returning())
        .get_result(&mut client.connection)
}

impl FiatRepository for DatabaseClient {
    fn add_fiat_assets(&mut self, values: Vec<crate::models::FiatAssetRow>) -> Result<usize, DatabaseError> {
        Ok(add_fiat_assets(self, values)?)
    }

    fn update_fiat_assets(&mut self, asset_ids: Vec<String>, updates: Vec<FiatAssetUpdate>) -> Result<usize, DatabaseError> {
        if asset_ids.is_empty() || updates.is_empty() {
            return Ok(0);
        }
        use crate::schema::fiat_assets::dsl::*;
        Ok(updates.into_iter().try_fold(0, |total, update| {
            let target = fiat_assets.filter(id.eq_any(&asset_ids));
            let updated = match update {
                FiatAssetUpdate::IsEnabled(value) => diesel::update(target).set(is_enabled.eq(value)).execute(&mut self.connection)?,
                FiatAssetUpdate::IsEnabledByProvider(value) => diesel::update(target).set(is_enabled_by_provider.eq(value)).execute(&mut self.connection)?,
                FiatAssetUpdate::IsBuyEnabled(value) => diesel::update(target).set(is_buy_enabled.eq(value)).execute(&mut self.connection)?,
                FiatAssetUpdate::IsSellEnabled(value) => diesel::update(target).set(is_sell_enabled.eq(value)).execute(&mut self.connection)?,
            };
            Ok::<_, diesel::result::Error>(total + updated)
        })?)
    }

    fn add_fiat_providers(&mut self, values: Vec<crate::models::FiatProviderRow>) -> Result<usize, DatabaseError> {
        Ok(add_fiat_providers(self, values)?)
    }

    fn add_fiat_providers_countries(&mut self, values: Vec<crate::models::FiatProviderCountryRow>) -> Result<usize, DatabaseError> {
        Ok(add_fiat_providers_countries(self, values)?)
    }

    fn update_fiat_providers_countries(&mut self, country_ids: Vec<String>, updates: Vec<FiatProviderCountryUpdate>) -> Result<usize, DatabaseError> {
        if country_ids.is_empty() || updates.is_empty() {
            return Ok(0);
        }
        use crate::schema::fiat_providers_countries::dsl::*;
        Ok(updates.into_iter().try_fold(0, |total, update| {
            let target = fiat_providers_countries.filter(id.eq_any(&country_ids));
            let updated = match update {
                FiatProviderCountryUpdate::IsAllowed(value) => diesel::update(target).set(is_allowed.eq(value)).execute(&mut self.connection)?,
            };
            Ok::<_, diesel::result::Error>(total + updated)
        })?)
    }

    fn get_fiat_providers_countries_by_filter(&mut self, filters: Vec<FiatProviderCountryFilter>) -> Result<Vec<crate::models::FiatProviderCountryRow>, DatabaseError> {
        use crate::schema::fiat_providers_countries::dsl::*;
        let mut query = fiat_providers_countries.into_boxed();
        for filter in filters {
            query = match filter {
                FiatProviderCountryFilter::Provider(value) => query.filter(provider.eq(FiatProviderNameRow::from(value))),
                FiatProviderCountryFilter::IsAllowed(value) => query.filter(is_allowed.eq(value)),
            };
        }
        Ok(query.select(FiatProviderCountryRow::as_select()).load(&mut self.connection)?)
    }

    fn get_fiat_providers_countries(&mut self) -> Result<Vec<FiatProviderCountry>, DatabaseError> {
        let result = get_fiat_providers_countries(self)?;
        Ok(result.into_iter().map(|x| x.as_primitive()).collect())
    }

    fn get_fiat_transactions_by_device_id(&mut self, device_id: i32) -> Result<Vec<FiatTransaction>, DatabaseError> {
        let result = get_fiat_transactions_by_device_id(self, device_id)?;
        result.into_iter().map(|row| row.as_primitive()).collect()
    }

    fn get_fiat_transactions_by_device_and_wallet_id(&mut self, device_id: i32, wallet_id: i32) -> Result<Vec<FiatTransaction>, DatabaseError> {
        let result = get_fiat_transactions_by_device_and_wallet_id(self, device_id, wallet_id)?;
        result.into_iter().map(|row| row.as_primitive()).collect()
    }

    fn count_fiat_transactions_by_device_and_wallet_id(&mut self, device_id_value: i32, wallet_id_value: i32) -> Result<i64, DatabaseError> {
        use crate::schema::fiat_transactions;

        Ok(fiat_transactions::table
            .filter(fiat_transactions::device_id.eq(device_id_value))
            .filter(fiat_transactions::wallet_id.eq(wallet_id_value))
            .count()
            .get_result(&mut self.connection)?)
    }

    fn get_fiat_assets_by_filter(&mut self, filters: Vec<FiatAssetFilter>) -> Result<Vec<crate::models::FiatAssetRow>, DatabaseError> {
        use crate::schema::{fiat_assets, fiat_providers};

        let mut query = fiat_assets::table.inner_join(fiat_providers::table).into_boxed();

        for filter in filters {
            query = match filter {
                FiatAssetFilter::HasAssetId => query.filter(fiat_assets::asset_id.is_not_null()),
                FiatAssetFilter::Provider(value) => query.filter(fiat_assets::provider.eq(FiatProviderNameRow::from(value))),
                FiatAssetFilter::IsEnabled(value) => query.filter(fiat_assets::is_enabled.eq(value)),
                FiatAssetFilter::IsEnabledByProvider(value) => query.filter(fiat_assets::is_enabled_by_provider.eq(value)),
                FiatAssetFilter::IsBuyEnabled(value) => query.filter(fiat_assets::is_buy_enabled.eq(value)),
                FiatAssetFilter::IsSellEnabled(value) => query.filter(fiat_assets::is_sell_enabled.eq(value)),
                FiatAssetFilter::ProviderEnabled(value) => query.filter(fiat_providers::enabled.eq(value)),
                FiatAssetFilter::ProviderBuyEnabled(value) => query.filter(fiat_providers::buy_enabled.eq(value)),
                FiatAssetFilter::ProviderSellEnabled(value) => query.filter(fiat_providers::sell_enabled.eq(value)),
            };
        }

        Ok(query.select(FiatAssetRow::as_select()).distinct().order(fiat_assets::asset_id.asc()).load(&mut self.connection)?)
    }

    fn get_fiat_assets_popular(&mut self, from: NaiveDateTime, limit: i64) -> Result<Vec<AssetId>, DatabaseError> {
        Ok(get_fiat_assets_popular(self, from, limit)?.into_iter().map(Into::into).collect())
    }

    fn get_fiat_assets_for_asset_id(&mut self, asset_id: &AssetId) -> Result<Vec<crate::models::FiatAssetRow>, DatabaseError> {
        Ok(get_fiat_assets_for_asset_id(self, &asset_id.to_string())?)
    }

    fn set_fiat_rates(&mut self, rates: Vec<crate::models::FiatRateRow>) -> Result<usize, DatabaseError> {
        Ok(set_fiat_rates(self, rates)?)
    }

    fn get_fiat_rates(&mut self) -> Result<Vec<FiatRate>, DatabaseError> {
        let result = get_fiat_rates(self)?;
        Ok(result.into_iter().map(|x| x.as_primitive()).collect())
    }

    fn get_fiat_rate(&mut self, currency: &Currency) -> Result<FiatRate, DatabaseError> {
        let result = get_fiat_rate(self, currency).or_not_found(currency.to_string())?;
        Ok(result.as_primitive())
    }

    fn get_fiat_providers(&mut self) -> Result<Vec<crate::models::FiatProviderRow>, DatabaseError> {
        use crate::schema::fiat_providers::dsl::*;
        Ok(fiat_providers.select(FiatProviderRow::as_select()).load(&mut self.connection)?)
    }

    fn update_fiat_provider_payment_methods(&mut self, provider_id_value: FiatProviderName, values: serde_json::Value) -> Result<usize, DatabaseError> {
        use crate::schema::fiat_providers::dsl::*;
        Ok(diesel::update(fiat_providers.filter(id.eq(FiatProviderNameRow::from(provider_id_value))))
            .set(payment_methods.eq(values))
            .execute(&mut self.connection)?)
    }

    fn update_fiat_transaction(&mut self, provider: FiatProviderName, update: FiatTransactionUpdate) -> Result<FiatTransactionRow, DatabaseError> {
        use crate::schema::fiat_transactions::dsl::*;

        let provider = FiatProviderNameRow::from(provider);
        let changeset = UpdateFiatTransactionRow::from_primitive(&update);

        if let Some(row) = update_by_provider_transaction_id(self, &provider, &update.transaction_id, &changeset)? {
            return Ok(row);
        }

        Ok(match update.provider_transaction_id.as_deref() {
            Some(provider_transaction_id_value) => {
                if let Some(row) = update_by_provider_transaction_id(self, &provider, provider_transaction_id_value, &changeset)? {
                    return Ok(row);
                }
                if let Some(row) = update_by_quote_id(self, &provider, &update.transaction_id, provider_transaction_id_value, &changeset)? {
                    return Ok(row);
                }
                let existing = get_fiat_transaction_for_quote(self, &provider, &update.transaction_id)?.ok_or(diesel::result::Error::NotFound)?;
                let new_row = NewFiatTransactionRow::from_existing(&existing, &update, provider_transaction_id_value.to_string());
                diesel::insert_into(fiat_transactions).values(&new_row).returning(FiatTransactionRow::as_returning()).get_result(&mut self.connection)
            }
            None => {
                let existing = get_fiat_transaction_for_quote(self, &provider, &update.transaction_id)?.ok_or(diesel::result::Error::NotFound)?;
                update_fiat_transaction_by_id(self, existing.id, changeset)
            }
        }?)
    }

    fn get_fiat_transaction(&mut self, provider: FiatProviderName, transaction_id: &str) -> Result<Option<FiatTransactionRow>, DatabaseError> {
        use crate::schema::fiat_transactions::dsl::*;

        let provider = FiatProviderNameRow::from(provider);
        Ok(fiat_transactions
            .filter(provider_id.eq(&provider))
            .filter(provider_transaction_id.eq(transaction_id).or(quote_id.eq(transaction_id)))
            .order((updated_at.desc(), id.desc()))
            .select(FiatTransactionRow::as_select())
            .first(&mut self.connection)
            .optional()?)
    }

    fn add_fiat_transaction(&mut self, transaction: NewFiatTransactionRow) -> Result<usize, DatabaseError> {
        use crate::schema::fiat_transactions::dsl::*;

        Ok(diesel::insert_into(fiat_transactions).values(&transaction).on_conflict_do_nothing().execute(&mut self.connection)?)
    }
}
