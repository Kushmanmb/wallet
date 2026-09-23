pub mod store_pending_transactions_consumer;
pub mod store_prices_consumer;
pub mod store_transactions_consumer;
pub mod store_transactions_consumer_config;
pub mod wallet_stream_consumer;

pub use store_transactions_consumer::StoreTransactionsConsumer;
pub use store_transactions_consumer_config::StoreTransactionsConsumerConfig;

use std::error::Error;
use std::sync::Arc;

use crate::client::SwapVaultAddressClient;
use config_keys::ConfigKey;
use primitives::TransactionId;
use services::Services;
use settings::Settings;
use streamer::{ConsumerStatusReporter, PricesPayload, QueueName, ShutdownReceiver, TransactionsPayload, WalletStreamPayload, run_consumer};

use crate::consumers::{consumer_config, reader_for_queue};
use crate::pusher::Pusher;

use store_pending_transactions_consumer::StorePendingTransactionsConsumer;
use store_prices_consumer::StorePricesConsumer;
use wallet_stream_consumer::WalletStreamConsumer;

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
    let database = services.database();
    let queue = QueueName::StoreTransactions;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let stream_producer = services.stream_producer(&name, shutdown_rx.clone()).await?;
    let cacher = services.cacher().await?;
    let config_cacher = services.config();
    let config = StoreTransactionsConsumerConfig {
        swap_outdated_timeout: config_cacher.get_duration(ConfigKey::TransactionSwapOutdatedTimeout).await?,
        outdated_block_count: config_cacher.get_i64(ConfigKey::TransactionsOutdatedBlockCount).await? as u64,
        outdated_min_timeout: config_cacher.get_duration(ConfigKey::TransactionsOutdatedMinTimeout).await?,
        max_asset_transfer_count: config_cacher.get_usize(ConfigKey::TransactionsMaxAssetTransferCount).await?,
        min_amount_usd: config_cacher.get_f64(ConfigKey::TransactionsMinAmountUsd).await?,
        primary_price_max_age: config_cacher.get_duration(ConfigKey::PricePrimaryMaxAge).await?,
    };
    let consumer = StoreTransactionsConsumer {
        database: database.clone(),
        stream_producer,
        pusher: Pusher::new(database),
        config,
        vault_client: SwapVaultAddressClient::new(cacher),
    };
    run_consumer::<TransactionsPayload, StoreTransactionsConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_store_prices(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::StorePrices;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let cacher_client = services.cacher().await?;
    let price_client = services.prices(cacher_client);
    let config = services.config();
    let ttl_seconds = config.get_duration(ConfigKey::PriceOutdated).await?.as_secs() as i64;
    let consumer = StorePricesConsumer::new(
        database,
        price_client,
        store_prices_consumer::StorePricesConsumerConfig {
            ttl_seconds,
            primary_price_max_age: config.get_duration(ConfigKey::PricePrimaryMaxAge).await?,
        },
    );
    run_consumer::<PricesPayload, StorePricesConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_wallet_stream(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::WalletStreamEvents;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let cacher_client = services.cacher().await?;
    let retention = services.config().get_duration(ConfigKey::DeviceStreamRetention).await?;
    let consumer = WalletStreamConsumer { database, cacher_client, retention };
    run_consumer::<WalletStreamPayload, WalletStreamConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_store_pending_transactions(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let queue = QueueName::StorePendingTransactions;
    let (name, stream_reader) = reader_for_queue(&settings, &queue, &shutdown_rx).await?;
    let cacher = services.cacher().await?;
    let consumer = StorePendingTransactionsConsumer::new(cacher);
    run_consumer::<TransactionId, StorePendingTransactionsConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}
