use std::collections::HashSet;

use primitives::asset_score::AssetRank;
use primitives::{AssetId, ChainAddress, ScanTransactionPayload, ScanType, TransactionType};
use url::Url;

use super::model::{ScanDetection, ScanFinding, ScanPlan, ScanTargets, TransactionScanInput};
use crate::{AddressPoisoningTarget, AddressTarget, WebsiteTarget};

pub fn plan_transaction_scan(input: &TransactionScanInput) -> ScanPlan {
    let payload = &input.payload;
    let is_target_verified = input.addresses.iter().any(|address| address.is_verified_for(payload.target.asset_id.chain, &payload.target.address));
    let cached = cached_detections(input, is_target_verified);
    let mut detections = local_detections(input);
    detections.extend(cached.iter().cloned());
    let is_scanned = |scan_type: ScanType| {
        let is_verified_address = is_target_verified && matches!(scan_type, ScanType::Address | ScanType::AddressPoisoning);
        !is_verified_address && !cached.iter().any(|detection| detection.scan_type == scan_type)
    };
    let is_resolved = detections.iter().any(|detection| detection.is_enforced) || (is_target_verified && website_host(payload).is_none());
    let mut safe = Vec::new();
    let targets = if is_resolved {
        None
    } else {
        provider_targets(payload).map(|targets| ScanTargets {
            address: select_target(targets.address, ScanType::Address, is_scanned(ScanType::Address), &input.safe, &mut safe),
            poisoning: select_target(targets.poisoning, ScanType::AddressPoisoning, is_scanned(ScanType::AddressPoisoning), &input.safe, &mut safe),
            website: select_target(targets.website, ScanType::Website, is_scanned(ScanType::Website), &input.safe, &mut safe),
        })
    };

    ScanPlan {
        detections,
        is_memo_required: input.addresses.iter().any(|address| address.is_memo_required == Some(true)),
        targets,
        safe,
    }
}

pub fn safe_cache_targets(payload: &ScanTransactionPayload) -> Vec<(ScanType, String)> {
    let address = (!payload.target.address.is_empty()).then(|| (ScanType::Address, format!("{}:{}", payload.target.asset_id.chain.as_ref(), payload.target.address)));
    address.into_iter().chain(website_host(payload).map(|host| (ScanType::Website, host))).collect()
}

pub fn detection_targets(payload: &ScanTransactionPayload) -> Vec<String> {
    (!payload.target.address.is_empty()).then(|| payload.target.address.clone()).into_iter().chain(website_host(payload)).collect()
}

pub fn website_host(payload: &ScanTransactionPayload) -> Option<String> {
    Url::parse(payload.website.as_deref()?).ok()?.host_str().map(str::to_string)
}

pub fn token_asset_ids(payload: &ScanTransactionPayload) -> Vec<AssetId> {
    let mut targets = Vec::new();
    for asset_id in [&payload.origin.asset_id, &payload.target.asset_id] {
        if !asset_id.is_native() && !targets.contains(asset_id) {
            targets.push(asset_id.clone());
        }
    }
    targets
}

fn select_target<T>(target: Option<T>, scan_type: ScanType, is_scanned: bool, cached_safe: &HashSet<ScanType>, safe: &mut Vec<ScanType>) -> Option<T> {
    let target = target.filter(|_| is_scanned)?;
    if cached_safe.contains(&scan_type) {
        safe.push(scan_type);
        return None;
    }
    Some(target)
}

fn local_detections(input: &TransactionScanInput) -> Vec<ScanDetection> {
    let addresses = input
        .addresses
        .iter()
        .filter(|address| address.is_malicious == Some(true))
        .map(|address| ScanDetection::new(ScanType::Address, ScanFinding::Address(ChainAddress::new(address.chain, address.address.clone())), true, "manual"));
    let is_asset_enforced = input.enforced.contains(&ScanType::Asset);
    let assets = input
        .assets
        .iter()
        .filter(|asset| asset.score.rank <= AssetRank::Spam.threshold())
        .map(|asset| ScanDetection::new(ScanType::Asset, ScanFinding::Asset(asset.asset.id.clone()), is_asset_enforced, asset.asset.id.to_string()));
    addresses.chain(assets).collect()
}

