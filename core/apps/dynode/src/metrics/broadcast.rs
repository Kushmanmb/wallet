use primitives::Chain;
use prometheus_client::encoding::EncodeLabelSet;
use settings_chain::BroadcastProviders;

use super::Metrics;
use super::traffic::TrafficLabels;
use crate::BoxError;
use crate::proxy::ProxyResponse;
use crate::proxy::proxy_request::ProxyRequest;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub(super) struct BroadcastLabels {
    #[prometheus(flatten)]
    traffic: TrafficLabels,
    chain: String,
    outcome: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BroadcastOutcome {
    Success,
    Failure,
}

impl BroadcastOutcome {
    fn from_response(chain: Chain, response: &Result<ProxyResponse, BoxError>, providers: &BroadcastProviders) -> Self {
        match response {
            Err(_) => Self::Failure,
            Ok(response) if !(200..300).contains(&response.status) => Self::Failure,
            Ok(response) => match providers.decode_transaction_broadcast(chain, &response.body) {
                Some(identifier) if !identifier.is_empty() => Self::Success,
                Some(_) | None => Self::Failure,
            },
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
        }
    }
}

impl Metrics {
    pub(crate) fn initialize_transaction_broadcasts(&self, chains: impl IntoIterator<Item = Chain>) {
        for chain in chains {
            for outcome in [BroadcastOutcome::Success, BroadcastOutcome::Failure] {
                let labels = self.transaction_broadcast_labels(chain, outcome);
                drop(self.transaction_broadcasts.get_or_create(&labels));
                drop(self.transaction_broadcast_latency.get_or_create(&labels));
            }
        }
    }

    fn transaction_broadcast_labels(&self, chain: Chain, outcome: BroadcastOutcome) -> BroadcastLabels {
        BroadcastLabels {
            traffic: TrafficLabels::node(&self.source, chain.as_ref()),
            chain: chain.to_string(),
            outcome: outcome.as_str(),
        }
    }

    pub(crate) fn record_transaction_broadcast(&self, request: &ProxyRequest, response: &Result<ProxyResponse, BoxError>, providers: &BroadcastProviders) {
        let labels = self.transaction_broadcast_labels(request.chain, BroadcastOutcome::from_response(request.chain, response, providers));
        self.transaction_broadcasts.get_or_create(&labels).inc();
        self.transaction_broadcast_latency.get_or_create(&labels).observe(request.elapsed().as_secs_f64() * 1000.0);
    }
}

#[cfg(test)]
mod tests {
    use primitives::Chain;
    use reqwest::Method;
    use reqwest::header::HeaderMap;

    use super::*;
    use crate::testkit::config::metrics_config;

    #[test]
    fn test_broadcast_outcomes_require_chain_acceptance() {
        let providers = BroadcastProviders::from_chains([Chain::Ethereum, Chain::Tron]);
        for (chain, status, body, expected) in [
            (Chain::Ethereum, 200, r#"{"jsonrpc":"2.0","id":1,"result":"0xabc"}"#, BroadcastOutcome::Success),
            (
                Chain::Ethereum,
                200,
                r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"insufficient funds"}}"#,
                BroadcastOutcome::Failure,
            ),
            (Chain::Ethereum, 429, r#"{"jsonrpc":"2.0","id":1,"result":"0xabc"}"#, BroadcastOutcome::Failure),
            (Chain::Ethereum, 200, "invalid response", BroadcastOutcome::Failure),
            (Chain::Ethereum, 200, r#"{"jsonrpc":"2.0","id":1,"result":""}"#, BroadcastOutcome::Failure),
            (Chain::Tron, 200, r#"{"result":true,"txid":"abc"}"#, BroadcastOutcome::Success),
            (
                Chain::Tron,
                200,
                r#"{"result":false,"txid":"abc","code":"SIGERROR","message":"invalid signature"}"#,
                BroadcastOutcome::Failure,
            ),
        ] {
            let response = Ok(ProxyResponse::new(status, HeaderMap::new(), body.as_bytes().to_vec()));
            assert_eq!(BroadcastOutcome::from_response(chain, &response, &providers), expected);
        }
        assert_eq!(
            BroadcastOutcome::from_response(Chain::Ethereum, &Err("connection failed".into()), &providers),
            BroadcastOutcome::Failure
        );
    }

    #[test]
    fn test_broadcast_metrics_exclude_transaction_data() {
        let metrics = Metrics::new(metrics_config());
        let providers = BroadcastProviders::from_chains([Chain::Ethereum]);
        let request = ProxyRequest::new(
            Method::POST,
            HeaderMap::new(),
            vec![],
            "/".into(),
            "/".into(),
            "example.com".into(),
            "agent".into(),
            Chain::Ethereum,
        );
        let response = Ok(ProxyResponse::new(
            200,
            HeaderMap::new(),
            br#"{"jsonrpc":"2.0","id":1,"result":"private-identifier"}"#.to_vec(),
        ));
        metrics.record_transaction_broadcast(&request, &response, &providers);
        let encoded = metrics.get_metrics();
        assert_eq!(encoded.find("private-identifier"), None);
        for name in ["dynode_transaction_broadcasts_total", "dynode_transaction_broadcast_latency_milliseconds_count"] {
            assert_eq!(
                encoded.lines().filter(|line| line.starts_with(&format!("{name}{{"))).collect::<Vec<_>>(),
                vec![format!(
                    "{name}{{source=\"public\",group=\"evm\",service=\"ethereum\",chain=\"ethereum\",outcome=\"success\"}} 1"
                )]
            );
        }
    }
}
