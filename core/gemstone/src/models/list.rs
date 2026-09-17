use primitives::Chain;

use crate::config::social::GemSocialLink;
use crate::formatted_number::GemFormattedNumber;
use crate::models::copy::GemCopy;
use crate::services::error::GemServiceError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemListSectionTitle {
    None,
    Balances,
    Community,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemListRowTitle {
    Name,
    Network,
    Address,
    Available,
    Stake,
    Earn,
    PendingUnconfirmed,
    Reserved,
    Error,
    TermsOfService,
    PrivacyPolicy,
    Website,
    Version,
    UpdateApp,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemUrlTarget {
    InApp,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemListRowIcon {
    None,
    AppLogo,
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

#[derive(Debug, Clone, PartialEq, uniffi::Enum)]
pub enum GemListRow {
    Text {
        title: GemListRowTitle,
        value: String,
    },
    Amount {
        title: GemListRowTitle,
        amount: GemFormattedNumber,
    },
    Link {
        title: GemListRowTitle,
        value: Option<String>,
        icon: GemListRowIcon,
    },
    Url {
        title: GemListRowTitle,
        value: Option<String>,
        icon: GemListRowIcon,
        url: String,
        target: GemUrlTarget,
    },
    Social {
        links: Vec<GemSocialLink>,
    },
    Icon {
        chain: Chain,
    },
    Address {
        address: String,
        copy: GemCopy,
    },
    Explorer {
        name: String,
        url: String,
    },
    Loading,
    Error {
        error: GemServiceError,
    },
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct GemListSection {
    pub title: GemListSectionTitle,
    pub rows: Vec<GemListRow>,
}
