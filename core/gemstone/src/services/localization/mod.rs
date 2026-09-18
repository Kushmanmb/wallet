use primitives::{Chain, DelegationState, FeeUnitType, Resource, TransactionState};

use crate::duration_formatter::GemDurationPart;
use crate::formatted_number::GemFormattedNumber;

#[derive(Debug, Clone, PartialEq, uniffi::Enum)]
pub enum GemLocalizedText {
    WalletDefaultName { index: i32 },
    WalletDefaultNameChain { chain: Chain, index: i32 },
    WalletMulticoin,
    ChainNetworkName { chain: Chain },
    DelegationState { state: DelegationState },
    TransactionState { state: TransactionState },
    Resource { resource: Resource },
    FeeRate { rate: GemFormattedNumber, unit: FeeUnitType },
    Text { text: String },
    RewardsUnverified,
    RewardsPending { countdown: Vec<GemDurationPart> },
    RewardsPendingReady,
}
