use chrono::{DateTime, Utc};
use primitives::{Chain, TransactionState};

use crate::config::social::GemSocialLink;
use crate::duration_formatter::GemDurationPart;
use crate::formatted_number::{GemFormattedNumber, GemValueTone};
use crate::models::copy::GemCopy;
use crate::services::error::GemServiceError;
use crate::services::localization::GemLocalizedText;
use crate::services::transactions::GemTransactionStateTone;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemListSectionTitle {
    None,
    Balances,
    Community,
    Manage,
    Resources,
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
    Authentication,
    LockPeriod,
    PrivacyLock,
    HideBalance,
    Currency,
    Language,
    Appearance,
    Networks,
    Contacts,
    Perpetuals,
    PerpetualLeverage,
    PerpetualTakeProfit,
    PerpetualStopLoss,
    DailyVolume,
    OpenInterest,
    FundingApr,
    StakeApr,
    LockTime,
    MinimumAmount,
    Validator,
    Provider,
    Status,
    ActiveIn,
    AvailableIn,
    Date,
    Memo,
    Resource,
    Price,
    Pnl,
    Pin,
    Unpin,
    AddToWallet,
    PriceAlerts,
    Energy,
    Bandwidth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemInfoTopic {
    OpenInterest,
    FundingApr,
    StakeApr,
    StakeLockTime,
    TransactionStatus { state: TransactionState, tone: GemTransactionStateTone },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemListSectionFooter {
    None,
    Authentication,
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
    Currency,
    Language,
    Appearance,
    Networks,
    Contacts,
    Perpetuals,
    Pin,
    Unpin,
    AddToWallet,
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
        info: Option<GemInfoTopic>,
    },
    Duration {
        title: GemListRowTitle,
        parts: Vec<GemDurationPart>,
        info: Option<GemInfoTopic>,
    },
    Label {
        title: GemListRowTitle,
        text: GemLocalizedText,
        tone: GemValueTone,
        info: Option<GemInfoTopic>,
        progress: bool,
    },
    Date {
        title: GemListRowTitle,
        date: DateTime<Utc>,
    },
    Network {
        title: GemListRowTitle,
        chain: Chain,
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
    Toggle {
        title: GemListRowTitle,
        value: Option<String>,
        icon: GemListRowIcon,
        is_on: bool,
    },
    Picker {
        title: GemListRowTitle,
        value: String,
        icon: GemListRowIcon,
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
    pub footer: GemListSectionFooter,
    pub rows: Vec<GemListRow>,
}
