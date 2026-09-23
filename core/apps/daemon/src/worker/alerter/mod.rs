mod price_alerts_sender;
mod staking_rewards_notifier;

use std::error::Error;
use std::sync::Arc;

use config_keys::ConfigKey;
use job_runner::{JobHandle, ShutdownReceiver};
use price_alerts_sender::PriceAlertSender;
use primitives::Chain;
use settings::service_user_agent;
use staking_rewards_notifier::{StakeRewardsConfig, StakingRewardsNotifier};

use crate::model::WorkerService;
use crate::worker::context::WorkerContext;
use crate::worker::jobs::WorkerJob;

pub async fn jobs(ctx: WorkerContext, shutdown_rx: ShutdownReceiver) -> Result<Vec<JobHandle>, Box<dyn Error + Send + Sync>> {
    let services = ctx.services();
    let database = services.database();
    let config = services.config();
    let cacher = services.cacher().await?;
    let stream_producer = services.stream_producer("send_price_alerts", shutdown_rx.clone()).await?;
    let stake_rewards_config = StakeRewardsConfig {
        threshold: config.get_f64(ConfigKey::AlerterStakeRewardsThreshold).await?,
        lookback: config.get_duration(ConfigKey::AlerterStakeRewardsLookback).await?,
    };
    let chain_providers = Arc::new(services.chain_providers(&service_user_agent("daemon", Some("stake_rewards"))));

    ctx.plan_builder(WorkerService::Alerter, &config, shutdown_rx)
        .job(WorkerJob::AlertPriceAlerts, {
            let config = config.clone();
            let price_alert_client = services.price_alerts();
            let stream_producer = stream_producer.clone();
            move |_| {
                let config = config.clone();
                let price_alert_client = price_alert_client.clone();
                let stream_producer = stream_producer.clone();
                async move { PriceAlertSender::new(config, price_alert_client, stream_producer).run_observer().await }
            }
        })
        .jobs(WorkerJob::AlertStakeRewards, Chain::stakeable(), |chain, _| {
            let chain_providers = chain_providers.clone();
            let database = database.clone();
            let cacher = cacher.clone();
            let stream_producer = stream_producer.clone();
            move |_| {
                let chain_providers = chain_providers.clone();
                let database = database.clone();
                let cacher = cacher.clone();
                let stream_producer = stream_producer.clone();
                async move {
                    let notifier = StakingRewardsNotifier::new(chain_providers, database, stake_rewards_config, cacher, stream_producer);
                    notifier.check_chain(chain).await
                }
            }
        })
        .finish()
        .await
}
