mod decoder;
mod error;
mod model;
mod provider;
mod service;
mod solana_pay;

use primitives::UrlAction;

pub use decoder::{PaymentDecoderError, PaymentURLDecoder};
pub use error::PaymentError;
pub use model::PaymentTransaction;
pub use service::PaymentService;

pub fn classify_url(url: &str) -> Option<UrlAction> {
    UrlAction::from_url(url).or_else(|| PaymentURLDecoder::decode(url).ok().map(|payment| UrlAction::Payment { payment }))
}
