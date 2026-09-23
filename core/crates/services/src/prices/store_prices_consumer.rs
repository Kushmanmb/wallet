use crate::prices::PriceClient;
use async_trait::async_trait;
use std::error::Error;
use std::time::Duration;
use storage::{Database, DatabaseError, PricesRepository};
use streamer::{PricesPayload, consumer::MessageConsumer};

#[derive(Clone, Copy)]
pub struct StorePricesConsumerConfig {
    pub ttl_seconds: i64,
    pub primary_price_max_age: Duration,
}

pub struct StorePricesConsumer {
    pub database: Database,
    pub price_client: PriceClient,
    pub config: StorePricesConsumerConfig,
}

impl StorePricesConsumer {
    pub fn new(database: Database, price_client: PriceClient, config: StorePricesConsumerConfig) -> Self {
        Self { database, price_client, config }
    }
}

#[async_trait]
impl MessageConsumer<PricesPayload, usize> for StorePricesConsumer {
    async fn should_process(&self, _payload: &PricesPayload) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(true)
    }

    async fn process(&self, payload: PricesPayload) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let prices = payload.prices;
        let primary_price_max_age = self.config.primary_price_max_age;
        let (count, cache_entries) = self
            .database
            .run(move |client| -> Result<_, DatabaseError> {
                let asset_ids = client.set_prices(prices)?;
                if asset_ids.is_empty() {
                    return Ok((0, Vec::new()));
                }

                let cache_entries = client.get_primary_price_infos(&asset_ids, primary_price_max_age)?;
                Ok((cache_entries.len(), cache_entries))
            })
            .await?;
        if count == 0 {
            return Ok(0);
        }
        self.price_client.set_cache_prices(cache_entries, self.config.ttl_seconds).await?;

        Ok(count)
    }
}
