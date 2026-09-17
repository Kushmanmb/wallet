use primitives::{PlatformStore, Release};

use crate::config::public::PublicUrl;
use crate::config::social::community_links;
use crate::models::list::{GemListRow, GemListRowIcon, GemListRowTitle, GemListSection, GemListSectionFooter, GemListSectionTitle, GemUrlTarget};
use crate::services::currency::GemCurrencyRow;

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

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct GemSecurityInput {
    pub authentication_enabled: bool,
    pub authentication_name: Option<String>,
    pub lock_period: String,
    pub privacy_lock_enabled: bool,
    pub privacy_lock_supported: bool,
    pub hide_balance_enabled: bool,
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

pub fn security_sections(input: GemSecurityInput) -> Vec<GemListSection> {
    let toggle = |title: GemListRowTitle, is_on: bool| GemListRow::Toggle {
        title,
        value: None,
        icon: GemListRowIcon::None,
        is_on,
    };
    vec![
        GemListSection {
            title: GemListSectionTitle::None,
            footer: GemListSectionFooter::Authentication,
            rows: [
                Some(GemListRow::Toggle {
                    title: GemListRowTitle::Authentication,
                    value: input.authentication_name,
                    icon: GemListRowIcon::None,
                    is_on: input.authentication_enabled,
                }),
                input.authentication_enabled.then_some(GemListRow::Picker {
                    title: GemListRowTitle::LockPeriod,
                    value: input.lock_period,
                    icon: GemListRowIcon::None,
                }),
                (input.authentication_enabled && input.privacy_lock_supported).then(|| toggle(GemListRowTitle::PrivacyLock, input.privacy_lock_enabled)),
            ]
            .into_iter()
            .flatten()
            .collect(),
        },
        GemListSection {
            title: GemListSectionTitle::None,
            footer: GemListSectionFooter::None,
            rows: vec![toggle(GemListRowTitle::HideBalance, input.hide_balance_enabled)],
        },
    ]
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
            footer: GemListSectionFooter::None,
            rows: vec![
                page(GemListRowTitle::TermsOfService, PublicUrl::TermsOfService),
                page(GemListRowTitle::PrivacyPolicy, PublicUrl::PrivacyPolicy),
                page(GemListRowTitle::Website, PublicUrl::Website),
            ],
        },
        GemListSection {
            title: GemListSectionTitle::Community,
            footer: GemListSectionFooter::None,
            rows: vec![GemListRow::Social { links: community_links() }],
        },
        GemListSection {
            title: GemListSectionTitle::None,
            footer: GemListSectionFooter::None,
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

pub fn sections(wallets_count: usize, notifications_available: bool, wallet_connect_available: bool, shows_rewards: bool, developer_enabled: bool) -> Vec<GemListSection> {
    let link = |title: GemListRowTitle, icon: GemListRowIcon| GemListRow::Link { title, value: None, icon };
    [
        vec![
            GemListRow::Link {
                title: GemListRowTitle::Wallets,
                value: Some(wallets_count.to_string()),
                icon: GemListRowIcon::Wallets,
            },
            link(GemListRowTitle::Security, GemListRowIcon::Security),
        ],
        [
            notifications_available.then(|| link(GemListRowTitle::Notifications, GemListRowIcon::Notifications)),
            Some(link(GemListRowTitle::Preferences, GemListRowIcon::Preferences)),
        ]
        .into_iter()
        .flatten()
        .collect(),
        match wallet_connect_available {
            true => vec![link(GemListRowTitle::WalletConnect, GemListRowIcon::WalletConnect)],
            false => vec![],
        },
        [
            Some(link(GemListRowTitle::Support, GemListRowIcon::Support)),
            shows_rewards.then(|| link(GemListRowTitle::Rewards, GemListRowIcon::Rewards)),
            Some(link(GemListRowTitle::AboutUs, GemListRowIcon::AboutUs)),
            developer_enabled.then(|| link(GemListRowTitle::Developer, GemListRowIcon::Developer)),
        ]
        .into_iter()
        .flatten()
        .collect(),
    ]
    .into_iter()
    .filter(|rows: &Vec<GemListRow>| !rows.is_empty())
    .map(|rows| GemListSection {
        title: GemListSectionTitle::None,
        footer: GemListSectionFooter::None,
        rows,
    })
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
        let input = |authentication_enabled: bool| GemSecurityInput {
            authentication_enabled,
            authentication_name: Some("Face ID".to_string()),
            lock_period: "Immediately".to_string(),
            privacy_lock_enabled: true,
            privacy_lock_supported: true,
            hide_balance_enabled: false,
        };

        assert_eq!(
            security_sections(input(false)).first().map(|section| section.rows.len()),
            Some(1),
            "a device without authentication offers only the switch that turns it on"
        );
        assert_eq!(
            security_sections(input(true))
                .first()
                .map(|section| section.rows.iter().filter_map(row_title).collect::<Vec<_>>()),
            Some(vec![GemListRowTitle::Authentication, GemListRowTitle::LockPeriod, GemListRowTitle::PrivacyLock])
        );
        assert_eq!(
            security_sections(GemSecurityInput {
                privacy_lock_supported: false,
                ..input(true)
            })
            .first()
            .map(|section| section.rows.iter().filter_map(row_title).collect::<Vec<_>>()),
            Some(vec![GemListRowTitle::Authentication, GemListRowTitle::LockPeriod]),
            "a platform without a privacy lock drops that row"
        );
        assert_eq!(
            security_sections(input(true)).last().map(|section| section.rows.clone()),
            Some(vec![GemListRow::Toggle {
                title: GemListRowTitle::HideBalance,
                value: None,
                icon: GemListRowIcon::None,
                is_on: false
            }]),
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
        let titles = |sections: Vec<GemListSection>| {
            sections
                .iter()
                .map(|section| section.rows.iter().filter_map(row_title).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        };

        assert_eq!(
            titles(sections(3, true, true, true, true)),
            vec![
                vec![GemListRowTitle::Wallets, GemListRowTitle::Security],
                vec![GemListRowTitle::Notifications, GemListRowTitle::Preferences],
                vec![GemListRowTitle::WalletConnect],
                vec![GemListRowTitle::Support, GemListRowTitle::Rewards, GemListRowTitle::AboutUs, GemListRowTitle::Developer],
            ]
        );

        assert_eq!(
            titles(sections(1, false, false, false, false)),
            vec![
                vec![GemListRowTitle::Wallets, GemListRowTitle::Security],
                vec![GemListRowTitle::Preferences],
                vec![GemListRowTitle::Support, GemListRowTitle::AboutUs],
            ],
            "a device without notifications or WalletConnect drops those rows and their empty section"
        );

        assert_eq!(
            sections(3, false, false, false, false).first().and_then(|section| section.rows.first().cloned()),
            Some(GemListRow::Link {
                title: GemListRowTitle::Wallets,
                value: Some("3".to_string()),
                icon: GemListRowIcon::Wallets,
            }),
            "the wallets row counts the wallets the screen was given"
        );
    }

    fn row_title(row: &GemListRow) -> Option<GemListRowTitle> {
        match row {
            GemListRow::Link { title, .. }
            | GemListRow::Text { title, .. }
            | GemListRow::Amount { title, .. }
            | GemListRow::Url { title, .. }
            | GemListRow::Toggle { title, .. }
            | GemListRow::Picker { title, .. } => Some(*title),
            GemListRow::Social { .. } | GemListRow::Icon { .. } | GemListRow::Address { .. } | GemListRow::Explorer { .. } | GemListRow::Loading | GemListRow::Error { .. } => None,
        }
    }
}
