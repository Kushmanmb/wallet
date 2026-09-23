use std::error::Error;
use std::sync::Arc;

use primitives::TransactionId;
use services::Services;
use settings::Settings;
use streamer::{ConsumerStatusReporter, PricesPayload, QueueName, ShutdownReceiver, TransactionsPayload, WalletStreamPayload, run_consumer};

use crate::consumers::{consumer_config, reader_for_queue};

pub async fn run_consumer_store(settings: Settings, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let services = Services::new(Arc::new(settings))?;

    tokio::try_join!(
        run_store_transactions(services.clone(), shutdown_rx.clone(), reporter.clone()),
        run_store_prices(services.clone(), shutdown_rx.clone(), reporter.clone()),
        run_store_pending_transactions(services.clone(), shutdown_rx.clone(), reporter.clone()),
        run_wallet_stream(services.clone(), shutdown_rx.clone(), reporter.clone()),
    )?;

    Ok(())
}

async fn run_store_transactions(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let queue = QueueName::StoreTransactions;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let consumer = services.store_transactions_consumer(&name, shutdown_rx.clone()).await?;
    run_consumer::<TransactionsPayload, _, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_store_prices(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let queue = QueueName::StorePrices;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let consumer = services.store_prices_consumer().await?;
    run_consumer::<PricesPayload, _, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_wallet_stream(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let queue = QueueName::WalletStreamEvents;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let consumer = services.wallet_stream_consumer().await?;
    run_consumer::<WalletStreamPayload, _, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_store_pending_transactions(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let queue = QueueName::StorePendingTransactions;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let consumer = services.store_pending_transactions_consumer().await?;
    run_consumer::<TransactionId, _, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}
