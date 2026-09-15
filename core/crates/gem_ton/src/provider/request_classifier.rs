use chain_traits::ChainRequestClassifier;
use primitives::{ChainRequest, ChainRequestType};

use crate::provider::BroadcastProvider;

impl ChainRequestClassifier for BroadcastProvider {
    fn classify_request(&self, request: ChainRequest<'_>) -> ChainRequestType {
        if request.is_http_post_path("/api/v3/message") || request.is_http_post_path("/api/v2/sendBocReturnHash") {
            ChainRequestType::Broadcast
        } else {
            ChainRequestType::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::ChainRequestProtocol;

    #[test]
    fn test_classify_broadcast_versions() {
        for path in ["/api/v2/sendBocReturnHash", "/api/v3/message"] {
            assert_eq!(
                BroadcastProvider.classify_request(ChainRequest::new(ChainRequestProtocol::Http, "POST", path, b"{}")),
                ChainRequestType::Broadcast
            );
            assert_eq!(
                BroadcastProvider.classify_request(ChainRequest::new(ChainRequestProtocol::Http, "GET", path, b"")),
                ChainRequestType::Unknown
            );
        }
    }
}
