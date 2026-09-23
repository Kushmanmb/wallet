use primitives::{AssetId, ChartPeriod, ChartValue, PriceConfig, currency::Currency};
use std::error::Error;
use storage::{ChartsRepository, Database, FiatRepository, PricesRepository};

#[derive(Clone)]
pub struct ChartClient {
    database: Database,
    config: PriceConfig,
}

impl ChartClient {
    pub fn new(database: Database, config: PriceConfig) -> Self {
        Self { database, config }
    }

    pub async fn get_charts_prices(&self, asset_id: &AssetId, period: ChartPeriod, currency: &Currency) -> Result<Vec<ChartValue>, Box<dyn Error + Send + Sync>> {
        let asset_id = asset_id.clone();
        let currency = currency.clone();
        let primary_price_max_age = self.config.primary_price_max_age;
        let (rate_multiplier, charts) = self
            .database
            .run(move |client| -> Result<_, Box<dyn Error + Send + Sync>> {
                let base_rate = client.get_fiat_rate(&Currency::USD)?;
                let rate = client.get_fiat_rate(&currency)?;
                let key = client.get_primary_price_key(&asset_id, primary_price_max_age)?;
                Ok((rate.multiplier(base_rate.rate), client.get_charts(&key.id(), &period)?))
            })
            .await?;
        Ok(charts
            .into_iter()
            .map(|(ts, price)| ChartValue {
                timestamp: ts.and_utc().timestamp() as i32,
                value: (price * rate_multiplier) as f32,
            })
            .collect())
    }
}
