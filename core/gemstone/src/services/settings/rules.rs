use primitives::{PlatformStore, Release};

use crate::config::public::PublicUrl;
use crate::config::social::community_links;
use crate::models::list::{GemListRow, GemListRowIcon, GemListRowTitle, GemListSection, GemListSectionTitle, GemUrlTarget};
use crate::services::currency::GemCurrencyRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemSettingsRow {
    Wallets,
    Security,
    Notifications,
    Preferences,
    WalletConnect,
    Support,
    Rewards,
    AboutUs,
    Developer,
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct GemSettingsSection {
    pub rows: Vec<GemSettingsRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemPreferencesRow {
    Currency,
    Language,
    Appearance,
    Networks,
    Contacts,
    Perpetuals,
    PerpetualLeverage,
    PerpetualTakeProfit,
    PerpetualStopLoss,
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct GemPreferencesSection {
    pub rows: Vec<GemPreferencesRow>,
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct GemPreferencesState {
    pub currency: GemCurrencyRow,
    pub sections: Vec<GemPreferencesSection>,
    pub perpetual_defaults: GemPerpetualDefaults,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct GemPerpetualDefaults {
    pub leverage: u8,
    pub take_profit_percent: u8,
    pub stop_loss_percent: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemSecurityRow {
    Authentication,
    LockPeriod,
    PrivacyLock,
    HideBalance,
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct GemSecuritySection {
    pub rows: Vec<GemSecurityRow>,
}

pub fn preferences_sections(perpetuals_enabled: bool) -> Vec<GemPreferencesSection> {
    [
        vec![
            GemPreferencesRow::Currency,
            GemPreferencesRow::Language,
            GemPreferencesRow::Appearance,
            GemPreferencesRow::Networks,
            GemPreferencesRow::Contacts,
        ],
        [
            Some(GemPreferencesRow::Perpetuals),
            perpetuals_enabled.then_some(GemPreferencesRow::PerpetualLeverage),
            perpetuals_enabled.then_some(GemPreferencesRow::PerpetualTakeProfit),
            perpetuals_enabled.then_some(GemPreferencesRow::PerpetualStopLoss),
        ]
        .into_iter()
        .flatten()
        .collect(),
    ]
    .into_iter()
    .map(|rows| GemPreferencesSection { rows })
    .collect()
}

pub fn security_sections(authentication_enabled: bool) -> Vec<GemSecuritySection> {
    [
        [
            Some(GemSecurityRow::Authentication),
            authentication_enabled.then_some(GemSecurityRow::LockPeriod),
            authentication_enabled.then_some(GemSecurityRow::PrivacyLock),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
        vec![GemSecurityRow::HideBalance],
    ]
    .into_iter()
    .map(|rows| GemSecuritySection { rows })
    .collect()
}

pub fn about_sections(version: String, update: Option<Release>) -> Vec<GemListSection> {
    let page = |title: GemListRowTitle, url: PublicUrl| GemListRow::Url {
        title,
        value: None,
        icon: GemListRowIcon::None,
        url: url.url(),
        target: GemUrlTarget::InApp,
    };
    vec![
        GemListSection {
            title: GemListSectionTitle::None,
            rows: vec![
                page(GemListRowTitle::TermsOfService, PublicUrl::TermsOfService),
                page(GemListRowTitle::PrivacyPolicy, PublicUrl::PrivacyPolicy),
                page(GemListRowTitle::Website, PublicUrl::Website),
            ],
        },
        GemListSection {
            title: GemListSectionTitle::Community,
            rows: vec![GemListRow::Social { links: community_links() }],
        },
        GemListSection {
            title: GemListSectionTitle::None,
            rows: [
                Some(GemListRow::Text {
                    title: GemListRowTitle::Version,
                    value: version,
                }),
                update.map(|release| GemListRow::Url {
                    title: GemListRowTitle::UpdateApp,
                    value: Some(release.version),
                    icon: GemListRowIcon::AppLogo,
                    url: store_url(release.store).url(),
                    target: GemUrlTarget::External,
                }),
            ]
            .into_iter()
            .flatten()
            .collect(),
        },
    ]
}

fn store_url(store: PlatformStore) -> PublicUrl {
    match store {
        PlatformStore::AppStore => PublicUrl::AppStore,
        PlatformStore::GooglePlay => PublicUrl::PlayStore,
        PlatformStore::Fdroid
        | PlatformStore::Huawei
        | PlatformStore::SolanaStore
        | PlatformStore::SamsungStore
        | PlatformStore::ApkUniversal
        | PlatformStore::Emerald
        | PlatformStore::Local => PublicUrl::APK,
    }
}

pub fn sections(notifications_available: bool, wallet_connect_available: bool, shows_rewards: bool, developer_enabled: bool) -> Vec<GemSettingsSection> {
    [
        vec![GemSettingsRow::Wallets, GemSettingsRow::Security],
        [notifications_available.then_some(GemSettingsRow::Notifications), Some(GemSettingsRow::Preferences)]
            .into_iter()
            .flatten()
            .collect(),
        wallet_connect_available.then_some(vec![GemSettingsRow::WalletConnect]).unwrap_or_default(),
        [
            Some(GemSettingsRow::Support),
            shows_rewards.then_some(GemSettingsRow::Rewards),
            Some(GemSettingsRow::AboutUs),
            developer_enabled.then_some(GemSettingsRow::Developer),
        ]
        .into_iter()
        .flatten()
        .collect(),
    ]
    .into_iter()
    .filter(|rows: &Vec<GemSettingsRow>| !rows.is_empty())
    .map(|rows| GemSettingsSection { rows })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_perpetual_defaults_show_only_once_perpetuals_are_on() {
        assert_eq!(
            preferences_sections(false).last().map(|section| section.rows.clone()),
            Some(vec![GemPreferencesRow::Perpetuals])
        );
        assert_eq!(preferences_sections(true).last().map(|section| section.rows.len()), Some(4));
    }

    #[test]
    fn test_the_lock_rows_show_only_once_authentication_is_on() {
        assert_eq!(
            security_sections(false).first().map(|section| section.rows.clone()),
            Some(vec![GemSecurityRow::Authentication])
        );
        assert_eq!(
            security_sections(true).first().map(|section| section.rows.clone()),
            Some(vec![GemSecurityRow::Authentication, GemSecurityRow::LockPeriod, GemSecurityRow::PrivacyLock])
        );
        assert_eq!(
            security_sections(true).last().map(|section| section.rows.clone()),
            Some(vec![GemSecurityRow::HideBalance]),
            "hiding the balance is its own choice, not part of the lock"
        );
    }

    #[test]
    fn test_the_about_screen_offers_the_update_only_when_a_release_is_newer() {
        let plain = about_sections("1.2.3".to_string(), None);
        assert_eq!(
            plain.last().map(|section| section.rows.clone()),
            Some(vec![GemListRow::Text {
                title: GemListRowTitle::Version,
                value: "1.2.3".to_string()
            }])
        );

        let update = about_sections("1.2.3".to_string(), Some(Release::new(PlatformStore::AppStore, "1.3.0".to_string(), false)));
        assert_eq!(
            update.last().and_then(|section| section.rows.last().cloned()),
            Some(GemListRow::Url {
                title: GemListRowTitle::UpdateApp,
                value: Some("1.3.0".to_string()),
                icon: GemListRowIcon::AppLogo,
                url: PublicUrl::AppStore.url(),
                target: GemUrlTarget::External,
            }),
            "the row points at the store the release came from, and a store page opens outside the app"
        );
    }

    #[test]
    fn test_the_settings_rows_follow_what_the_device_and_wallet_offer() {
        let full = sections(true, true, true, true);
        assert_eq!(
            full.iter().map(|section| section.rows.clone()).collect::<Vec<_>>(),
            vec![
                vec![GemSettingsRow::Wallets, GemSettingsRow::Security],
                vec![GemSettingsRow::Notifications, GemSettingsRow::Preferences],
                vec![GemSettingsRow::WalletConnect],
                vec![GemSettingsRow::Support, GemSettingsRow::Rewards, GemSettingsRow::AboutUs, GemSettingsRow::Developer],
            ]
        );

        let plain = sections(false, false, false, false);
        assert_eq!(
            plain.iter().map(|section| section.rows.clone()).collect::<Vec<_>>(),
            vec![
                vec![GemSettingsRow::Wallets, GemSettingsRow::Security],
                vec![GemSettingsRow::Preferences],
                vec![GemSettingsRow::Support, GemSettingsRow::AboutUs],
            ],
            "a device without notifications or WalletConnect drops those rows and their empty section"
        );
    }
}
