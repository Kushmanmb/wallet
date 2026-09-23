use async_trait::async_trait;
use pricer::PriceClient;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use storage::models::PriceRow;
use storage::{AssetsRepository, Database, DatabaseError, PricesRepository};
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
        let prices: Vec<PriceRow> = payload.prices.into_iter().map(PriceRow::from_price_data).collect();
        let primary_price_max_age = self.config.primary_price_max_age;
        let (count, cache_entries) = self
            .database
            .run(move |client| -> Result<_, DatabaseError> {
                let asset_ids = client.set_prices(prices)?;
                if asset_ids.is_empty() {
                    return Ok((0, Vec::new()));
                }

                let primary_prices = client.get_primary_prices(&asset_ids, primary_price_max_age)?;
                if primary_prices.is_empty() {
                    return Ok((0, Vec::new()));
                }

                let count = primary_prices.len();
                let primary_asset_ids: Vec<_> = primary_prices.iter().map(|(id, _)| id.clone()).collect();
                let assets_by_id: HashMap<String, _> = client.get_assets_rows(primary_asset_ids)?.into_iter().map(|a| (a.id.clone(), a)).collect();
                let cache_entries = primary_prices
                    .into_iter()
                    .filter_map(|(asset_id, price)| assets_by_id.get(&asset_id.to_string()).map(|asset| price.as_price_asset_info(asset)))
                    .collect();
                Ok((count, cache_entries))
            })
            .await?;
        if count == 0 {
            return Ok(0);
        }
        self.price_client.set_cache_prices(cache_entries, self.config.ttl_seconds).await?;

        Ok(count)
    }
}
