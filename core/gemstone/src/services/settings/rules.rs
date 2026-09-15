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

pub fn sections(notifications_available: bool, wallet_connect_available: bool, shows_rewards: bool, developer_enabled: bool) -> Vec<GemSettingsSection> {
    [
        vec![GemSettingsRow::Wallets, GemSettingsRow::Security],
        [
            notifications_available.then_some(GemSettingsRow::Notifications),
            Some(GemSettingsRow::Preferences),
        ]
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
    fn test_the_settings_rows_follow_what_the_device_and_wallet_offer() {
        let full = sections(true, true, true, true);
        assert_eq!(
            full.iter().map(|section| section.rows.clone()).collect::<Vec<_>>(),
            vec![
                vec![GemSettingsRow::Wallets, GemSettingsRow::Security],
                vec![GemSettingsRow::Notifications, GemSettingsRow::Preferences],
                vec![GemSettingsRow::WalletConnect],
                vec![
                    GemSettingsRow::Support,
                    GemSettingsRow::Rewards,
                    GemSettingsRow::AboutUs,
                    GemSettingsRow::Developer
                ],
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