fn cached_detections(input: &TransactionScanInput, is_target_verified: bool) -> Vec<ScanDetection> {
    let payload = &input.payload;
    let chain = payload.target.asset_id.chain;
    let address = payload.target.address.as_str();
    let address_types = if is_target_verified || address.is_empty() { vec![] } else { vec![ScanType::Address, ScanType::AddressPoisoning] };
    let website_host = website_host(payload);
    let targets = address_types
        .into_iter()
        .map(|scan_type| (scan_type, Some(chain), address))
        .chain(website_host.as_deref().map(|host| (ScanType::Website, None, host)));

    targets
        .filter_map(|(scan_type, target_chain, target)| {
            let verdict = input.verdicts.iter().find(|verdict| verdict.matches(scan_type, target_chain, target))?;
            let finding = match scan_type {
                ScanType::Website => ScanFinding::Website(payload.website.clone()?),
                _ => ScanFinding::Address(ChainAddress::new(chain, address.to_string())),
            };
            let source = format!("cached {}", ScanDetection::provider_source(verdict.provider, verdict.reason.as_deref()));
            Some(ScanDetection::new(scan_type, finding, input.enforced.contains(&scan_type), source))
        })
        .collect()
}

fn provider_targets(payload: &ScanTransactionPayload) -> Option<ScanTargets> {
    let address = (!payload.target.address.is_empty()).then(|| AddressTarget {
        chain: payload.target.asset_id.chain,
        address: payload.target.address.clone(),
    });
    let poisoning = match payload.transaction_type {
        TransactionType::Transfer | TransactionType::TransferNFT => address.clone().map(|target| AddressPoisoningTarget {
            target,
            user_address: payload.origin.address.clone(),
        }),
        TransactionType::StakeDelegate
        | TransactionType::StakeUndelegate
        | TransactionType::StakeRewards
        | TransactionType::StakeRedelegate
        | TransactionType::StakeWithdraw
        | TransactionType::StakeFreeze
        | TransactionType::StakeUnfreeze => return None,
        TransactionType::Swap
        | TransactionType::TokenApproval
        | TransactionType::AssetActivation
        | TransactionType::SmartContractCall
        | TransactionType::PerpetualOpenPosition
        | TransactionType::PerpetualClosePosition
        | TransactionType::PerpetualModifyPosition
        | TransactionType::EarnDeposit
        | TransactionType::EarnWithdraw => None,
    };
    let website = payload.website.clone().map(|website| WebsiteTarget { website });
    Some(ScanTargets { address, poisoning, website })
}

#[cfg(test)]
mod tests {
    use primitives::asset_score::AssetRank;
    use primitives::{AssetBasic, Chain, ScanAddress, ScanProvider, ScanVerdict};

    use super::*;

    fn payload(transaction_type: TransactionType, website: Option<&str>) -> ScanTransactionPayload {
        ScanTransactionPayload {
            transaction_type,
            website: website.map(str::to_string),
            ..ScanTransactionPayload::mock_with_assets(AssetId::from_chain(Chain::SmartChain), AssetId::from_chain(Chain::SmartChain))
        }
    }

    fn verified(address: &str) -> ScanAddress {
        ScanAddress::contract(Chain::SmartChain, address, "Verified")
    }

    fn verdict(scan_type: ScanType, chain: Option<Chain>, target: &str) -> ScanVerdict {
        ScanVerdict {
            scan_type,
            chain,
            target: target.to_string(),
            provider: ScanProvider::HashDit,
            reason: Some("phishing".to_string()),
        }
    }

