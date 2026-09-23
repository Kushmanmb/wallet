mod in_transit_updater;
mod pending_transactions_updater;
mod vault_addresses_updater;

use chain_providers::ProviderFactory;
use config_keys::{ConfigKey, ConfigParamKey};
use in_transit_updater::{InTransitConfig, InTransitUpdater};
use job_runner::{JobHandle, ShutdownReceiver};
use pending_transactions_updater::{PendingTransactionsUpdater, PendingTransactionsUpdaterConfig};
use primitives::{JobConfiguration, SwapProvider};
use settings::service_user_agent;
use std::error::Error;
use std::sync::Arc;
use swapper::NativeProvider;
use swapper::swapper::GemSwapper;
use vault_addresses_updater::VaultAddressesUpdater;

use crate::model::WorkerService;
use crate::worker::context::WorkerContext;
use crate::worker::jobs::WorkerJob;
use services::transactions::SwapVaultAddressClient;

pub async fn jobs(ctx: WorkerContext, shutdown_rx: ShutdownReceiver) -> Result<Vec<JobHandle>, Box<dyn Error + Send + Sync>> {
    let services = ctx.services();
    let database = services.database();
    let settings = services.settings();
    let config = services.config();

    let in_transit_config = InTransitConfig {
        timeout: config.get_duration(ConfigKey::TransactionInTransitTimeout).await?,
        query_limit: config.get_i64(ConfigKey::TransactionInTransitQueryLimit).await?,
        check_interval: JobConfiguration {
            initial_interval_ms: config.get_duration(ConfigKey::TransactionTimerInTransitUpdate).await?.as_millis() as u32,
            max_interval_ms: config.get_duration(ConfigKey::TransactionInTransitMaxCheckInterval).await?.as_millis() as u32,
            step_factor: config.get_f64(ConfigKey::TransactionInTransitCheckIntervalFactor).await? as f32,
        },
    };
    let pending_config = PendingTransactionsUpdaterConfig::from_config(&config).await?;

    let endpoints = ProviderFactory::get_chain_endpoints(&settings);
    let providers = Arc::new(services.chain_providers(&service_user_agent("daemon", Some("transactions"))));
    let swapper = Arc::new(GemSwapper::new(Arc::new(NativeProvider::new_with_endpoints(endpoints))));

    let stream_producer = services.stream_producer("transactions_worker", shutdown_rx.clone()).await?;
    let cacher = services.cacher().await?;
    let in_transit_updater = Arc::new(InTransitUpdater::new(
        database.clone(),
        in_transit_config,
        swapper.clone(),
        stream_producer.clone(),
        SwapVaultAddressClient::new(cacher.clone()),
    ));
    let pending_updater = Arc::new(PendingTransactionsUpdater::new(providers.clone(), cacher.clone(), stream_producer.clone(), database.clone(), pending_config));

    ctx.plan_builder(WorkerService::Transactions, &config, shutdown_rx)
        .job(WorkerJob::UpdateInTransitTransactions, {
            let updater = in_transit_updater.clone();
            move |_| {
                let updater = updater.clone();
                async move { updater.update().await }
            }
        })
        .job(WorkerJob::UpdatePendingTransactions, {
            let updater = pending_updater.clone();
            move |_| {
                let updater = updater.clone();
                async move { updater.update().await }
            }
        })
        .jobs_with_config(WorkerJob::UpdateSwapVaultAddresses, SwapProvider::cross_chain_providers(), ConfigParamKey::SwapperVaultAddresses, |provider, _| {
            let updater = Arc::new(VaultAddressesUpdater::new(swapper.clone(), cacher.clone()));
            move |ctx| {
                let updater = updater.clone();
                async move { updater.update(provider, ctx.last_success_at).await }
            }
        })
        .finish()
        .await
}
