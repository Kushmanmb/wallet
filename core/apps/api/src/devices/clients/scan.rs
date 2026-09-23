use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use cacher::{AccessTokenCacherClient, CacherClient};
use config_keys::{ConfigKey, ConfigParamKey};
use gem_client::ReqwestClient;
use gem_tracing::{DurationMs, info_with_fields};
use primitives::{AssetId, ChainAddress, ScanMode, ScanProvider, ScanSource, ScanTransaction, ScanTransactionPayload, ScanType, TransactionType, asset_score::AssetRank};
use reqwest::Url;
use rocket::futures::future;
use security_provider::providers::goplus::GoPlusProvider;
use security_provider::{AddressPoisoningTarget, AddressScanProviderConfig, AddressTarget, ScanProviderFactory, ScanProviderRemoteConfig, ScanResult, TransactionScanProviders, WebsiteTarget};
use serde::Serialize;
use serde_json::json;
use settings::Settings;
use storage::models::NewScanDetectionRow;
use storage::{AssetsRepository, ConfigRepository, Database, ScanAddressesRepository, ScanDetectionsRepository};

use crate::metrics::Metrics;

pub fn scan_providers(settings: &Settings, cacher: CacherClient, timeout: Duration) -> Result<TransactionScanProviders, Box<dyn Error + Send + Sync>> {
    let config = AddressScanProviderConfig {
        timeout,
        goplus: ScanProviderRemoteConfig {
            url: settings.security.goplus.url.clone(),
            public_key: settings.security.goplus.key.public.clone(),
            secret_key: settings.security.goplus.key.secret.clone(),
        },
        hashdit: settings.security.hashdit.remote_provider_config(),
        tronscan: settings.security.tronscan.remote_provider_config(),
    };
    ScanProviderFactory::new_transaction_providers(config, Arc::new(AccessTokenCacherClient::new(cacher, GoPlusProvider::<ReqwestClient>::NAME)))
}

#[derive(Clone)]
pub struct TransactionScanConfig {
    pub providers: TransactionScanProviders,
    pub required_successes: usize,
}

