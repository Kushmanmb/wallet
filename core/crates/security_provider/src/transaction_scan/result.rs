use std::collections::{BTreeMap, BTreeSet};

use primitives::{ScanSource, ScanTransaction, ScanType, ScanVerdict};

use super::check::ProviderCheck;
use super::model::ScanDetection;

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
            .filter_map(|check| check.error.as_ref().map(|error| format!("{}/{}: {}", check.provider.as_ref(), check.scan_type.as_ref(), error)))
            .collect::<Vec<_>>()
            .join("; ")
    }

    pub fn dry_run(&self) -> BTreeSet<ScanType> {
        self.detections.iter().filter(|detection| !detection.is_enforced).map(|detection| detection.scan_type).collect()
    }

    pub fn providers(&self) -> BTreeMap<&str, BTreeMap<&str, &ProviderCheck>> {
        let mut providers: BTreeMap<&str, BTreeMap<&str, &ProviderCheck>> = BTreeMap::new();
        for check in &self.checks {
            providers.entry(check.provider.as_ref()).or_default().insert(check.scan_type.as_ref(), check);
        }
        providers
    }
}
