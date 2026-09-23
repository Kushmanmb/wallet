use std::collections::HashSet;

use primitives::{AssetBasic, AssetId, ChainAddress, ScanAddress, ScanProvider, ScanTransactionPayload, ScanType, ScanVerdict};

use crate::{AddressPoisoningTarget, AddressTarget, WebsiteTarget};

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

impl ScanTargets {
    pub fn contains(&self, scan_type: ScanType) -> bool {
        match scan_type {
            ScanType::Address => self.address.is_some(),
            ScanType::AddressPoisoning => self.poisoning.is_some(),
            ScanType::Website => self.website.is_some(),
            ScanType::Asset => false,
        }
    }

    pub fn retain(self, keep: impl Fn(ScanType) -> bool) -> Self {
        Self {
            address: self.address.filter(|_| keep(ScanType::Address)),
            poisoning: self.poisoning.filter(|_| keep(ScanType::AddressPoisoning)),
            website: self.website.filter(|_| keep(ScanType::Website)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScanPlan {
    pub detections: Vec<ScanDetection>,
    pub is_memo_required: bool,
    pub targets: Option<ScanTargets>,
    pub safe: Vec<ScanType>,
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
