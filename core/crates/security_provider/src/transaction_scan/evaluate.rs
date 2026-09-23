use primitives::{ChainAddress, ScanSource, ScanTransaction, ScanType, ScanVerdict};

use super::model::{ProviderCheck, ScanDetection, ScanFinding, ScanPlan, TransactionScanInput, TransactionScanResult};
use super::plan::website_host;

const PROVIDER_SCAN_TYPES: [ScanType; 3] = [ScanType::Address, ScanType::AddressPoisoning, ScanType::Website];

pub fn evaluate_transaction_scan(input: &TransactionScanInput, plan: ScanPlan, checks: Vec<ProviderCheck>) -> TransactionScanResult {
    let payload = &input.payload;
    let mut detections = plan.detections;
    let mut new_verdicts = Vec::new();

    for scan_type in PROVIDER_SCAN_TYPES {
        let Some(check) = checks.iter().find(|check| check.scan_type == scan_type && check.check.malicious == Some(true)) else {
            continue;
        };
        let (finding, chain, target) = match scan_type {
            ScanType::Website => {
                let (Some(website), Some(host)) = (payload.website.clone(), website_host(payload)) else {
                    continue;
                };
                (ScanFinding::Website(website), None, host)
            }
            _ => (
                ScanFinding::Address(ChainAddress::new(payload.target.asset_id.chain, payload.target.address.clone())),
                Some(payload.target.asset_id.chain),
                payload.target.address.clone(),
            ),
        };
        let is_enforced = input.enforced.contains(&scan_type);
        if is_enforced {
            new_verdicts.push(ScanVerdict {
                scan_type,
                chain,
                target,
                provider: check.provider,
                reason: check.check.reason.clone(),
            });
        }
        detections.push(ScanDetection::new(scan_type, finding, is_enforced, ScanDetection::provider_source(check.provider, check.check.reason.as_deref())));
    }

    let is_scan_complete = match plan.targets {
        None => true,
        Some(_) => {
            let completed = checks.iter().filter(|check| input.enforced.contains(&check.scan_type) && check.check.malicious.is_some()).count();
            completed >= input.required_successes
        }
    };
    let source = if checks.is_empty() { ScanSource::Local } else { ScanSource::Remote };

    TransactionScanResult {
        scan: scan_transaction(&detections, plan.is_memo_required, is_scan_complete),
        source,
        detections,
        new_verdicts,
        checks,
    }
}

fn scan_transaction(detections: &[ScanDetection], is_memo_required: bool, is_scan_complete: bool) -> ScanTransaction {
    let mut malicious_addresses = Vec::new();
    let mut malicious_assets = Vec::new();
    let mut malicious_website = None;
    for detection in detections.iter().filter(|detection| detection.is_enforced) {
        match &detection.finding {
            ScanFinding::Address(address) if !malicious_addresses.contains(address) => malicious_addresses.push(address.clone()),
            ScanFinding::Asset(asset_id) if !malicious_assets.contains(asset_id) => malicious_assets.push(asset_id.clone()),
            ScanFinding::Website(website) => malicious_website = Some(website.clone()),
            ScanFinding::Address(_) | ScanFinding::Asset(_) => {}
        }
    }
    ScanTransaction {
        is_malicious: Some(!malicious_addresses.is_empty() || !malicious_assets.is_empty() || malicious_website.is_some()),
        is_memo_required: Some(is_memo_required),
        is_scan_complete,
        malicious_addresses: Some(malicious_addresses),
        malicious_assets: Some(malicious_assets),
        malicious_website,
    }
}

#[cfg(test)]
mod tests {
    use primitives::{AssetId, Chain, ScanProvider, ScanTransactionPayload, TransactionType};

    use super::*;
    use crate::transaction_scan::plan_transaction_scan;
    use crate::transaction_scan::{ProviderCheck, TransactionScanInput};

    fn input(transaction_type: TransactionType, website: Option<&str>) -> TransactionScanInput {
        TransactionScanInput::mock(ScanTransactionPayload {
            transaction_type,
            website: website.map(str::to_string),
            ..ScanTransactionPayload::mock_with_assets(AssetId::from_chain(Chain::SmartChain), AssetId::from_chain(Chain::SmartChain))
        })
    }