    fn spam_asset(asset_id: AssetId) -> AssetBasic {
        let mut asset = AssetBasic::mock_with_price(Chain::SmartChain, 1.0, 0.0);
        asset.asset.id = asset_id;
        asset.score.rank = AssetRank::Spam.threshold();
        asset
    }

    #[test]
    fn test_website_host_excludes_credentials_and_query_values() {
        assert_eq!(
            website_host(&payload(TransactionType::SmartContractCall, Some("https://user:password@example.com/path?token=secret#fragment"))),
            Some("example.com".into())
        );
        assert_eq!(website_host(&payload(TransactionType::SmartContractCall, Some("invalid website"))), None);
    }

    #[test]
    fn test_token_asset_ids() {
        let token = AssetId::from_token(Chain::SmartChain, "0x123");
        let other = AssetId::from_token(Chain::Ethereum, "0x456");

        assert!(token_asset_ids(&ScanTransactionPayload::mock_with_assets(AssetId::from_chain(Chain::Ethereum), AssetId::from_chain(Chain::Ethereum))).is_empty());
        assert_eq!(token_asset_ids(&ScanTransactionPayload::mock_with_assets(token.clone(), token.clone())), vec![token.clone()]);
        assert_eq!(token_asset_ids(&ScanTransactionPayload::mock_with_assets(token.clone(), other.clone())), vec![token, other]);
    }

    #[test]
    fn test_detection_targets() {
        assert_eq!(detection_targets(&payload(TransactionType::Transfer, Some("https://example.com/path"))), vec!["target", "example.com"]);

        let mut payload = payload(TransactionType::SmartContractCall, None);
        payload.target.address = String::new();
        assert!(detection_targets(&payload).is_empty());
    }

    #[test]
    fn test_safe_cache_targets() {
        assert_eq!(
            safe_cache_targets(&payload(TransactionType::Transfer, Some("https://example.com/path"))),
            vec![(ScanType::Address, "smartchain:target".to_string()), (ScanType::Website, "example.com".to_string())]
        );
    }

    #[test]
    fn test_plan_skips_cached_safe_types() {
        let mut input = TransactionScanInput::mock(payload(TransactionType::Transfer, Some("https://example.com")));
        input.safe = HashSet::from([ScanType::Address]);

        let plan = plan_transaction_scan(&input);
        let targets = plan.targets.unwrap();

        assert_eq!(plan.safe, vec![ScanType::Address]);
        assert_eq!(targets.address, None);
        assert!(targets.poisoning.is_some());
        assert!(targets.website.is_some());
    }

    #[test]
    fn test_plan_verified_target_ignores_cached_safe_address() {
        let mut input = TransactionScanInput::mock(payload(TransactionType::SmartContractCall, Some("https://example.com")));
        input.addresses = vec![verified("target")];
        input.safe = HashSet::from([ScanType::Address]);

        assert!(plan_transaction_scan(&input).safe.is_empty());
    }

    #[test]
    fn test_plan_scans_recipient_targets() {
        let plan = plan_transaction_scan(&TransactionScanInput::mock(payload(TransactionType::Transfer, Some("https://example.com"))));
        let address = AddressTarget {
            chain: Chain::SmartChain,
            address: "target".to_string(),
        };

        assert_eq!(
            plan.targets,
            Some(ScanTargets {
                address: Some(address.clone()),
                poisoning: Some(AddressPoisoningTarget {
                    target: address,
                    user_address: "origin".to_string(),
                }),
                website: Some(WebsiteTarget { website: "https://example.com".to_string() }),
            })
        );
    }

    #[test]
    fn test_plan_skips_poisoning_for_contract_calls() {
        let targets = plan_transaction_scan(&TransactionScanInput::mock(payload(TransactionType::Swap, None))).targets.unwrap();

        assert!(targets.address.is_some());
        assert!(targets.poisoning.is_none());
    }

