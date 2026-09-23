pub mod fiat_webhook_consumer;

use std::error::Error;
use std::sync::Arc;

use cacher::AccessTokenCacherClient;
use primitives::FiatProviderName;
use services::Services;
use settings::Settings;
use streamer::{ConsumerStatusReporter, FiatWebhookPayload, QueueName, ShutdownReceiver, run_consumer};

use crate::consumers::{consumer_config, reader_for_queue};

use fiat_webhook_consumer::FiatWebhookConsumer;

pub async fn run_consumer_fiat(settings: Settings, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let services = Services::new(Arc::new(settings.clone()))?;
    let database = services.database();
    let queue = QueueName::FiatOrderWebhooks;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let stream_producer = services.stream_producer(&format!("{name}_producer"), shutdown_rx.clone()).await?;
    let cacher = services.cacher().await?;
    let access_token_cacher = Arc::new(AccessTokenCacherClient::new(cacher, FiatProviderName::Transak.id()));
    let consumer = FiatWebhookConsumer::new(database, settings.clone(), stream_producer, access_token_cacher);
    let consumer_config = consumer_config(&settings.consumer);
    run_consumer::<FiatWebhookPayload, FiatWebhookConsumer, bool>(&name, stream_reader, queue, None, consumer, consumer_config, shutdown_rx, reporter).await
}
