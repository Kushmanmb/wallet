use tokio::task::spawn_blocking;

mod config_cacher;
pub mod database;
pub mod error;
pub mod models;
pub mod repositories;
pub mod schema;
pub mod sql_types;
#[cfg(any(test, feature = "testkit"))]
pub mod testkit;

pub use config_cacher::ConfigCacher;

diesel::allow_columns_to_appear_in_same_group_by_clause!(schema::transactions_addresses::address, schema::transactions::chain,);

pub use self::database::{
    DatabaseClient,
    assets::{AssetFilter, AssetUpdate},
    charts::ChartFilter,
    fiat::{FiatAssetFilter, FiatAssetUpdate, FiatProviderCountryFilter, FiatProviderCountryUpdate},
    nft::{NftAssetFilter, NftCollectionFilter},
    prices::{AssetsWithPricesFilter, PriceUpdate},
    referrals::{AbusePatterns, ReferralUpdate},
    rewards::{RewardsFilter, RewardsUpdate},
    rewards_redemptions::RedemptionUpdate,
    transactions::{TransactionFilter, TransactionUpdate},
};
pub use self::error::{DatabaseError, DieselResultExt, ReferralValidationError, UsernameValidationError};
pub use self::models::{ApiClientGrant, ApiClientResource, ApiClientRow, ApiClientScope, AssetUsageRankRow, FiatAssetRowsExt, NewNotificationRow, NewSupportSessionRow, NewWalletRow, RewardRedemptionOptionRow};
pub use self::repositories::{
    api_clients_repository::ApiClientsRepository,
    assets_addresses_repository::AssetsAddressesRepository,
    assets_links_repository::AssetsLinksRepository,
    assets_repository::AssetsRepository,
    assets_usage_ranks_repository::AssetsUsageRanksRepository,
    chains_repository::ChainsRepository,
    charts_repository::ChartsRepository,
    config_repository::ConfigRepository,
    devices_repository::DevicesRepository,
    fiat_repository::FiatRepository,
    migrations_repository::MigrationsRepository,
    nft_repository::NftRepository,
    notifications_repository::NotificationsRepository,
    parser_state_repository::ParserStateRepository,
    perpetuals_repository::PerpetualsRepository,
    price_alerts_repository::PriceAlertsRepository,
    prices_providers_repository::PricesProvidersRepository,
    prices_repository::PricesRepository,
    releases_repository::ReleasesRepository,
    rewards_redemptions_repository::RewardsRedemptionsRepository,
    rewards_repository::{ReferrerInfo, RewardsEligibilityConfig, RewardsRepository},
    risk_signals_repository::RiskSignalsRepository,
    scan_addresses_repository::ScanAddressesRepository,
    scan_detections_repository::ScanDetectionsRepository,
    support_sessions_repository::SupportSessionsRepository,
    tag_repository::TagRepository,
    transactions_repository::TransactionsRepository,
    wallets_repository::WalletsRepository,
};
pub use self::sql_types::{NotificationType, TransactionState, TransactionType, WalletSource, WalletType};
pub use diesel::OptionalExtension;

#[derive(Clone)]
pub struct Database(database::PgPool);

impl Database {
    pub fn new(database_url: &str, pool_size: u32) -> Result<Self, DatabaseError> {
        Ok(Self(database::create_pool(database_url, pool_size)?))
    }

    pub async fn run<T, E, F>(&self, operation: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: From<DatabaseError> + Send + 'static,
        F: FnOnce(&mut DatabaseClient) -> Result<T, E> + Send + 'static,
    {
        let pool = self.0.clone();
        match spawn_blocking(move || operation(&mut DatabaseClient::from_pool(&pool)?)).await {
            Ok(result) => result,
            Err(error) => Err(DatabaseError::from(error).into()),
        }
    }

    pub async fn transaction<T, E, F>(&self, operation: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: From<DatabaseError> + Send + 'static,
        F: FnOnce(&mut DatabaseClient) -> Result<T, E> + Send + 'static,
    {
        self.run(move |client| client.transaction(operation)).await
    }
}
