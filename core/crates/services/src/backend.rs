use std::error::Error;
use std::sync::Arc;

use cacher::{AccessTokenCacherClient, CacherClient};
use chain_providers::ChainProviders;
use coingecko::CoinGeckoClient;
use config_keys::ConfigKey;
use defi::{DefiProviderClient, DefiProviderConfig};
use fiat::{FiatProvider, FiatProviderFactory};
use lists::CoinGeckoListProvider;
use nft::NFTProviderConfig;
use primitives::{AccessTokenCacher, Chain, FiatProviderName, PriceConfig};
use pusher::PusherClient;
use search_index::{SearchIndexClient, SearchIndexConfig};
use settings::Settings;
use storage::{ConfigCacher, Database, DatabaseError};
use streamer::{Retry, ShutdownReceiver, StreamProducer, StreamProducerConfig};

use crate::assets::ListsClient;
use crate::auth::AuthClient;
use crate::defi::DefiClient;
use crate::fiat::FiatClient;
use crate::nft::NFTClient;
use crate::prices::{ChartClient, MarketsClient, PriceAlertClient, PriceClient};
use crate::support::SupportClient;

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

    pub async fn auth(&self) -> Result<AuthClient, Box<dyn Error + Send + Sync>> {
        Ok(AuthClient::new(self.cacher().await?))
    }

    pub async fn stream_producer(&self, name: &str, shutdown_rx: ShutdownReceiver) -> Result<StreamProducer, Box<dyn Error + Send + Sync>> {
        let rabbitmq = &self.settings.rabbitmq;
        let config = StreamProducerConfig::new(rabbitmq.url.clone(), Retry::new(rabbitmq.retry.delay, rabbitmq.retry.timeout));
        StreamProducer::new(&config, name, shutdown_rx).await
    }

    pub async fn support(&self, shutdown_rx: ShutdownReceiver) -> Result<SupportClient, Box<dyn Error + Send + Sync>> {
        let stream_producer = self.stream_producer("daemon_support_producer", shutdown_rx).await?;
        Ok(SupportClient::new(self.database(), stream_producer, self.cacher().await?))
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

    pub async fn fiat(&self, stream_producer: StreamProducer) -> Result<FiatClient, Box<dyn Error + Send + Sync>> {
        let cacher = self.cacher().await?;
        let providers = self.fiat_providers(fiat_access_token_cacher(cacher.clone()));
        Ok(FiatClient::new(self.database(), cacher, providers, FiatProviderFactory::new_ip_check_client(&self.settings), stream_producer))
    }

    pub async fn fiat_access_token_cacher(&self) -> Result<Arc<dyn AccessTokenCacher>, Box<dyn Error + Send + Sync>> {
        Ok(fiat_access_token_cacher(self.cacher().await?))
    }

    pub fn fiat_providers(&self, access_token_cacher: Arc<dyn AccessTokenCacher>) -> Vec<Box<dyn FiatProvider + Send + Sync>> {
        FiatProviderFactory::new_providers(&self.settings, access_token_cacher)
    }

    pub fn lists(&self) -> ListsClient {
        let coingecko = CoinGeckoClient::new(self.settings.coingecko.remote_provider_config());
        ListsClient::new(self.database(), vec![Arc::new(CoinGeckoListProvider::new(coingecko))])
    }

    pub fn nft(&self) -> NFTClient {
        NFTClient::from_config(self.database(), NFTProviderConfig::from_settings(&self.settings), self.settings.nft.url.clone())
    }

    pub fn prices(&self, cacher: CacherClient) -> PriceClient {
        PriceClient::new(self.database(), cacher)
    }

    pub fn charts(&self, config: PriceConfig) -> ChartClient {
        ChartClient::new(self.database(), config)
    }

    pub fn markets(&self, cacher: CacherClient) -> MarketsClient {
        MarketsClient::new(self.database(), cacher)
    }

    pub fn price_alerts(&self) -> PriceAlertClient {
        PriceAlertClient::new(self.database())
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

fn fiat_access_token_cacher(cacher: CacherClient) -> Arc<dyn AccessTokenCacher> {
    Arc::new(AccessTokenCacherClient::new(cacher, FiatProviderName::Transak.id()))
}