    fn evaluate(input: &TransactionScanInput, checks: Vec<ProviderCheck>) -> TransactionScanResult {
        evaluate_transaction_scan(input, plan_transaction_scan(input), checks)
    }

    #[test]
    fn test_enforced_malicious_address_is_blocked_and_stored() {
        let input = input(TransactionType::Transfer, None);

        let result = evaluate(
            &input,
            vec![
                ProviderCheck::mock(ScanProvider::HashDit, ScanType::Address, Some(true)),
                ProviderCheck::mock(ScanProvider::HashDit, ScanType::AddressPoisoning, Some(false)),
            ],
        );

        assert_eq!(result.scan.is_malicious, Some(true));
        assert!(result.scan.is_scan_complete);
        assert_eq!(result.scan.malicious_addresses, Some(vec![ChainAddress::new(Chain::SmartChain, "target".to_string())]));
        assert_eq!(result.source, ScanSource::Remote);
        assert_eq!(
            result.new_verdicts,
            vec![ScanVerdict {
                scan_type: ScanType::Address,
                chain: Some(Chain::SmartChain),
                target: "target".to_string(),
                provider: ScanProvider::HashDit,
                reason: Some("phishing".to_string()),
            }]
        );
        assert_eq!(result.findings(), "address hashdit: phishing");
    }

    #[test]
    fn test_malicious_website_returns_url_and_stores_host() {
        let input = input(TransactionType::SmartContractCall, Some("https://bnbdaily.finance/path"));

        let result = evaluate(&input, vec![ProviderCheck::mock(ScanProvider::HashDit, ScanType::Website, Some(true))]);

        assert_eq!(result.scan.malicious_website.as_deref(), Some("https://bnbdaily.finance/path"));
        assert_eq!(result.new_verdicts[0].chain, None);
        assert_eq!(result.new_verdicts[0].target, "bnbdaily.finance");
    }

    #[test]
    fn test_dry_run_finding_is_logged_but_not_returned_or_stored() {
        let mut input = input(TransactionType::SmartContractCall, Some("https://example.com"));
        input.enforced.remove(&ScanType::Website);

        let result = evaluate(
            &input,
            vec![
                ProviderCheck::mock(ScanProvider::HashDit, ScanType::Address, Some(false)),
                ProviderCheck::mock(ScanProvider::HashDit, ScanType::Website, Some(true)),
            ],
        );

        assert_eq!(result.scan.is_malicious, Some(false));
        assert_eq!(result.scan.malicious_website, None);
        assert!(result.new_verdicts.is_empty());
        assert_eq!(result.dry_run().into_iter().collect::<Vec<_>>(), vec![ScanType::Website]);
    }

    #[test]
    fn test_scan_complete_counts_enforced_successes() {
        let mut input = input(TransactionType::Transfer, Some("https://example.com"));
        input.required_successes = 2;
        input.enforced.remove(&ScanType::Website);
        let checks = || {
            vec![
                ProviderCheck::mock(ScanProvider::HashDit, ScanType::Address, Some(false)),
                ProviderCheck::mock(ScanProvider::GoPlus, ScanType::Address, None),
                ProviderCheck::mock(ScanProvider::HashDit, ScanType::Website, Some(false)),
            ]
        };
        assert!(!evaluate(&input, checks()).scan.is_scan_complete);

        input.enforced.insert(ScanType::Website);
        assert!(evaluate(&input, checks()).scan.is_scan_complete);
    }

    #[test]
    fn test_local_result_is_complete() {
        let input = input(TransactionType::StakeDelegate, None);

        let result = evaluate(&input, vec![]);

        assert!(result.scan.is_scan_complete);
        assert_eq!(result.scan.is_malicious, Some(false));
        assert_eq!(result.source, ScanSource::Local);
    }

    #[test]
    fn test_errors_summary() {
        let input = input(TransactionType::Transfer, None);

        let result = evaluate(
            &input,
            vec![
                ProviderCheck::mock(ScanProvider::Tronscan, ScanType::Address, None),
                ProviderCheck::mock(ScanProvider::HashDit, ScanType::AddressPoisoning, Some(false)),
            ],
        );

        assert_eq!(result.errors(), "tronscan/address: timeout");
    }
}
