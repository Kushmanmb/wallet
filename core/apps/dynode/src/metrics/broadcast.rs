use gem_tracing::info_with_fields;
use primitives::{Chain, ValueAccess};
use prometheus_client::encoding::EncodeLabelSet;
use serde_json::Value;
use settings_chain::BroadcastProviders;

use super::Metrics;
use super::traffic::{TrafficLabels, chain_group};
use crate::BoxError;
use crate::failure_reason::FailureReason;
use crate::proxy::ProxyResponse;
use crate::proxy::proxy_request::ProxyRequest;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub(super) struct BroadcastLabels {
    #[prometheus(flatten)]
    traffic: TrafficLabels,
    chain: String,
    outcome: &'static str,
}

impl Metrics {
    pub(crate) fn initialize_transaction_broadcasts(&self, chains: impl IntoIterator<Item = Chain>) {
        for chain in chains {
            for outcome in ["success", "failure"] {
                let labels = self.transaction_broadcast_labels(chain, outcome);
                drop(self.transaction_broadcasts.get_or_create(&labels));
                drop(self.transaction_broadcast_latency.get_or_create(&labels));
            }
        }
    }

    fn transaction_broadcast_labels(&self, chain: Chain, outcome: &'static str) -> BroadcastLabels {
        BroadcastLabels {
            traffic: TrafficLabels::node(&self.source, chain.as_ref()),
            chain: chain.to_string(),
            outcome,
        }
    }

    pub(crate) fn record_transaction_broadcast(&self, request: &ProxyRequest, response: &Result<ProxyResponse, BoxError>, providers: &BroadcastProviders) {
        let outcome = match broadcast_result(request.chain, response, providers) {
            Ok(transaction_id) => {
                info_with_fields!(
                    "Broadcast accepted",
                    id = request.id.as_str(),
                    chain = request.chain.as_ref(),
                    transaction_id = transaction_id.as_str(),
                );
                "success"
            }
            Err(error) => {
                info_with_fields!(
                    "Broadcast failed",
                    id = request.id.as_str(),
                    chain = request.chain.as_ref(),
                    group = chain_group(request.chain),
                    error = error.as_str(),
                );
                "failure"
            }
        };
        let labels = self.transaction_broadcast_labels(request.chain, outcome);
        self.transaction_broadcasts.get_or_create(&labels).inc();
        self.transaction_broadcast_latency.get_or_create(&labels).observe(request.elapsed().as_secs_f64() * 1000.0);
    }
}

fn broadcast_result(chain: Chain, response: &Result<ProxyResponse, BoxError>, providers: &BroadcastProviders) -> Result<String, String> {
    if let Ok(response) = response
        && (200..300).contains(&response.status)
        && let Some(identifier) = providers.decode_transaction_broadcast(chain, &response.body)
        && !identifier.is_empty()
    {
        return Ok(identifier);
    }
    Err(broadcast_error_message(response))
}

fn broadcast_error_message(response: &Result<ProxyResponse, BoxError>) -> String {
    const MAX_ERROR_LENGTH: usize = 1024;
    let message = match response {
        Err(error) => FailureReason::from_error(error.as_ref()).to_string(),
        Ok(response) => serde_json::from_slice::<Value>(&response.body)
            .ok()
            .and_then(|body| {
                body.get_value("error")
                    .and_then(|error| error.get_string("message"))
                    .or_else(|_| body.get_string("message"))
                    .ok()
                    .filter(|message| !message.trim().is_empty())
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| format!("Broadcast rejected or response could not be decoded (HTTP {})", response.status)),
    };
    message.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(MAX_ERROR_LENGTH).collect()
}

#[cfg(test)]
mod tests {
    use primitives::Chain;
    use reqwest::Method;
    use reqwest::header::HeaderMap;

    use super::*;
    use crate::testkit::config::metrics_config;

    #[test]
    fn test_broadcast_result_requires_chain_acceptance() {
        let providers = BroadcastProviders::from_chains([Chain::Ethereum, Chain::Tron]);
        for (chain, status, body, expected) in [
            (Chain::Ethereum, 200, r#"{"jsonrpc":"2.0","id":1,"result":"0xabc"}"#, Ok("0xabc")),
            (
                Chain::Ethereum,
                200,
                r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"insufficient funds"}}"#,
                Err("insufficient funds"),
            ),
            (
                Chain::Ethereum,
                429,
                r#"{"jsonrpc":"2.0","id":1,"result":"0xabc"}"#,
                Err("Broadcast rejected or response could not be decoded (HTTP 429)"),
            ),
            (
                Chain::Ethereum,
                200,
                "invalid response",
                Err("Broadcast rejected or response could not be decoded (HTTP 200)"),
            ),
            (
                Chain::Ethereum,
                200,
                r#"{"jsonrpc":"2.0","id":1,"result":""}"#,
                Err("Broadcast rejected or response could not be decoded (HTTP 200)"),
            ),
            (Chain::Tron, 200, r#"{"result":true,"txid":"abc"}"#, Ok("abc")),
            (
                Chain::Tron,
                200,
                r#"{"result":false,"txid":"abc","code":"SIGERROR","message":"invalid signature"}"#,
                Err("invalid signature"),
            ),
        ] {
            let response = Ok(ProxyResponse::new(status, HeaderMap::new(), body.as_bytes().to_vec()));
            assert_eq!(broadcast_result(chain, &response, &providers).as_deref().map_err(String::as_str), expected);
        }
        assert_eq!(broadcast_result(Chain::Ethereum, &Err("connection failed".into()), &providers), Err("request_error".into()));
    }

    #[test]
    fn test_broadcast_error_message_excludes_response_data_and_transport_details() {
        let response = Ok(ProxyResponse::new(
            200,
            HeaderMap::new(),
            br#"{"error":{"message":"insufficient\nfunds","data":"signed-payload"},"id":1}"#.to_vec(),
        ));
        assert_eq!(broadcast_error_message(&response), "insufficient funds");
        let response = Ok(ProxyResponse::new(503, HeaderMap::new(), b"private-response-body".to_vec()));
        assert_eq!(broadcast_error_message(&response), "Broadcast rejected or response could not be decoded (HTTP 503)");
        assert_eq!(broadcast_error_message(&Err("https://node.example/secret-key".into())), "request_error");
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
