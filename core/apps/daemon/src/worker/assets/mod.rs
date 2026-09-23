mod asset_rank_updater;
mod assets_has_price_updater;
mod assets_images_updater;
mod perpetual_updater;
mod staking_apy_updater;
mod usage_rank_updater;
mod validator_scanner;

use std::error::Error;
use std::sync::Arc;

use asset_rank_updater::AssetRankUpdater;
use assets_has_price_updater::AssetsHasPriceUpdater;
use assets_images_updater::AssetsImagesUpdater;
use config_keys::ConfigKey;
use job_runner::{JobHandle, ShutdownReceiver};
use perpetual_updater::PerpetualUpdater;
use primitives::Chain;
use services::StaticAssetsClient;
use services::assets::AssetClassificationRules;
use settings::service_user_agent;
use staking_apy_updater::StakeApyUpdater;
use usage_rank_updater::{UsageRankUpdater, UsageRankUpdaterConfig};
use validator_scanner::ValidatorScanner;

use crate::model::WorkerService;
use crate::worker::context::WorkerContext;
use crate::worker::jobs::WorkerJob;

pub async fn jobs(ctx: WorkerContext, shutdown_rx: ShutdownReceiver) -> Result<Vec<JobHandle>, Box<dyn Error + Send + Sync>> {
    let services = ctx.services();
    let database = services.database();
    let settings = services.settings();
    let config = services.config();
    let classification_rules = AssetClassificationRules::from_config(&config).await?;
    let usage_rank_updater_config = UsageRankUpdaterConfig {
        batch_size: config.get_usize(ConfigKey::AssetsUsageRankBatchSize).await?,
    };
    ctx.plan_builder(WorkerService::Assets, &config, shutdown_rx)
        .job(WorkerJob::UpdateSuspiciousAssetRanks, {
            let database = database.clone();
            let classification_rules = classification_rules.clone();
            move |_| {
                let suspicious_updater = AssetRankUpdater::new(database.clone(), classification_rules.clone());
                async move { suspicious_updater.update_suspicious_assets().await }
            }
        })
        .jobs(WorkerJob::UpdatePerpetuals, PerpetualUpdater::chains(), |chain, _| {
            let chain = *chain;
            let settings = settings.clone();
            let database = database.clone();
            move |_| {
                let settings = settings.clone();
                let database = database.clone();
                async move {
                    let updater = PerpetualUpdater::new((*settings.as_ref()).clone(), database.clone());
                    updater.update_chain(chain).await
                }
            }
        })
        .job(WorkerJob::UpdateUsageRanks, {
            let database = database.clone();
            move |_| {
                let updater = UsageRankUpdater::new(database.clone(), usage_rank_updater_config);
                async move { updater.update_usage_ranks().await }
            }
        })
        .jobs(WorkerJob::UpdateAssetsImages, Chain::all(), |chain, _| {
            let static_assets_client = StaticAssetsClient::new(&settings.assets.url);
            let database = database.clone();
            move |_| {
                let updater = AssetsImagesUpdater::new(static_assets_client.clone(), database.clone());
                async move { updater.update_chain(chain).await }
            }
        })
        .job(WorkerJob::UpdateAssetsHasPrice, {
            let database = database.clone();
            move |_| {
                let updater = AssetsHasPriceUpdater::new(database.clone());
                async move { updater.update().await }
            }
        })
        .jobs(WorkerJob::UpdateStakeApy, Chain::stakeable(), {
            let services = services.clone();
            let database = database.clone();
            move |chain, _| {
                let providers = Arc::new(services.chain_providers_for(chain, &service_user_agent("daemon", Some("staking_apy"))));
                let database = database.clone();
                move |_| {
                    let updater = StakeApyUpdater::new(providers.clone(), database.clone());
                    async move { updater.update_chain(chain).await }
                }
            }
        })
        .jobs(WorkerJob::UpdateChainValidators, Chain::stakeable(), {
            let services = services.clone();
            let database = database.clone();
            move |chain, _| {
                let providers = Arc::new(services.chain_providers_for(chain, &service_user_agent("daemon", Some("scan_validators"))));
                let database = database.clone();
                move |_| {
                    let scanner = ValidatorScanner::new(providers.clone(), database.clone());
                    async move { scanner.update_validators_for_chain(chain).await }
                }
            }
        })
        .jobs(WorkerJob::UpdateValidatorsFromStaticAssets, [Chain::Tron, Chain::SmartChain], {
            let services = services.clone();
            let settings = settings.clone();
            let database = database.clone();
            move |chain, _| {
                let providers = Arc::new(services.chain_providers_for(chain, &service_user_agent("daemon", Some("scan_static_assets"))));
                let assets_url = settings.assets.url.clone();
                let database = database.clone();
                move |_| {
                    let scanner = ValidatorScanner::new(providers.clone(), database.clone());
                    let assets_url = assets_url.clone();
                    async move { scanner.update_validators_from_static_assets_for_chain(chain, &assets_url).await }
                }
            }
        })
        .finish()
        .await
}
