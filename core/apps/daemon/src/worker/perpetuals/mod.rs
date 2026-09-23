mod perpetual_address_refresher;
pub(crate) mod perpetual_classifier;
mod perpetual_observer;

use config_keys::ConfigKey;
use job_runner::{JobHandle, ShutdownReceiver};
use perpetual_address_refresher::PerpetualAddressRefresher;
use perpetual_classifier::{PerpetualPositionClassifier, PerpetualPositionClassifierConfig};
use perpetual_observer::PerpetualPositionObserver;
use primitives::Chain;
use std::error::Error;
use std::sync::Arc;

use crate::model::WorkerService;
use crate::worker::context::WorkerContext;
use crate::worker::jobs::WorkerJob;

pub async fn jobs(ctx: WorkerContext, shutdown_rx: ShutdownReceiver) -> Result<Vec<JobHandle>, Box<dyn Error + Send + Sync>> {
    let services = ctx.services();
    let database = services.database();
    let config = services.config();

    let stream_producer = services.stream_producer("perpetuals_worker", shutdown_rx.clone()).await?;
    let cacher = services.cacher().await?;

    let providers = Arc::new(services.chain_providers(&settings::service_user_agent("daemon", Some("perpetual_observer"))));
    let classifier_config = PerpetualPositionClassifierConfig {
        trigger_bps: config.get_i64(ConfigKey::PerpetualPriorityTriggerBps).await?,
        liquidation_bps: config.get_i64(ConfigKey::PerpetualPriorityLiquidationBps).await?,
        concurrency: config.get_usize(ConfigKey::PerpetualClassifierConcurrency).await?,
    };
    let refresher = Arc::new(PerpetualAddressRefresher::new(providers.clone(), database.clone(), cacher.clone()));

    ctx.plan_builder(WorkerService::Perpetuals, &config, shutdown_rx)
        .jobs(WorkerJob::ClassifyPerpetualAddresses, Chain::perpetual_chains(), |chain, _| {
            let classifier = Arc::new(PerpetualPositionClassifier::new(chain, providers.clone(), cacher.clone(), classifier_config));
            move |_| {
                let classifier = classifier.clone();
                async move { classifier.classify().await }
            }
        })
        .jobs(WorkerJob::ObservePerpetualActiveAddresses, Chain::perpetual_chains(), |chain, _| {
            let observer = Arc::new(PerpetualPositionObserver::new(chain, providers.clone(), cacher.clone(), services.config(), stream_producer.clone()));
            move |_| {
                let observer = observer.clone();
                async move { observer.observe_active().await }
            }
        })
        .jobs(WorkerJob::ObservePerpetualPriorityAddresses, Chain::perpetual_chains(), |chain, _| {
            let observer = Arc::new(PerpetualPositionObserver::new(chain, providers.clone(), cacher.clone(), services.config(), stream_producer.clone()));
            move |_| {
                let observer = observer.clone();
                async move { observer.observe_priority().await }
            }
        })
        .jobs(WorkerJob::RefreshPerpetualTrackedAddresses, Chain::perpetual_chains(), |chain, _| {
            let refresher = refresher.clone();
            move |_| {
                let refresher = refresher.clone();
                async move { refresher.update(chain).await }
            }
        })
        .finish()
        .await
}
