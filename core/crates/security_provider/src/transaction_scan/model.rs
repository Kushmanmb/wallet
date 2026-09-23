use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::error::Error;
use std::time::Duration;

use gem_tracing::DurationMs;
use primitives::{AssetBasic, AssetId, ChainAddress, ScanAddress, ScanProvider, ScanSource, ScanTransaction, ScanTransactionPayload, ScanType, ScanVerdict};
use serde::Serialize;

use crate::{AddressPoisoningTarget, AddressTarget, ScanResult, WebsiteTarget};

pub struct TransactionScanInput {
    pub payload: ScanTransactionPayload,
    pub enforced: HashSet<ScanType>,
    pub addresses: Vec<ScanAddress>,
    pub assets: Vec<AssetBasic>,
    pub verdicts: Vec<ScanVerdict>,
    pub safe: HashSet<ScanType>,
    pub required_successes: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScanFinding {
    Address(ChainAddress),
    Asset(AssetId),
    Website(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScanDetection {
    pub scan_type: ScanType,
    pub finding: ScanFinding,
    pub is_enforced: bool,
    pub source: String,
}

impl ScanDetection {
    pub fn new(scan_type: ScanType, finding: ScanFinding, is_enforced: bool, source: impl Into<String>) -> Self {
        Self {
            scan_type,
            finding,
            is_enforced,
            source: source.into(),
        }
    }

    pub fn provider_source(provider: ScanProvider, reason: Option<&str>) -> String {
        format!("{}: {}", provider.as_ref(), reason.unwrap_or("malicious"))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScanTargets {
    pub address: Option<AddressTarget>,
    pub poisoning: Option<AddressPoisoningTarget>,
    pub website: Option<WebsiteTarget>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScanPlan {
    pub detections: Vec<ScanDetection>,
    pub is_memo_required: bool,
    pub targets: Option<ScanTargets>,
    pub safe: Vec<ScanType>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanCheck {
    pub latency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malicious: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProviderCheck {
    pub provider: ScanProvider,
    pub scan_type: ScanType,
    pub check: ScanCheck,
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
            check: ScanCheck {
                latency: DurationMs(latency).to_string(),
                malicious,
                reason,
                error,
            },
        }
    }
}

pub struct TransactionScanResult {
    pub scan: ScanTransaction,
    pub source: ScanSource,
    pub detections: Vec<ScanDetection>,
    pub new_verdicts: Vec<ScanVerdict>,
    pub safe: Vec<ScanType>,
    pub new_safe: Vec<ScanType>,
    pub checks: Vec<ProviderCheck>,
}

impl TransactionScanResult {
    pub fn findings(&self) -> String {
        self.detections.iter().map(|detection| format!("{} {}", detection.scan_type.as_ref(), detection.source)).collect::<Vec<_>>().join("; ")
    }

    pub fn errors(&self) -> String {
        self.checks
            .iter()
            .filter_map(|check| check.check.error.as_ref().map(|error| format!("{}/{}: {}", check.provider.as_ref(), check.scan_type.as_ref(), error)))
            .collect::<Vec<_>>()
            .join("; ")
    }

    pub fn dry_run(&self) -> BTreeSet<ScanType> {
        self.detections.iter().filter(|detection| !detection.is_enforced).map(|detection| detection.scan_type).collect()
    }

    pub fn providers(&self) -> BTreeMap<&str, BTreeMap<&str, &ScanCheck>> {
        let mut providers: BTreeMap<&str, BTreeMap<&str, &ScanCheck>> = BTreeMap::new();
        for check in &self.checks {
            providers.entry(check.provider.as_ref()).or_default().insert(check.scan_type.as_ref(), &check.check);
        }
        providers
    }
}

#[cfg(test)]
impl TransactionScanInput {
    pub fn mock(payload: ScanTransactionPayload) -> Self {
        Self {
            payload,
            enforced: ScanType::all().into_iter().collect(),
            addresses: vec![],
            assets: vec![],
            verdicts: vec![],
            safe: HashSet::new(),
            required_successes: 1,
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
