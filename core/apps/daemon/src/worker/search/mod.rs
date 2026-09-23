mod asset_lists_index_updater;
mod assets_index_updater;
mod nfts_index_updater;
mod perpetuals_index_updater;
mod sync;

use crate::model::WorkerService;
use crate::worker::context::WorkerContext;
use crate::worker::jobs::WorkerJob;
use asset_lists_index_updater::AssetListsIndexUpdater;
use assets_index_updater::AssetsIndexUpdater;
use config_keys::ConfigKey;
use job_runner::{JobHandle, ShutdownReceiver};
use nfts_index_updater::NftsIndexUpdater;
use perpetuals_index_updater::PerpetualsIndexUpdater;
use std::error::Error;

pub async fn jobs(ctx: WorkerContext, shutdown_rx: ShutdownReceiver) -> Result<Vec<JobHandle>, Box<dyn Error + Send + Sync>> {
    let services = ctx.services();
    let database = services.database();
    let config = services.config();

    let primary_price_max_age = config.get_duration(ConfigKey::PricePrimaryMaxAge).await?;
    let search_index_client = services.search_index().await?;
    ctx.plan_builder(WorkerService::Search, &config, shutdown_rx)
        .job(WorkerJob::UpdateAssetsIndex, {
            let database = database.clone();
            let search_index_client = search_index_client.clone();
            move |_| {
                let updater = AssetsIndexUpdater::new(database.clone(), &search_index_client, primary_price_max_age);
                async move { updater.update().await }
            }
        })
        .job(WorkerJob::UpdateAssetListsIndex, {
            let database = database.clone();
            let search_index_client = search_index_client.clone();
            move |_| {
                let updater = AssetListsIndexUpdater::new(database.clone(), &search_index_client);
                async move { updater.update().await }
            }
        })
        .job(WorkerJob::UpdatePerpetualsIndex, {
            let database = database.clone();
            let search_index_client = search_index_client.clone();
            move |_| {
                let updater = PerpetualsIndexUpdater::new(database.clone(), &search_index_client);
                async move { updater.update().await }
            }
        })
        .job(WorkerJob::UpdateNftsIndex, {
            let database = database.clone();
            let search_index_client = search_index_client.clone();
            move |_| {
                let updater = NftsIndexUpdater::new(database.clone(), &search_index_client);
                async move { updater.update().await }
            }
        })
        .finish()
        .await
}
