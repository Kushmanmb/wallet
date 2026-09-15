use std::sync::Arc;

use async_trait::async_trait;
use primitives::{Currency, Platform, PlatformStore, Wallet};

use super::GemAppStartService;
use crate::api::{GemApiClient, GemDeviceApiClient};
use crate::gateway::EmptyPreferences;
use crate::services::asset_discovery::testkit::DiscoveryTestkit;
use crate::services::banner::GemBannerService;
use crate::services::banner::testkit::MemoryBannerStore;
use crate::services::config::GemConfigService;
use crate::services::device::platform::{GemDeviceInfo, GemDevicePlatform};
use crate::services::device::{GemDeviceKeyService, GemDeviceService};
use crate::services::error::GemServiceError;
use crate::services::preferences::GemPreferencesService;
use crate::services::preferences::testkit::MemoryPreferencesStore;
use crate::services::subscription::GemSubscriptionService;
use crate::services::wallet::testkit::{OTHER_PHRASE, PHRASE, WalletTestkit};
use crate::services::wallet_configuration::GemWalletConfigurationService;
use crate::testkit::TestAlienProvider;

pub struct MemoryDevicePlatform;

#[async_trait]
impl GemDevicePlatform for MemoryDevicePlatform {
    async fn device_id(&self) -> Result<String, GemServiceError> {
        Ok("device".to_string())
    }
    async fn device_info(&self) -> Result<GemDeviceInfo, GemServiceError> {
        Ok(GemDeviceInfo {
            platform: Platform::IOS,
            platform_store: PlatformStore::AppStore,
            os: "18".to_string(),
            model: "test".to_string(),
            version: "1.0".to_string(),
            locale_identifier: "en".to_string(),
        })
    }
    async fn push_token(&self) -> Result<String, GemServiceError> {
        Ok(String::new())
    }
    async fn is_push_enabled(&self) -> Result<bool, GemServiceError> {
        Ok(false)
    }
    async fn get_currency(&self) -> Result<Currency, GemServiceError> {
        Ok(Currency::USD)
    }
}

pub struct AppStartTestkit {
    pub service: GemAppStartService,
    pub wallets: WalletTestkit,
    pub first: Wallet,
    pub second: Wallet,
}

impl AppStartTestkit {
    pub async fn new() -> Self {
        let provider = Arc::new(TestAlienProvider::with_json_by_path(200, &[]));
        let wallets = WalletTestkit::new();
        let first = wallets.import("First", PHRASE).await;
        let second = wallets.import("Second", OTHER_PHRASE).await;
        let discovery = DiscoveryTestkit::with_provider(provider.clone(), first.clone());
        *discovery.wallets.wallets.lock().unwrap() = vec![first.clone(), second.clone()];
        let preferences = Arc::new(GemPreferencesService::new(Arc::new(MemoryPreferencesStore::default())));
        let device_key = Arc::new(GemDeviceKeyService::new(Arc::new(EmptyPreferences)));
        let device_api = Arc::new(GemDeviceApiClient::new(provider.clone(), device_key));
        let banner_store = Arc::new(MemoryBannerStore::default());
        let service = GemAppStartService::new(
            Arc::new(GemConfigService::new(Arc::new(GemApiClient::new(provider.clone())), preferences.clone())),
            Arc::new(GemBannerService::new(banner_store.clone())),
            discovery.assets.clone(),
            discovery.balance.clone(),
            Arc::new(GemWalletConfigurationService::new(device_api.clone(), banner_store, discovery.wallet_preferences.clone())),
            wallets.service.clone(),
            Arc::new(GemDeviceService::new(
                device_api.clone(),
                Arc::new(GemSubscriptionService::new(device_api, discovery.wallets.clone())),
                discovery.wallets.clone(),
                Arc::new(MemoryDevicePlatform),
                preferences,
            )),
        );
        Self { service, wallets, first, second }
    }
}
