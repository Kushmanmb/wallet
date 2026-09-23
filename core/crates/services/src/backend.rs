use std::error::Error;
use std::sync::Arc;

use cacher::CacherClient;
use chain_providers::ChainProviders;
use coingecko::CoinGeckoClient;
use config_keys::ConfigKey;
use defi::{DefiProviderClient, DefiProviderConfig};
use lists::CoinGeckoListProvider;
use primitives::Chain;
use pusher::PusherClient;
use search_index::{SearchIndexClient, SearchIndexConfig};
use settings::Settings;
use storage::{ConfigCacher, Database, DatabaseError};
use streamer::{Retry, ShutdownReceiver, StreamProducer, StreamProducerConfig};

use crate::assets::ListsClient;
use crate::defi::DefiClient;

#[derive(Clone)]
pub struct Services {
    settings: Arc<Settings>,
    database: Database,
}

impl Services {
    pub fn new(settings: Arc<Settings>) -> Result<Self, DatabaseError> {
        let database = Database::new(&settings.postgres.url, settings.postgres.pool)?;
        Ok(Self { settings, database })
    }

    pub fn settings(&self) -> Arc<Settings> {
        self.settings.clone()
    }

    pub fn database(&self) -> Database {
        self.database.clone()
    }

    pub fn config(&self) -> ConfigCacher {
        ConfigCacher::new(self.database())
    }

    pub async fn cacher(&self) -> Result<CacherClient, Box<dyn Error + Send + Sync>> {
        CacherClient::new(&self.settings.redis.url).await
    }

    pub async fn stream_producer(&self, name: &str, shutdown_rx: ShutdownReceiver) -> Result<StreamProducer, Box<dyn Error + Send + Sync>> {
        let rabbitmq = &self.settings.rabbitmq;
        let config = StreamProducerConfig::new(rabbitmq.url.clone(), Retry::new(rabbitmq.retry.delay, rabbitmq.retry.timeout));
        StreamProducer::new(&config, name, shutdown_rx).await
    }

    pub async fn search_index(&self) -> Result<SearchIndexClient, Box<dyn Error + Send + Sync>> {
        let config = SearchIndexConfig {
            batch_size: self.config().get_usize(ConfigKey::SearchIndexBatchSize).await?,
        };
        Ok(SearchIndexClient::new(&self.settings.meilisearch.url, &self.settings.meilisearch.key, config))
    }

    pub fn defi(&self) -> DefiClient {
        DefiClient::new(self.database(), DefiProviderClient::new(DefiProviderConfig::from_settings(&self.settings)))
    }

    pub fn lists(&self) -> ListsClient {
        let coingecko = CoinGeckoClient::new(self.settings.coingecko.remote_provider_config());
        ListsClient::new(self.database(), vec![Arc::new(CoinGeckoListProvider::new(coingecko))])
    }

    pub fn pusher(&self) -> PusherClient {
        PusherClient::new(self.settings.pusher.url.clone(), self.settings.pusher.ios.topic.clone())
    }

    pub fn chain_providers(&self, user_agent: &str) -> ChainProviders {
        ChainProviders::from_settings(&self.settings, user_agent)
    }

    pub fn chain_providers_for(&self, chain: Chain, user_agent: &str) -> ChainProviders {
        ChainProviders::for_chain(chain, &self.settings, user_agent)
    }
}