    #[test]
    fn test_plan_skips_empty_address() {
        let mut payload = payload(TransactionType::SmartContractCall, Some("https://example.com"));
        payload.target.address = String::new();

        let targets = plan_transaction_scan(&TransactionScanInput::mock(payload)).targets.unwrap();

        assert_eq!(targets.address, None);
        assert_eq!(targets.poisoning, None);
        assert!(targets.website.is_some());
    }

    #[test]
    fn test_plan_skips_staking() {
        let plan = plan_transaction_scan(&TransactionScanInput::mock(payload(TransactionType::StakeDelegate, Some("https://example.com"))));

        assert_eq!(plan.targets, None);
    }

    #[test]
    fn test_plan_verified_target_without_website_is_resolved() {
        let mut input = TransactionScanInput::mock(payload(TransactionType::Transfer, None));
        input.addresses = vec![verified("target")];

        assert_eq!(plan_transaction_scan(&input).targets, None);
    }

    #[test]
    fn test_plan_verified_target_still_scans_website() {
        let mut input = TransactionScanInput::mock(payload(TransactionType::SmartContractCall, Some("https://bnbdaily.finance/")));
        input.addresses = vec![verified("target")];

        let targets = plan_transaction_scan(&input).targets.unwrap();

        assert_eq!(targets.address, None);
        assert_eq!(targets.poisoning, None);
        assert_eq!(
            targets.website,
            Some(WebsiteTarget {
                website: "https://bnbdaily.finance/".to_string()
            })
        );
    }

    #[test]
    fn test_plan_flagged_address_is_resolved() {
        let mut flagged = verified("target");
        flagged.is_malicious = Some(true);
        let mut input = TransactionScanInput::mock(payload(TransactionType::Transfer, None));
        input.addresses = vec![flagged];

        let plan = plan_transaction_scan(&input);

        assert_eq!(plan.targets, None);
        assert_eq!(plan.detections[0].source, "manual");
    }

    #[test]
    fn test_plan_enforced_verdict_is_resolved() {
        let mut input = TransactionScanInput::mock(payload(TransactionType::Transfer, None));
        input.verdicts = vec![verdict(ScanType::Address, Some(Chain::SmartChain), "target")];

        let plan = plan_transaction_scan(&input);

        assert_eq!(plan.targets, None);
        assert_eq!(plan.detections[0].source, "cached hashdit: phishing");
    }

    #[test]
    fn test_plan_ignores_verdict_for_other_chain() {
        let mut input = TransactionScanInput::mock(payload(TransactionType::Transfer, None));
        input.verdicts = vec![verdict(ScanType::Address, Some(Chain::Ethereum), "target")];

        let plan = plan_transaction_scan(&input);

        assert!(plan.detections.is_empty());
        assert!(plan.targets.unwrap().address.is_some());
    }

    #[test]
    fn test_plan_dry_run_verdict_skips_only_its_type() {
        let mut input = TransactionScanInput::mock(payload(TransactionType::Transfer, Some("https://example.com")));
        input.enforced.remove(&ScanType::Website);
        input.verdicts = vec![verdict(ScanType::Website, None, "example.com")];

        let plan = plan_transaction_scan(&input);
        let targets = plan.targets.unwrap();

        assert!(!plan.detections[0].is_enforced);
        assert_eq!(targets.website, None);
        assert!(targets.address.is_some());
    }

    #[test]
    fn test_plan_spam_asset() {
        let token = AssetId::from_token(Chain::SmartChain, "0x123");
        let mut input = TransactionScanInput::mock(ScanTransactionPayload::mock_with_assets(token.clone(), token.clone()));
        input.assets = vec![spam_asset(token)];
        assert_eq!(plan_transaction_scan(&input).targets, None);

        input.enforced.remove(&ScanType::Asset);
        let plan = plan_transaction_scan(&input);
        assert!(!plan.detections[0].is_enforced);
        assert!(plan.targets.is_some());
    }
}
