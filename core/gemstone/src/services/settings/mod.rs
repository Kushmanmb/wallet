pub mod rules;

use std::sync::Arc;

use primitives::Wallet;

use crate::services::preferences::GemPreferencesService;
use crate::services::wallet_session::GemWalletSessionService;

pub use rules::{GemSettingsRow, GemSettingsSection};

#[derive(uniffi::Object)]
pub struct GemSettingsService {
    preferences: Arc<GemPreferencesService>,
    session: Arc<GemWalletSessionService>,
}

#[uniffi::export]
impl GemSettingsService {
    #[uniffi::constructor]
    pub fn new(preferences: Arc<GemPreferencesService>, session: Arc<GemWalletSessionService>) -> Self {
        Self { preferences, session }
    }

    pub fn sections(&self, wallets: Vec<Wallet>, notifications_available: bool, wallet_connect_available: bool) -> Vec<GemSettingsSection> {
        rules::sections(
            notifications_available,
            wallet_connect_available,
            self.session.shows_rewards(wallets),
            self.preferences.is_developer_enabled(),
        )
    }
}
