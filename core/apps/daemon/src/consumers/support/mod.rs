pub mod support_webhook_consumer;

use std::error::Error;
use std::sync::Arc;

use services::Services;
use settings::Settings;
use streamer::{ConsumerStatusReporter, QueueName, ShutdownReceiver, SupportWebhookPayload, run_consumer};
use support::SupportClient;

use crate::consumers::{consumer_config, reader_for_queue};

use support_webhook_consumer::SupportWebhookConsumer;

pub async fn run_consumer_support(settings: Settings, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let services = Services::new(Arc::new(settings.clone()))?;
    let database = services.database();

    let stream_producer = services.stream_producer("daemon_support_producer", shutdown_rx.clone()).await?;
    let cacher = services.cacher().await?;

    let support_client = SupportClient::new(database, stream_producer, cacher);
    let consumer = SupportWebhookConsumer::new(support_client);

    let queue = QueueName::SupportWebhooks;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let consumer_config = consumer_config(&settings.consumer);
    run_consumer::<SupportWebhookPayload, SupportWebhookConsumer, bool>(&name, stream_reader, queue, None, consumer, consumer_config, shutdown_rx, reporter).await
}
