pub mod rules;

use std::sync::Arc;

use primitives::{Currency, Wallet};

use crate::services::currency;
use crate::services::preferences::GemPreferencesService;
use crate::services::wallet_session;

pub use rules::{
    GemAboutRow, GemAboutSection, GemPreferencesRow, GemPreferencesSection, GemPreferencesState, GemSecurityRow, GemSecuritySection, GemSettingsRow, GemSettingsSection,
};

#[derive(uniffi::Object)]
pub struct GemSettingsService {
    preferences: Arc<GemPreferencesService>,
}

#[uniffi::export]
impl GemSettingsService {
    #[uniffi::constructor]
    pub fn new(preferences: Arc<GemPreferencesService>) -> Self {
        Self { preferences }
    }

    pub fn preferences(&self, currency: Currency, perpetuals_enabled: bool) -> GemPreferencesState {
        GemPreferencesState {
            currency: currency::rules::row(currency),
            sections: rules::preferences_sections(perpetuals_enabled),
        }
    }

    pub fn security_sections(&self, authentication_enabled: bool) -> Vec<GemSecuritySection> {
        rules::security_sections(authentication_enabled)
    }

    pub fn sections(&self, wallets: Vec<Wallet>, notifications_available: bool, wallet_connect_available: bool) -> Vec<GemSettingsSection> {
        rules::sections(
            notifications_available,
            wallet_connect_available,
            wallet_session::rules::shows_rewards(&wallets),
            self.preferences.is_developer_enabled(),
        )
    }
}

#[uniffi::export]
pub fn about_sections() -> Vec<GemAboutSection> {
    rules::about_sections()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::preferences::testkit::MemoryPreferencesStore;

    fn service() -> GemSettingsService {
        GemSettingsService::new(Arc::new(GemPreferencesService::new(Arc::new(MemoryPreferencesStore::default()))))
    }

    #[test]
    fn test_the_preferences_screen_flags_the_selected_currency_beside_its_rows() {
        let state = service().preferences(Currency::GBP, false);

        assert_eq!(state.currency.text(), "\u{1f1ec}\u{1f1e7} GBP");
        assert_eq!(state.sections, rules::preferences_sections(false));
    }
}
