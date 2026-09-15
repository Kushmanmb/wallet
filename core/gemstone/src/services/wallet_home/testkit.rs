use std::sync::{Arc, Mutex};

use primitives::{Transaction, Wallet, WalletId};

use super::GemWalletHomeService;
use crate::api::{GemApiClient, GemDeviceApiClient};
use crate::gateway::{EmptyPreferences, GemGateway};
use crate::services::asset_discovery::GemAssetDiscoveryService;
use crate::services::assets::GemAssetsService;
use crate::services::assets::testkit::MemoryAssetStore;
use crate::services::balance::GemBalanceService;
use crate::services::balance::testkit::RecordingBalanceStore;
use crate::services::banner::GemBannerService;
use crate::services::banner::testkit::MemoryBannerStore;
use crate::services::device::GemDeviceKeyService;
use crate::services::nft::GemNftService;
use crate::services::nft::testkit::MemoryNftStore;
use crate::services::preferences::GemPreferencesService;
use crate::services::preferences::testkit::MemoryPreferencesStore;
use crate::services::price::GemPriceService;
use crate::services::price::testkit::MemoryPriceStore;
use crate::services::stream::testkit::SubscriptionTestkit;
use crate::services::transaction_state::GemTransactionStatusService;
use crate::services::transactions::GemTransactionsService;
use crate::services::transactions::testkit::MemoryTransactionStore;
use crate::services::wallet::testkit::{MemoryAddressStore, MemoryWalletStore};
use crate::services::wallet_preferences::GemWalletPreferencesService;
use crate::services::wallet_preferences::testkit::MemoryWalletPreferencesStore;
use crate::services::wallet_session::GemWalletSessionService;
use crate::services::wallet_session::testkit::MemoryWalletSessionStore;
use crate::testkit::TestAlienProvider;

#[derive(Default)]
pub struct RecordingTransactionStatus {
    pub tracked: Mutex<Vec<Vec<Transaction>>>,
}

impl GemTransactionStatusService for RecordingTransactionStatus {
    fn track(&self, _: WalletId, transactions: Vec<Transaction>) {
        self.tracked.lock().unwrap().push(transactions);
    }
}

pub struct WalletHomeTestkit {
    pub service: GemWalletHomeService,
    pub provider: Arc<TestAlienProvider>,
    pub balances: Arc<RecordingBalanceStore>,
    pub wallet_preferences: Arc<GemWalletPreferencesService>,
    pub wallet_id: WalletId,
}

impl WalletHomeTestkit {
    pub fn with_status(status: u16) -> Self {
        let wallet = Wallet::mock();
        let provider = Arc::new(TestAlienProvider::with_status(status));
        let preferences_store = Arc::new(MemoryPreferencesStore::default());
        let preferences = Arc::new(GemPreferencesService::new(preferences_store.clone()));
        let wallets = Arc::new(MemoryWalletStore {
            wallets: Mutex::new(vec![wallet.clone()]),
            ..Default::default()
        });
        let session = Arc::new(GemWalletSessionService::new(
            Arc::new(MemoryWalletSessionStore {
                current: Mutex::new(Some(wallet.id.clone())),
            }),
            wallets.clone(),
        ));
        let gateway = Arc::new(GemGateway::new(provider.clone(), preferences_store, Arc::new(EmptyPreferences)));
        let device_api = Arc::new(GemDeviceApiClient::new(provider.clone(), Arc::new(GemDeviceKeyService::new(Arc::new(EmptyPreferences)))));
        let asset_store = Arc::new(MemoryAssetStore::default());
        let price = Arc::new(GemPriceService::new(Arc::new(MemoryPriceStore::default())));
        let assets = Arc::new(GemAssetsService::new(
            Arc::new(GemApiClient::new(provider.clone())),
            gateway.clone(),
            asset_store.clone(),
            price,
            preferences.clone(),
            session.clone(),
        ));
        let balances = Arc::new(RecordingBalanceStore::default());
        let balance = Arc::new(GemBalanceService::new(
            gateway,
            wallets,
            asset_store,
            balances.clone(),
            assets.clone(),
            Arc::new(SubscriptionTestkit::new(&[], &[]).service),
        ));
        let wallet_preferences = Arc::new(GemWalletPreferencesService::new(Arc::new(MemoryWalletPreferencesStore::default())));
        let transactions = Arc::new(GemTransactionsService::new(
            device_api.clone(),
            assets,
            Arc::new(MemoryTransactionStore::default()),
            Arc::new(MemoryAddressStore::default()),
            wallet_preferences.clone(),
            preferences.clone(),
            session.clone(),
            Arc::new(RecordingTransactionStatus::default()),
        ));
        let nft = Arc::new(GemNftService::new(device_api.clone(), Arc::new(MemoryNftStore::default()), session.clone()));
        let discovery = Arc::new(GemAssetDiscoveryService::new(
            device_api,
            balance.clone(),
            transactions,
            nft,
            Arc::new(MemoryWalletStore {
                wallets: Mutex::new(vec![wallet.clone()]),
                ..Default::default()
            }),
            wallet_preferences.clone(),
        ));
        let service = GemWalletHomeService::new(
            balance,
            discovery,
            Arc::new(GemBannerService::new(Arc::new(MemoryBannerStore::default()))),
            wallet_preferences.clone(),
            preferences,
            session,
        );
        Self {
            service,
            provider,
            balances,
            wallet_preferences,
            wallet_id: wallet.id,
        }
    }
}
