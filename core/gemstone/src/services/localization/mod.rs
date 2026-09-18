use primitives::{Chain, DelegationState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum GemLocalizedText {
    WalletDefaultName { index: i32 },
    WalletDefaultNameChain { chain: Chain, index: i32 },
    WalletMulticoin,
    ChainNetworkName { chain: Chain },
    DelegationState { state: DelegationState },
}
