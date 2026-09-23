use diesel::prelude::*;
use primitives::{Chain, ScanProvider, ScanType};
use serde::{Deserialize, Serialize};

use crate::sql_types::{ChainRow, ScanProviderRow, ScanTypeRow};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::scan_detections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ScanDetectionRow {
    pub id: i32,
    pub scan_type: ScanTypeRow,
    pub chain: Option<ChainRow>,
    pub target: String,
    pub provider: ScanProviderRow,
    pub reason: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

impl ScanDetectionRow {
    pub fn matches(&self, scan_type: ScanType, chain: Option<Chain>, target: &str) -> bool {
        self.scan_type.0 == scan_type && self.chain.as_ref().map(|chain| chain.0) == chain && self.target == target
    }
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::scan_detections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewScanDetectionRow {
    pub scan_type: ScanTypeRow,
    pub chain: Option<ChainRow>,
    pub target: String,
    pub provider: ScanProviderRow,
    pub reason: Option<String>,
}

impl NewScanDetectionRow {
    pub fn new(scan_type: ScanType, chain: Option<Chain>, target: String, provider: ScanProvider, reason: Option<String>) -> Self {
        Self {
            scan_type: scan_type.into(),
            chain: chain.map(ChainRow::from),
            target,
            provider: provider.into(),
            reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;

    #[test]
    fn test_matches() {
        let row = ScanDetectionRow {
            id: 1,
            scan_type: ScanType::Address.into(),
            chain: Some(Chain::SmartChain.into()),
            target: "0x123".to_string(),
            provider: ScanProvider::HashDit.into(),
            reason: None,
            updated_at: DateTime::UNIX_EPOCH.naive_utc(),
            created_at: DateTime::UNIX_EPOCH.naive_utc(),
        };

        assert!(row.matches(ScanType::Address, Some(Chain::SmartChain), "0x123"));
        assert!(!row.matches(ScanType::AddressPoisoning, Some(Chain::SmartChain), "0x123"));
        assert!(!row.matches(ScanType::Address, Some(Chain::Ethereum), "0x123"));
        assert!(!row.matches(ScanType::Address, None, "0x123"));
        assert!(!row.matches(ScanType::Address, Some(Chain::SmartChain), "0x456"));
    }
}
