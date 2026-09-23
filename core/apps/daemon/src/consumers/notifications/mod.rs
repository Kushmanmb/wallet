mod in_app_notifications_consumer;
mod notifications_consumer;
mod notifications_failed_consumer;

pub use in_app_notifications_consumer::InAppNotificationsConsumer;
pub use notifications_consumer::NotificationsConsumer;
pub use notifications_failed_consumer::NotificationsFailedConsumer;

use services::Services;
use settings::Settings;
use std::error::Error;
use std::sync::Arc;
use streamer::{ConsumerStatusReporter, InAppNotificationPayload, NotificationsFailedPayload, NotificationsPayload, QueueName, ShutdownReceiver, StreamReader, run_consumer};

use crate::consumers::{consumer_config, reader_config};

pub async fn run(settings: Settings, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let services = Services::new(Arc::new(settings))?;

    futures::future::try_join_all(vec![
        tokio::spawn(run_notification_consumer(services.clone(), QueueName::NotificationsPriceAlerts, shutdown_rx.clone(), reporter.clone())),
        tokio::spawn(run_notification_consumer(services.clone(), QueueName::NotificationsTransactions, shutdown_rx.clone(), reporter.clone())),
        tokio::spawn(run_notification_consumer(services.clone(), QueueName::NotificationsObservers, shutdown_rx.clone(), reporter.clone())),
        tokio::spawn(run_notification_consumer(services.clone(), QueueName::NotificationsSupport, shutdown_rx.clone(), reporter.clone())),
        tokio::spawn(run_notification_consumer(services.clone(), QueueName::NotificationsRewards, shutdown_rx.clone(), reporter.clone())),
        tokio::spawn(run_notification_consumer(services.clone(), QueueName::NotificationsFiatPurchase, shutdown_rx.clone(), reporter.clone())),
        tokio::spawn(run_notifications_failed_consumer(services.clone(), QueueName::NotificationsFailed, shutdown_rx.clone(), reporter.clone())),
        tokio::spawn(run_in_app_notifications_consumer(services.clone(), shutdown_rx.clone(), reporter.clone())),
    ])
    .await?;

    Ok(())
}

async fn run_notification_consumer(services: Services, queue: QueueName, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let name = queue.to_string();
    let stream_reader = StreamReader::new(reader_config(&settings.rabbitmq, name.clone()), &shutdown_rx).await?.ok_or("shutdown during connect")?;
    let pusher_client = services.pusher();
    let stream_producer = services.stream_producer(&name, shutdown_rx.clone()).await?;
    let consumer = NotificationsConsumer::new(pusher_client, stream_producer);

    run_consumer::<NotificationsPayload, NotificationsConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_notifications_failed_consumer(services: Services, queue: QueueName, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let name = queue.to_string();
    let stream_reader = StreamReader::new(reader_config(&settings.rabbitmq, name.clone()), &shutdown_rx).await?.ok_or("shutdown during connect")?;
    let consumer = NotificationsFailedConsumer::new(database);

    let consumer_config = consumer_config(&settings.consumer);
    run_consumer::<NotificationsFailedPayload, NotificationsFailedConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config, shutdown_rx, reporter).await
}

async fn run_in_app_notifications_consumer(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::NotificationsInApp;
    let name = queue.to_string();
    let stream_reader = StreamReader::new(reader_config(&settings.rabbitmq, name.clone()), &shutdown_rx).await?.ok_or("shutdown during connect")?;
    let stream_producer = services.stream_producer(&name, shutdown_rx.clone()).await?;
    let consumer = InAppNotificationsConsumer::new(database, stream_producer);

    let consumer_config = consumer_config(&settings.consumer);
    run_consumer::<InAppNotificationPayload, InAppNotificationsConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config, shutdown_rx, reporter).await
}
