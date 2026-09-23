pub mod fetch_address_transactions_consumer;
pub mod fetch_asset_associations_consumer;
pub mod fetch_asset_status_consumer;
pub mod fetch_assets_consumer;
pub mod fetch_blocks_consumer;
pub mod fetch_coin_addresses_consumer;
pub mod fetch_list_consumer;
pub mod fetch_nft_asset_consumer;
pub mod fetch_nft_assets_addresses_consumer;
pub mod fetch_prices_consumer;
pub mod fetch_prices_metadata_consumer;
pub mod fetch_token_addresses_consumer;
pub mod fetch_transaction_consumer;

use config_keys::ConfigKey;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use ::nft::{NFTClient, NFTProviderConfig};
use cacher::{AccessTokenCacherClient, CacherClient};
use coingecko::CoinGeckoClient;
use futures::future;
use gem_client::ReqwestClient;
use lists::{CoinGeckoListProvider, ListsClient};
use pricer::PriceClient;
use primitives::{AssetId, Chain, NFTChain, PriceId, PriceProvider, TransactionIdRequest};
use security::providers::goplus::GoPlusProvider;
use security::{ScanProviderConfig, ScanProviderFactory, TokenScanProviders};
use services::Services;
use settings::Settings;
use streamer::{
    ChainAddressPayload, ConsumerConfig, ConsumerStatusReporter, FetchAssetAssociationsPayload, FetchAssetsPayload, FetchBlocksPayload, FetchListPayload, FetchNFTAssetPayload, FetchPricesPayload, QueueName, ShutdownReceiver,
    StreamConnection, StreamProducer, StreamReader, run_consumer,
};

use crate::asset_spam::AssetClassificationRules;
use crate::consumers::runner::ChainConsumerRunner;
use crate::consumers::{consumer_config, consumer_user_agent, reader_config};
use crate::model::{IndexerConsumer, IndexerService};
use crate::worker::prices::price_providers;
use fetch_address_transactions_consumer::FetchAddressTransactionsConsumer;
use fetch_asset_associations_consumer::FetchAssetAssociationsConsumer;
use fetch_asset_status_consumer::FetchAssetStatusConsumer;
use fetch_assets_consumer::FetchAssetsConsumer;
use fetch_blocks_consumer::FetchBlocksConsumer;
use fetch_coin_addresses_consumer::FetchCoinAddressesConsumer;
use fetch_list_consumer::FetchListConsumer;
use fetch_nft_asset_consumer::FetchNftAssetConsumer;
use fetch_nft_assets_addresses_consumer::FetchNftAssetsAddressesConsumer;
use fetch_prices_consumer::FetchPricesConsumer;
use fetch_prices_metadata_consumer::FetchPricesMetadataConsumer;
use fetch_token_addresses_consumer::FetchTokenAddressesConsumer;
use fetch_transaction_consumer::FetchTransactionConsumer;

pub async fn run_consumer_indexer(settings: Settings, service: IndexerService, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>, only: Option<IndexerConsumer>) -> Result<(), Box<dyn Error + Send + Sync>> {
    use IndexerConsumer::*;

    let services = Services::new(Arc::new(settings))?;

    let selected = match only {
        Some(consumer) if service.consumers().contains(&consumer) => vec![consumer],
        Some(consumer) => return Err(format!("Indexer consumer {} does not belong to {}", consumer.as_ref(), service.as_ref()).into()),
        None => service.consumers().to_vec(),
    };

    let handles: Vec<_> = selected
        .into_iter()
        .map(|kind| {
            let services = services.clone();
            let shutdown_rx = shutdown_rx.clone();
            let reporter = reporter.clone();
            tokio::spawn(async move {
                match kind {
                    FetchBlocks => run_fetch_blocks(services, shutdown_rx, reporter).await,
                    FetchAssets => run_fetch_assets(services, shutdown_rx, reporter).await,
                    FetchAssetStatus => run_fetch_asset_status(services, shutdown_rx, reporter).await,
                    FetchAssetAssociations => run_fetch_asset_associations(services, shutdown_rx, reporter).await,
                    FetchLists => run_fetch_lists(services, shutdown_rx, reporter).await,
                    FetchPrices => run_fetch_prices(services, shutdown_rx, reporter).await,
                    FetchPricesMetadata => run_fetch_prices_metadata(services, shutdown_rx, reporter).await,
                    FetchTokenAssociations => run_fetch_token_associations(services, shutdown_rx, reporter).await,
                    FetchCoinAssociations => run_fetch_coin_associations(services, shutdown_rx, reporter).await,
                    FetchNftAssociations => run_fetch_nft_associations(services, shutdown_rx, reporter).await,
                    FetchNftAssets => run_fetch_nft_assets(services, shutdown_rx, reporter).await,
                    FetchAddressTransactions => run_fetch_transaction_associations(services, shutdown_rx, reporter).await,
                    FetchTransactions => run_fetch_transactions(services, shutdown_rx, reporter).await,
                }
            })
        })
        .collect();

    for handle in future::join_all(handles).await {
        handle??;
    }
    Ok(())
}

