use crate::model::WorkerService;
use crate::worker::context::WorkerContext;
use crate::worker::jobs::WorkerJob;
use fiat_assets_updater::FiatAssetsUpdater;
use fiat_rates_updater::FiatRatesUpdater;
use job_runner::{JobHandle, ShutdownReceiver};
use prices::{FiatRatesProviderConfig, build_fiat_rates_providers};
use primitives::FiatProviderName;
use std::error::Error;

mod fiat_assets_updater;
mod fiat_rates_updater;

pub async fn jobs(ctx: WorkerContext, shutdown_rx: ShutdownReceiver) -> Result<Vec<JobHandle>, Box<dyn Error + Send + Sync>> {
    let services = ctx.services();
    let settings = services.settings();
    let config = services.config();

    let cacher_client = services.cacher().await?;
    let access_token_cacher = services.fiat_access_token_cacher().await?;
    let providers = build_fiat_rates_providers(&FiatRatesProviderConfig {
        coingecko: settings.coingecko.remote_provider_config(),
        coinmarketcap: settings.coinmarketcap.remote_provider_config(),
    });
    let new_assets_updater = {
        let services = services.clone();
        move || FiatAssetsUpdater::new(services.database(), services.fiat_providers(access_token_cacher.clone()))
    };

    ctx.plan_builder(WorkerService::Fiat, &config, shutdown_rx)
        .jobs(WorkerJob::UpdateFiatRates, providers.keys().copied(), |provider, _| {
            let provider = providers[&provider].clone();
            let price_client = services.prices(cacher_client.clone());
            move |_| {
                let updater = FiatRatesUpdater::new(provider.clone(), price_client.clone());
                async move { updater.update().await }
            }
        })
        .jobs(WorkerJob::UpdateFiatAssets, FiatProviderName::all(), |provider, _| {
            let new_assets_updater = new_assets_updater.clone();
            move |_| {
                let updater = new_assets_updater();
                async move { updater.update_fiat_assets_for(provider).await }
            }
        })
        .jobs(WorkerJob::UpdateFiatProviderCountries, FiatProviderName::all(), |provider, _| {
            let new_assets_updater = new_assets_updater.clone();
            move |_| {
                let updater = new_assets_updater();
                async move { updater.update_fiat_countries_for(provider).await }
            }
        })
        .job(WorkerJob::UpdateFiatBuyableAssets, {
            let new_assets_updater = new_assets_updater.clone();
            move |_| {
                let updater = new_assets_updater();
                async move { updater.update_buyable_assets().await }
            }
        })
        .job(WorkerJob::UpdateFiatSellableAssets, {
            let new_assets_updater = new_assets_updater.clone();
            move |_| {
                let updater = new_assets_updater();
                async move { updater.update_sellable_assets().await }
            }
        })
        .job(WorkerJob::UpdateTrendingFiatAssets, {
            let new_assets_updater = new_assets_updater.clone();
            move |_| {
                let updater = new_assets_updater();
                async move { updater.update_trending_fiat_assets().await }
            }
        })
        .finish()
        .await
}
