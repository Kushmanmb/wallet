pub mod rewards_consumer;
pub mod rewards_redemption_consumer;

use std::error::Error;
use std::sync::Arc;

use config_keys::ConfigKey;
use primitives::rewards::RedemptionStatus;
use rewards::TransferRedemptionService;
use services::Services;
use settings::Settings;
use streamer::{ConsumerStatusReporter, QueueName, RewardsNotificationPayload, RewardsRedemptionPayload, ShutdownReceiver, run_consumer};

use crate::consumers::{consumer_config, reader_for_queue};

pub async fn run_consumer_rewards(settings: Settings, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let services = Services::new(Arc::new(settings))?;

    futures::future::try_join_all(vec![
        tokio::spawn(run_rewards_events(services.clone(), shutdown_rx.clone(), reporter.clone())),
        tokio::spawn(run_rewards_redemptions(services.clone(), shutdown_rx.clone(), reporter.clone())),
    ])
    .await?;

    Ok(())
}

async fn run_rewards_events(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::RewardsEvents;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let stream_producer = services.stream_producer(&name, shutdown_rx.clone()).await?;
    let consumer = rewards_consumer::RewardsConsumer::new(database, stream_producer);
    let consumer_config = consumer_config(&settings.consumer);
    run_consumer::<RewardsNotificationPayload, rewards_consumer::RewardsConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config, shutdown_rx, reporter).await
}

async fn run_rewards_redemptions(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let config = services.config();
    let retry_config = rewards_redemption_consumer::RedemptionRetryConfig {
        max_retries: config.get_i64(ConfigKey::RedemptionRetryMaxRetries).await? as u32,
        delay: config.get_duration(ConfigKey::RedemptionRetryDelay).await?,
        errors: config.get_vec_string(ConfigKey::RedemptionRetryErrors).await?,
    };
    let queue = QueueName::RewardsRedemptions;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let stream_producer = services.stream_producer(&name, shutdown_rx.clone()).await?;
    let redemption_service = Arc::new(services.redemption_service()?);
    let consumer = rewards_redemption_consumer::RewardsRedemptionConsumer::new(database, redemption_service, retry_config, stream_producer);
    let consumer_config = consumer_config(&settings.consumer);
    run_consumer::<RewardsRedemptionPayload, rewards_redemption_consumer::RewardsRedemptionConsumer<TransferRedemptionService>, RedemptionStatus>(&name, stream_reader, queue, None, consumer, consumer_config, shutdown_rx, reporter).await
}
