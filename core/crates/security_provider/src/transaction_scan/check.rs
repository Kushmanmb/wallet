use std::error::Error;
use std::time::Duration;

use gem_tracing::DurationMs;
use primitives::{ScanProvider, ScanType};
use serde::Serialize;

use crate::ScanResult;

#[derive(Debug, Clone, Serialize)]
pub struct ProviderCheck {
    #[serde(skip)]
    pub provider: ScanProvider,
    #[serde(skip)]
    pub scan_type: ScanType,
    pub latency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malicious: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ProviderCheck {
    pub fn new<T>(provider: ScanProvider, scan_type: ScanType, result: Result<ScanResult<T>, Box<dyn Error + Send + Sync>>, latency: Duration) -> Self {
        let (malicious, reason, error) = match result {
            Ok(result) => (Some(result.is_malicious), result.reason, None),
            Err(error) => (None, None, Some(error.to_string())),
        };
        Self {
            provider,
            scan_type,
            latency: DurationMs(latency).to_string(),
            malicious,
            reason,
            error,
        }
    }
}

#[cfg(test)]
impl ProviderCheck {
    pub fn mock(provider: ScanProvider, scan_type: ScanType, malicious: Option<bool>) -> Self {
        let result = match malicious {
            Some(is_malicious) => Ok(ScanResult {
                target: (),
                is_malicious,
                reason: is_malicious.then(|| "phishing".to_string()),
                provider: provider.as_ref().to_string(),
            }),
            None => Err("timeout".into()),
        };
        Self::new(provider, scan_type, result, Duration::from_millis(100))
    }
}