#[derive(Serialize)]
struct ScanCheck {
    latency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    malicious: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl ScanCheck {
    fn new<T>(result: Result<ScanResult<T>, Box<dyn Error + Send + Sync>>, duration: Duration) -> Self {
        let (malicious, reason, error) = match result {
            Ok(result) => (Some(result.is_malicious), result.reason, None),
            Err(error) => (None, None, Some(error.to_string())),
        };
        Self {
            latency: DurationMs(duration).to_string(),
            malicious,
            reason,
            error,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ScanFinding {
    Address(ChainAddress),
    Asset(AssetId),
    Website(String),
}

#[derive(Debug, Clone, PartialEq)]
struct ScanDetection {
    scan_type: ScanType,
    finding: ScanFinding,
    is_enforced: bool,
}

impl ScanDetection {
    fn new(scan_type: ScanType, finding: ScanFinding, mode: ScanMode) -> Self {
        Self {
            scan_type,
            finding,
            is_enforced: mode == ScanMode::On,
        }
    }
}

struct LocalScan {
    detections: Vec<ScanDetection>,
    is_memo_required: bool,
    is_target_verified: bool,
}

struct ScanModes(HashMap<ScanType, ScanMode>);

impl ScanModes {
    fn get(&self, scan_type: ScanType) -> ScanMode {
        self.0.get(&scan_type).copied().unwrap_or(ScanMode::Off)
    }
}

#[derive(Clone)]
pub struct ScanClient {
    database: Database,
    config: TransactionScanConfig,
    metrics: Arc<Metrics>,
}

impl ScanClient {
    pub fn new(database: Database, config: TransactionScanConfig, metrics: Arc<Metrics>) -> Self {
        Self { database, config, metrics }
    }

    pub async fn get_scan_transaction(&self, payload: ScanTransactionPayload) -> Result<ScanTransaction, Box<dyn Error + Send + Sync>> {
        let modes = self.get_scan_modes()?;
        let website_host = payload.website.as_deref().and_then(Self::website_host);
        let local = self.get_scan_transaction_local(&payload, &modes)?;
        let cached = self.get_cached_detections(&payload, website_host.as_deref(), &modes, local.is_target_verified)?;
        let mut detections = [local.detections, cached.clone()].concat();
        if detections.iter().any(|detection| detection.is_enforced) || local.is_target_verified {
            let scan = Self::scan_transaction(&detections, local.is_memo_required, true);
            return Ok(Self::scan_transaction_response(&payload, scan, &detections, ScanSource::Local, &BTreeMap::new()));
        }

        let Some((address_target, poisoning_target, website_target)) = Self::provider_targets(&payload) else {
            let scan = Self::scan_transaction(&detections, local.is_memo_required, true);
            return Ok(Self::scan_transaction_response(&payload, scan, &detections, ScanSource::Local, &BTreeMap::new()));
        };
        let is_scanned = |scan_type: ScanType| modes.get(scan_type) != ScanMode::Off && !cached.iter().any(|detection| detection.scan_type == scan_type);
        let providers = self.config.providers.filter_enabled(&self.get_enabled_providers()?);
        let (address_scans, poisoning_scans, website_scans) = future::join3(
            self.scan_address_providers(&providers, is_scanned(ScanType::Address).then(|| address_target.clone())),
            self.scan_address_poisoning_providers(&providers, poisoning_target.filter(|_| is_scanned(ScanType::AddressPoisoning))),
            self.scan_website_providers(&providers, website_target.filter(|_| is_scanned(ScanType::Website))),
        )
        .await;
        let scans = [(ScanType::Address, &address_scans), (ScanType::AddressPoisoning, &poisoning_scans), (ScanType::Website, &website_scans)];

        let mut new_detections = Vec::new();
        for (scan_type, checks) in scans {
            let Some((provider, check)) = checks.iter().find(|(_, check)| check.malicious == Some(true)) else {
                continue;
            };
            let (finding, chain, target) = match scan_type {
                ScanType::Website => {
                    let (Some(website), Some(host)) = (payload.website.clone(), website_host.clone()) else {
                        continue;
                    };
                    (ScanFinding::Website(website), None, host)
                }
                _ => (
                    ScanFinding::Address(ChainAddress::new(address_target.chain, address_target.address.clone())),
                    Some(address_target.chain),
                    address_target.address.clone(),
                ),
            };
            let mode = modes.get(scan_type);
            if mode == ScanMode::On {
                new_detections.push(NewScanDetectionRow::new(scan_type, chain, target, *provider, check.reason.clone()));
            }
            detections.push(ScanDetection::new(scan_type, finding, mode));
        }
        self.database.scan_detections()?.add_scan_detections(new_detections)?;

        let completed_scans = scans
            .iter()
            .filter(|(scan_type, _)| modes.get(*scan_type) == ScanMode::On)
            .flat_map(|(_, checks)| checks.iter().map(|(_, check)| check.malicious.is_some()))
            .collect::<Vec<_>>();
        let scan = Self::scan_transaction(&detections, local.is_memo_required, Self::is_scan_complete(self.config.required_successes, &completed_scans));
        let source = if scans.iter().all(|(_, checks)| checks.is_empty()) { ScanSource::Local } else { ScanSource::Remote };
        let mut providers: BTreeMap<&str, BTreeMap<&str, &ScanCheck>> = BTreeMap::new();
        for (scan_type, checks) in &scans {
            for (provider, check) in checks.iter() {
                providers.entry(provider.as_ref()).or_default().insert(scan_type.as_ref(), check);
            }
        }
        Ok(Self::scan_transaction_response(&payload, scan, &detections, source, &providers))
    }

    fn get_scan_modes(&self) -> Result<ScanModes, Box<dyn Error + Send + Sync>> {
        let mut database = self.database.client()?;
        let mut modes = HashMap::new();
        for scan_type in ScanType::all() {
            modes.insert(scan_type, ScanMode::from_str(&database.get_config_param(ConfigParamKey::ScanTypeMode(scan_type))?)?);
        }
        Ok(ScanModes(modes))
    }

    fn get_enabled_providers(&self) -> Result<Vec<ScanProvider>, Box<dyn Error + Send + Sync>> {
        let mut database = self.database.client()?;
        let mut enabled = Vec::new();
        for provider in ScanProvider::all() {
            if database.get_config_param_bool(ConfigParamKey::ScanProviderEnable(provider))? {
                enabled.push(provider);
            }
        }
        Ok(enabled)
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

    fn scan_transaction_response(payload: &ScanTransactionPayload, scan: ScanTransaction, detections: &[ScanDetection], source: ScanSource, providers: &BTreeMap<&str, BTreeMap<&str, &ScanCheck>>) -> ScanTransaction {
        let logged_scan = ScanTransaction {
            malicious_website: scan.malicious_website.as_deref().and_then(Self::website_host),
            ..scan.clone()
        };
        let dry_run = detections.iter().filter(|detection| !detection.is_enforced).map(|detection| detection.scan_type).collect::<BTreeSet<_>>();
        let message = if scan.is_malicious == Some(true) { "security transaction blocked" } else { "security transaction result" };
        info_with_fields!(
            message,
            transaction_type = payload.transaction_type.as_ref(),
            chain = payload.target.asset_id.chain.as_ref(),
            source = source.as_ref(),
            malicious = scan.is_malicious == Some(true),
            dry_run_malicious = !dry_run.is_empty(),
            provider_errors = providers.values().flat_map(|checks| checks.values()).filter(|check| check.error.is_some()).count(),
            origin_asset_id = payload.origin.asset_id,
            target_asset_id = payload.target.asset_id,
            address = format!("{:?}", payload.target.address),
            website_host = json!(payload.website.as_deref().and_then(Self::website_host)),
            scan = json!(logged_scan),
            dry_run = json!(dry_run),
            providers = json!(providers)
        );
        scan
    }

    fn website_host(website: &str) -> Option<String> {
        Url::parse(website).ok()?.host_str().map(str::to_string)
    }

    fn get_scan_transaction_local(&self, payload: &ScanTransactionPayload, modes: &ScanModes) -> Result<LocalScan, Box<dyn Error + Send + Sync>> {
        let queries = [(payload.origin.asset_id.chain, payload.origin.address.as_str()), (payload.target.asset_id.chain, payload.target.address.as_str())];
        let addresses = self.database.scan_addresses()?.get_scan_addresses(&queries)?;
        let mut detections = addresses
            .iter()
            .filter(|address| address.is_fraudulent)
            .map(|address| ScanDetection::new(ScanType::Address, ScanFinding::Address(ChainAddress::new(address.chain.0, address.address.clone())), ScanMode::On))
            .collect::<Vec<_>>();
        let asset_mode = modes.get(ScanType::Asset);
        if asset_mode != ScanMode::Off {
            let token_assets = self.database.assets()?.get_assets_basic(Self::token_asset_ids(payload))?;
            detections.extend(
                token_assets
                    .into_iter()
                    .filter(|asset| Self::is_malicious_asset_rank(asset.score.rank))
                    .map(|asset| ScanDetection::new(ScanType::Asset, ScanFinding::Asset(asset.asset.id), asset_mode)),
            );
        }

        Ok(LocalScan {
            detections,
            is_memo_required: addresses.iter().any(|address| address.is_memo_required),
            is_target_verified: addresses.iter().any(|address| address.is_verified_for(payload.target.asset_id.chain, &payload.target.address)),
        })
    }

    fn get_cached_detections(&self, payload: &ScanTransactionPayload, website_host: Option<&str>, modes: &ScanModes, is_target_verified: bool) -> Result<Vec<ScanDetection>, Box<dyn Error + Send + Sync>> {
        let chain = payload.target.asset_id.chain;
        let address = payload.target.address.as_str();
        let address_types = if is_target_verified { vec![] } else { vec![ScanType::Address, ScanType::AddressPoisoning] };
        let targets = address_types
            .into_iter()
            .map(|scan_type| (scan_type, Some(chain), address))
            .chain(website_host.map(|host| (ScanType::Website, None, host)))
            .filter(|(scan_type, _, _)| modes.get(*scan_type) != ScanMode::Off)
            .collect::<Vec<_>>();
        if targets.is_empty() {
            return Ok(Vec::new());
        }
        let max_age = self.database.client()?.get_config_duration(ConfigKey::ScanDetectionMaxAge)?;
        let rows = self.database.scan_detections()?.get_scan_detections(targets.iter().map(|(_, _, target)| target.to_string()).collect(), max_age)?;

        Ok(targets
            .into_iter()
            .filter(|(scan_type, chain, target)| rows.iter().any(|row| row.matches(*scan_type, *chain, target)))
            .filter_map(|(scan_type, _, _)| {
                let finding = match scan_type {
                    ScanType::Website => ScanFinding::Website(payload.website.clone()?),
                    _ => ScanFinding::Address(ChainAddress::new(chain, address.to_string())),
                };
                Some(ScanDetection::new(scan_type, finding, modes.get(scan_type)))
            })
            .collect())
    }

    fn provider_targets(payload: &ScanTransactionPayload) -> Option<(AddressTarget, Option<AddressPoisoningTarget>, Option<WebsiteTarget>)> {
        let address = AddressTarget {
            chain: payload.target.asset_id.chain,
            address: payload.target.address.clone(),
        };
        let poisoning = match payload.transaction_type {
            TransactionType::Transfer | TransactionType::TransferNFT => Some(AddressPoisoningTarget {
                target: address.clone(),
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
        Some((address, poisoning, website))
    }

    fn is_scan_complete(required_successes: usize, scans: &[bool]) -> bool {
        scans.iter().filter(|is_complete| **is_complete).count() >= required_successes
    }

    fn is_malicious_asset_rank(rank: i32) -> bool {
        rank <= AssetRank::Spam.threshold()
    }

    fn token_asset_ids(payload: &ScanTransactionPayload) -> Vec<AssetId> {
        let mut targets = Vec::new();
        for asset_id in [&payload.origin.asset_id, &payload.target.asset_id] {
            if asset_id.is_native() {
                continue;
            }
            if !targets.contains(asset_id) {
                targets.push(asset_id.clone());
            }
        }
        targets
    }

    async fn scan_address_providers(&self, providers: &TransactionScanProviders, target: Option<AddressTarget>) -> Vec<(ScanProvider, ScanCheck)> {
        let Some(target) = target else {
            return Vec::new();
        };
        future::join_all(providers.addresses.iter().filter(|provider| provider.supports_chain(target.chain)).map(|provider| async {
            let start = Instant::now();
            let result = provider.scan_address(&target).await;
            let latency = start.elapsed();
            self.metrics.record_scan(provider.provider(), "address", result.as_ref().ok().map(|scan| scan.is_malicious), latency);
            (provider.provider(), ScanCheck::new(result, latency))
        }))
        .await
    }

    async fn scan_address_poisoning_providers(&self, providers: &TransactionScanProviders, target: Option<AddressPoisoningTarget>) -> Vec<(ScanProvider, ScanCheck)> {
        let Some(target) = target else {
            return Vec::new();
        };
        future::join_all(providers.poisoning.iter().filter(|provider| provider.supports_chain(target.target.chain)).map(|provider| async {
            let start = Instant::now();
            let result = provider.scan_address_poisoning(&target).await;
            let latency = start.elapsed();
            self.metrics.record_scan(provider.provider(), "address_poisoning", result.as_ref().ok().map(|scan| scan.is_malicious), latency);
            (provider.provider(), ScanCheck::new(result, latency))
        }))
        .await
    }

    async fn scan_website_providers(&self, providers: &TransactionScanProviders, target: Option<WebsiteTarget>) -> Vec<(ScanProvider, ScanCheck)> {
        let Some(target) = target else {
            return Vec::new();
        };
        future::join_all(providers.websites.iter().map(|provider| async {
            let start = Instant::now();
            let result = provider.scan_website(&target).await;
            let latency = start.elapsed();
            self.metrics.record_scan(provider.provider(), "website", result.as_ref().ok().map(|scan| scan.is_malicious), latency);
            (provider.provider(), ScanCheck::new(result, latency))
        }))
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::{AssetId, Chain};

    #[test]
    fn test_website_host_excludes_credentials_and_query_values() {
        assert_eq!(ScanClient::website_host("https://user:password@example.com/path?token=secret#fragment"), Some("example.com".into()));
        assert_eq!(ScanClient::website_host("invalid website"), None);
    }

    #[test]
    fn test_scan_transaction_excludes_dry_run_detections() {
        let address = ChainAddress::new(Chain::SmartChain, "0x123".to_string());
        let detections = vec![
            ScanDetection::new(ScanType::Address, ScanFinding::Address(address.clone()), ScanMode::On),
            ScanDetection::new(ScanType::AddressPoisoning, ScanFinding::Address(address.clone()), ScanMode::On),
            ScanDetection::new(ScanType::Website, ScanFinding::Website("https://example.com".to_string()), ScanMode::DryRun),
            ScanDetection::new(ScanType::Asset, ScanFinding::Asset(AssetId::from_token(Chain::SmartChain, "0x456")), ScanMode::DryRun),
        ];

        let scan = ScanClient::scan_transaction(&detections, true, true);

        assert_eq!(scan.is_malicious, Some(true));
        assert_eq!(scan.is_memo_required, Some(true));
        assert_eq!(scan.malicious_addresses, Some(vec![address]));
        assert_eq!(scan.malicious_assets, Some(vec![]));
        assert_eq!(scan.malicious_website, None);
    }

    #[test]
    fn test_scan_transaction_dry_run_only_is_not_malicious() {
        let detections = vec![ScanDetection::new(ScanType::Website, ScanFinding::Website("https://example.com".to_string()), ScanMode::DryRun)];

        let scan = ScanClient::scan_transaction(&detections, false, true);

        assert_eq!(scan.is_malicious, Some(false));
        assert_eq!(scan.malicious_addresses, Some(vec![]));
        assert_eq!(scan.malicious_website, None);
    }

    #[test]
    fn test_scan_modes_default_to_off() {
        let modes = ScanModes(HashMap::from([(ScanType::Website, ScanMode::DryRun)]));

        assert_eq!(modes.get(ScanType::Website), ScanMode::DryRun);
        assert_eq!(modes.get(ScanType::Address), ScanMode::Off);
    }

    #[test]
    fn test_scan_complete_requires_configured_success_count() {
        assert!(ScanClient::is_scan_complete(1, &[true, false, false]));
        assert!(ScanClient::is_scan_complete(1, &[false, true, false]));
        assert!(ScanClient::is_scan_complete(1, &[false, false, true]));
        assert!(ScanClient::is_scan_complete(2, &[true, false, true]));
        assert!(ScanClient::is_scan_complete(2, &[true, true, true]));
        assert!(!ScanClient::is_scan_complete(2, &[true, false, false]));
        assert!(!ScanClient::is_scan_complete(3, &[true, true]));
        assert!(!ScanClient::is_scan_complete(1, &[false, false]));
        assert!(!ScanClient::is_scan_complete(1, &[]));
        assert!(ScanClient::is_scan_complete(0, &[false, false]));
        assert!(ScanClient::is_scan_complete(0, &[true]));
        assert!(ScanClient::is_scan_complete(0, &[]));
    }

    #[test]
    fn test_spam_or_lower_asset_rank_is_malicious() {
        assert!(!ScanClient::is_malicious_asset_rank(-14));
        assert!(ScanClient::is_malicious_asset_rank(-15));
        assert!(ScanClient::is_malicious_asset_rank(-20));
        assert!(ScanClient::is_malicious_asset_rank(i32::MIN));
    }

    #[test]
    fn test_native_assets_do_not_create_token_asset_ids() {
        let payload = ScanTransactionPayload::mock_with_assets(AssetId::from_chain(Chain::Ethereum), AssetId::from_chain(Chain::Ethereum));

        assert!(ScanClient::token_asset_ids(&payload).is_empty());
    }

    #[test]
    fn test_same_token_is_looked_up_once() {
        let token = AssetId::from_token(Chain::SmartChain, "0x123");
        let payload = ScanTransactionPayload::mock_with_assets(token.clone(), token);

        assert_eq!(ScanClient::token_asset_ids(&payload).len(), 1);
    }

    #[test]
    fn test_swap_looks_up_both_distinct_token_assets() {
        let payload = ScanTransactionPayload {
            transaction_type: TransactionType::Swap,
            ..ScanTransactionPayload::mock_with_assets(AssetId::from_token(Chain::Ethereum, "0x123"), AssetId::from_token(Chain::SmartChain, "0x456"))
        };

        assert_eq!(ScanClient::token_asset_ids(&payload), vec![AssetId::from_token(Chain::Ethereum, "0x123"), AssetId::from_token(Chain::SmartChain, "0x456"),]);
    }

    #[test]
    fn test_provider_targets_use_recipient_context() {
        let payload = ScanTransactionPayload {
            website: Some("https://example.com".to_string()),
            ..ScanTransactionPayload::mock_with_assets(AssetId::from_chain(Chain::SmartChain), AssetId::from_token(Chain::SmartChain, "0x456"))
        };

        assert_eq!(
            ScanClient::provider_targets(&payload).unwrap(),
            (
                AddressTarget {
                    chain: Chain::SmartChain,
                    address: "target".to_string(),
                },
                Some(AddressPoisoningTarget {
                    target: AddressTarget {
                        chain: Chain::SmartChain,
                        address: "target".to_string(),
                    },
                    user_address: "origin".to_string(),
                }),
                Some(WebsiteTarget { website: "https://example.com".to_string() }),
            )
        );
    }

    #[test]
    fn test_provider_targets_skip_staking() {
        for transaction_type in [
            TransactionType::StakeDelegate,
            TransactionType::StakeUndelegate,
            TransactionType::StakeRewards,
            TransactionType::StakeRedelegate,
            TransactionType::StakeWithdraw,
            TransactionType::StakeFreeze,
            TransactionType::StakeUnfreeze,
        ] {
            let payload = ScanTransactionPayload {
                website: Some("https://example.com".to_string()),
                transaction_type,
                ..ScanTransactionPayload::mock_with_assets(AssetId::from_chain(Chain::Monad), AssetId::from_chain(Chain::Monad))
            };

            assert_eq!(ScanClient::provider_targets(&payload), None);
        }
    }

    #[test]
    fn test_provider_targets_skip_poisoning_for_contract_calls() {
        let payload = ScanTransactionPayload {
            transaction_type: TransactionType::Swap,
            ..ScanTransactionPayload::mock_with_assets(AssetId::from_chain(Chain::Ethereum), AssetId::from_token(Chain::SmartChain, "0x456"))
        };

        assert_eq!(ScanClient::provider_targets(&payload).unwrap().1, None);
    }
}
