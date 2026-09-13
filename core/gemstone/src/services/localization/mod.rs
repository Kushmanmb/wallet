#[cfg(test)]
pub(crate) mod testkit;

use primitives::Chain;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum GemLocalizedText {
    WalletDefaultName { index: i32 },
    WalletDefaultNameChain { chain: Chain, index: i32 },
}

#[uniffi::export(rust, foreign)]
pub trait GemLocalizer: Send + Sync {
    fn text(&self, text: GemLocalizedText) -> String;
}