async fn run_fetch_asset_associations(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::FetchAssetAssociations;
    let name = queue.to_string();
    let connection = StreamConnection::new(&settings.rabbitmq.url, name.clone()).await?;
    let config = reader_config(&settings.rabbitmq, name.clone());
    let stream_reader = StreamReader::from_connection(&connection, config).await?;
    let consumer = FetchAssetAssociationsConsumer {
        database,
        providers: crate::worker::prices::price_providers(&settings, PriceProvider::all()),
    };
    run_consumer::<FetchAssetAssociationsPayload, _, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_fetch_blocks(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    ChainConsumerRunner::new(services, QueueName::FetchBlocks, shutdown_rx, reporter)
        .await?
        .run(|runner, chain| async move {
            let queue = QueueName::FetchBlocks;
            let name = format!("{}.{}", queue, chain.as_ref());
            let stream_reader = runner.stream_reader().await?;
            let stream_producer = runner.stream_producer().await?;
            let consumer = FetchBlocksConsumer::new(runner.services.chain_providers_for(chain, &consumer_user_agent(&name)), stream_producer);
            run_consumer::<FetchBlocksPayload, FetchBlocksConsumer, usize>(&name, stream_reader, queue, Some(chain.as_ref()), consumer, runner.config, runner.shutdown_rx, runner.reporter).await
        })
        .await
}

async fn run_fetch_assets(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::FetchAssets;
    let name = queue.to_string();
    let connection = StreamConnection::new(&settings.rabbitmq.url, name.clone()).await?;
    let config = reader_config(&settings.rabbitmq, name.clone());
    let stream_reader = StreamReader::from_connection(&connection, config).await?;
    let stream_producer = StreamProducer::from_connection(&connection, shutdown_rx.clone()).await?;
    let cacher = services.cacher().await?;
    let classification_rules = AssetClassificationRules::from_config(&services.config()).await?;
    let consumer = FetchAssetsConsumer {
        providers: services.chain_providers(&consumer_user_agent(&name)),
        database,
        cacher,
        classification_rules,
        stream_producer,
    };
    run_consumer::<FetchAssetsPayload, FetchAssetsConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_fetch_asset_status(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::FetchAssetStatus;
    let name = queue.to_string();
    let connection = StreamConnection::new(&settings.rabbitmq.url, name.clone()).await?;
    let config = reader_config(&settings.rabbitmq, name.clone());
    let stream_reader = StreamReader::from_connection(&connection, config).await?;
    let cacher = services.cacher().await?;
    let providers = scan_providers(&settings, cacher, services.config().get_duration(ConfigKey::ScanTimeout).await?)?;
    let consumer = FetchAssetStatusConsumer { database, providers };
    run_consumer::<AssetId, _, bool>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

fn scan_providers(settings: &Settings, cacher: CacherClient, timeout: Duration) -> Result<TokenScanProviders, Box<dyn Error + Send + Sync>> {
    let config = ScanProviderConfig::new(&settings.security, timeout);
    ScanProviderFactory::new_token_providers(&config, Arc::new(AccessTokenCacherClient::new(cacher, GoPlusProvider::<ReqwestClient>::NAME)))
}

async fn run_fetch_lists(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::FetchLists;
    let name = queue.to_string();
    let connection = StreamConnection::new(&settings.rabbitmq.url, name.clone()).await?;
    let config = reader_config(&settings.rabbitmq, name.clone());
    let stream_reader = StreamReader::from_connection(&connection, config).await?;
    let coin_gecko_client = CoinGeckoClient::new(settings.coingecko.remote_provider_config());
    let lists_client = ListsClient::new(database.clone(), vec![Arc::new(CoinGeckoListProvider::new(database, coin_gecko_client))]);
    let consumer = FetchListConsumer { lists_client };
    run_consumer::<FetchListPayload, FetchListConsumer, u32>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_fetch_prices_metadata(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::FetchPricesMetadata;
    let name = queue.to_string();
    let connection = StreamConnection::new(&settings.rabbitmq.url, name.clone()).await?;
    let config = reader_config(&settings.rabbitmq, name.clone());
    let stream_reader = StreamReader::from_connection(&connection, config).await?;
    let consumer = FetchPricesMetadataConsumer {
        config: services.config(),
        database,
        cacher: services.cacher().await?,
        providers: price_providers(&settings, PriceProvider::all()),
    };
    let config = ConsumerConfig {
        skip_on_error: true,
        ..consumer_config(&settings.consumer)
    };
    run_consumer::<PriceId, _, usize>(&name, stream_reader, queue, None, consumer, config, shutdown_rx, reporter).await
}

async fn run_fetch_prices(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::FetchPrices;
    let name = queue.to_string();
    let connection = StreamConnection::new(&settings.rabbitmq.url, name.clone()).await?;
    let config = reader_config(&settings.rabbitmq, name.clone());
    let stream_reader = StreamReader::from_connection(&connection, config).await?;
    let cacher = services.cacher().await?;
    let price_client = PriceClient::new(database, cacher);
    let providers = crate::worker::prices::price_providers(&settings, PriceProvider::all());
    let consumer = FetchPricesConsumer { price_client, providers };
    run_consumer::<FetchPricesPayload, FetchPricesConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_fetch_token_associations(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    ChainConsumerRunner::new(services, QueueName::FetchTokenAssociations, shutdown_rx, reporter)
        .await?
        .run(|runner, chain| async move {
            let queue = QueueName::FetchTokenAssociations;
            let name = format!("{}.{}", queue, chain.as_ref());
            let stream_reader = runner.stream_reader().await?;
            let stream_producer = runner.stream_producer().await?;
            let consumer = FetchTokenAddressesConsumer::new(runner.services.chain_providers_for(chain, &consumer_user_agent(&name)), runner.database, stream_producer, runner.cacher);
            run_consumer::<ChainAddressPayload, FetchTokenAddressesConsumer, usize>(&name, stream_reader, queue, Some(chain.as_ref()), consumer, runner.config, runner.shutdown_rx, runner.reporter).await
        })
        .await
}

async fn run_fetch_coin_associations(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    ChainConsumerRunner::new(services, QueueName::FetchCoinAssociations, shutdown_rx, reporter)
        .await?
        .run(|runner, chain| async move {
            let queue = QueueName::FetchCoinAssociations;
            let name = format!("{}.{}", queue, chain.as_ref());
            let stream_reader = runner.stream_reader().await?;
            let consumer = FetchCoinAddressesConsumer::new(runner.services.chain_providers_for(chain, &consumer_user_agent(&name)), runner.database, runner.cacher);
            run_consumer::<ChainAddressPayload, FetchCoinAddressesConsumer, String>(&name, stream_reader, queue, Some(chain.as_ref()), consumer, runner.config, runner.shutdown_rx, runner.reporter).await
        })
        .await
}

async fn run_fetch_nft_associations(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chains: Vec<Chain> = NFTChain::all().into_iter().map(Into::into).collect();
    ChainConsumerRunner::new(services, QueueName::FetchNftAssociations, shutdown_rx, reporter)
        .await?
        .run_for_chains(chains, |runner, chain| async move {
            FetchNftAssetsAddressesConsumer::run(runner.settings, runner.database, chain, &runner.connection, runner.cacher, runner.config, runner.shutdown_rx, runner.reporter).await
        })
        .await
}

async fn run_fetch_nft_assets(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = services.settings();
    let database = services.database();
    let queue = QueueName::FetchNFTCollectionAssets;
    let name = queue.to_string();
    let connection = StreamConnection::new(&settings.rabbitmq.url, name.clone()).await?;
    let config = reader_config(&settings.rabbitmq, name.clone());
    let stream_reader = StreamReader::from_connection(&connection, config).await?;
    let cacher = services.cacher().await?;
    let nft_config = NFTProviderConfig::from_settings(&settings);
    let nft_client = NFTClient::from_config(database, nft_config, settings.nft.url.clone());
    let consumer = FetchNftAssetConsumer { nft_client, cacher };
    run_consumer::<FetchNFTAssetPayload, FetchNftAssetConsumer, usize>(&name, stream_reader, queue, None, consumer, consumer_config(&settings.consumer), shutdown_rx, reporter).await
}

async fn run_fetch_transaction_associations(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    ChainConsumerRunner::new(services, QueueName::FetchAddressTransactions, shutdown_rx, reporter)
        .await?
        .run(|runner, chain| async move {
            let queue = QueueName::FetchAddressTransactions;
            let name = format!("{}.{}", queue, chain.as_ref());
            let stream_reader = runner.stream_reader().await?;
            let stream_producer = runner.stream_producer().await?;
            let consumer = FetchAddressTransactionsConsumer::new(runner.services.chain_providers_for(chain, &consumer_user_agent(&name)), stream_producer, runner.cacher, runner.services.config());
            run_consumer::<ChainAddressPayload, FetchAddressTransactionsConsumer, usize>(&name, stream_reader, queue, Some(chain.as_ref()), consumer, runner.config, runner.shutdown_rx, runner.reporter).await
        })
        .await
}

async fn run_fetch_transactions(services: Services, shutdown_rx: ShutdownReceiver, reporter: Arc<dyn ConsumerStatusReporter>) -> Result<(), Box<dyn Error + Send + Sync>> {
    ChainConsumerRunner::new(services, QueueName::FetchTransactions, shutdown_rx, reporter)
        .await?
        .run(|runner, chain| async move {
            let queue = QueueName::FetchTransactions;
            let name = format!("{}.{}", queue, chain.as_ref());
            let stream_reader = runner.stream_reader().await?;
            let stream_producer = runner.stream_producer().await?;
            let consumer = FetchTransactionConsumer::new(runner.services.chain_providers_for(chain, &consumer_user_agent(&name)), stream_producer, runner.cacher);
            run_consumer::<TransactionIdRequest, FetchTransactionConsumer, usize>(&name, stream_reader, queue, Some(chain.as_ref()), consumer, runner.config, runner.shutdown_rx, runner.reporter).await
        })
        .await
}
