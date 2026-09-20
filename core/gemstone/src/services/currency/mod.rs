pub mod model;
pub(crate) mod rules;

use std::sync::Arc;

use primitives::Currency;

pub use model::{GemCurrencies, GemCurrencyRow};

use crate::services::device::GemDeviceService;
use crate::services::error::GemServiceError;
use crate::services::preferences::GemPreferencesService;
use crate::services::price::GemPriceService;

#[derive(uniffi::Object)]
pub struct GemCurrencyService {
    preferences: Arc<GemPreferencesService>,
    prices: Arc<GemPriceService>,
    device: Arc<GemDeviceService>,
}

#[uniffi::export]
impl GemCurrencyService {
    #[uniffi::constructor]
    pub fn new(preferences: Arc<GemPreferencesService>, prices: Arc<GemPriceService>, device: Arc<GemDeviceService>) -> Self {
        Self { preferences, prices, device }
    }

    pub fn get_currency(&self) -> Currency {
        self.preferences.get_currency()
    }

    pub fn currencies(&self, locale: Option<Currency>) -> GemCurrencies {
        rules::currencies(self.get_currency(), locale)
    }

    pub async fn set_currency(&self, currency: Currency) -> Result<(), GemServiceError> {
        if currency == self.get_currency() {
            return Ok(());
        }
        self.prices.change_currency(currency.clone()).await?;
        self.preferences.set_currency(currency)?;
        let _ = self.device.synchronize_if_needed().await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::GemDeviceApiClient;
    use crate::services::device::GemDeviceKeyService;
    use crate::services::device::testkit::MemoryDevicePlatform;
    use crate::services::preferences::testkit::MemoryPreferencesStore;
    use crate::services::price::testkit::MemoryPriceStore;
    use crate::services::subscription::GemSubscriptionService;
    use crate::services::wallet::testkit::MemoryWalletStore;
    use crate::testkit::{EmptyPreferences, TestAlienProvider};
    use futures::executor::block_on;

    fn service(prices: MemoryPriceStore) -> (GemCurrencyService, Arc<MemoryPriceStore>, Arc<TestAlienProvider>) {
        let prices = Arc::new(prices);
        let provider = Arc::new(TestAlienProvider::offline());
        let preferences = Arc::new(GemPreferencesService::new(Arc::new(MemoryPreferencesStore::default())));
        let wallets = Arc::new(MemoryWalletStore::default());
        let api = Arc::new(GemDeviceApiClient::new(provider.clone(), Arc::new(GemDeviceKeyService::new(Arc::new(EmptyPreferences)))));
        let device = Arc::new(GemDeviceService::new(
            api.clone(),
            Arc::new(GemSubscriptionService::new(api, wallets.clone())),
            wallets,
            Arc::new(MemoryDevicePlatform),
            preferences.clone(),
        ));
        (GemCurrencyService::new(preferences, Arc::new(GemPriceService::new(prices.clone())), device), prices, provider)
    }

    #[test]
    fn test_a_currency_without_a_rate_keeps_the_previous_currency() {
        let (service, prices, provider) = service(MemoryPriceStore::default());
        let previous = service.get_currency();

        assert!(block_on(service.set_currency(Currency::EUR)).is_err());
        assert_eq!(service.get_currency(), previous);
        assert!(prices.converted.lock().unwrap().is_empty());
        assert!(provider.requested_paths().is_empty(), "a currency that was never applied does not re-register the device");
    }

    #[test]
    fn test_a_currency_with_a_rate_converts_prices_and_is_saved() {
        let (service, prices, provider) = service(MemoryPriceStore::with_rate(Currency::EUR, 0.9));

        block_on(service.set_currency(Currency::EUR)).unwrap();

        assert_eq!(service.get_currency(), Currency::EUR);
        assert_eq!(*prices.converted.lock().unwrap(), vec![(Currency::EUR, 0.9)]);
        assert!(!provider.requested_paths().is_empty(), "the device carries the currency and re-registers even when the attempt fails");
    }
}
