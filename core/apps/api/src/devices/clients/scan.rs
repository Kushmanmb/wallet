use std::collections::HashSet;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use cacher::{AccessTokenCacherClient, CacheKey, CacherClient};
use config_keys::{ConfigKey, ConfigParamKey};
use gem_client::ReqwestClient;
use gem_tracing::info_with_fields;
use primitives::{ScanProvider, ScanTransaction, ScanTransactionPayload, ScanType};
use rocket::futures::future;
use security_provider::providers::goplus::GoPlusProvider;
use security_provider::transaction_scan::{ProviderCheck, ScanSubject, ScanTargets, TransactionScanInput, TransactionScanResult, evaluate_transaction_scan, plan_transaction_scan, scan_subjects, token_asset_ids, website_host};
use security_provider::{AddressScanProviderConfig, ScanProviderFactory, ScanProviderRemoteConfig, ScanResult, TransactionScanProviders};
use serde_json::json;
use settings::Settings;
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

struct SafeCacheKey {
    scan_type: ScanType,
    target: String,
    ttl: u64,
}

impl SafeCacheKey {
    fn key(&self) -> CacheKey<'_> {
        CacheKey::ScanSafe(self.scan_type.as_ref(), &self.target, self.ttl)
    }
}

#[derive(Clone)]
pub struct ScanClient {
    database: Database,
    cacher: CacherClient,
    config: TransactionScanConfig,
    metrics: Arc<Metrics>,
}

impl ScanClient {
    pub fn new(database: Database, cacher: CacherClient, config: TransactionScanConfig, metrics: Arc<Metrics>) -> Self {
        Self { database, cacher, config, metrics }
    }

    pub async fn get_scan_transaction(&self, payload: ScanTransactionPayload) -> Result<ScanTransaction, Box<dyn Error + Send + Sync>> {
        let subjects = scan_subjects(&payload);
        let safe_keys = self.get_safe_cache_keys(&subjects)?;
        let safe = self.get_cached_safe(&safe_keys).await?;
        let input = self.get_scan_input(payload, &subjects, safe)?;
        let plan = plan_transaction_scan(&input);
        let checks = match &plan.targets {
            Some(targets) => self.run_checks(targets).await?,
            None => Vec::new(),
        };
        let result = evaluate_transaction_scan(&input, plan, checks);
        self.database.scan_detections()?.add_scan_detections(result.new_verdicts.clone())?;
        for key in safe_keys.iter().filter(|key| result.new_safe.contains(&key.scan_type)) {
            self.cacher.set_cached(key.key(), &true).await?;
        }
        Self::log(&input.payload, &result);
        Ok(result.scan)
    }

    fn get_scan_input(&self, payload: ScanTransactionPayload, subjects: &[ScanSubject], safe: HashSet<ScanType>) -> Result<TransactionScanInput, Box<dyn Error + Send + Sync>> {
        let mut database = self.database.client()?;
        let mut enforced = HashSet::new();
        for scan_type in ScanType::all() {
            if database.get_config_param_bool(ConfigParamKey::ScanTypeEnable(scan_type))? {
                enforced.insert(scan_type);
            }
        }
        let queries = [(payload.origin.asset_id.chain, payload.origin.address.as_str()), (payload.target.asset_id.chain, payload.target.address.as_str())];
        let addresses = database.get_scan_addresses(&queries)?.iter().map(|address| address.as_scan_address()).collect();
        let assets = database.get_assets_basic(token_asset_ids(&payload))?;
        let mut targets = subjects.iter().map(|subject| subject.target.clone()).collect::<Vec<_>>();
        targets.dedup();
        let verdicts = if targets.is_empty() {
            Vec::new()
        } else {
            let max_age = database.get_config_duration(ConfigKey::ScanDetectionMaxAge)?;
            database.get_scan_detections(targets, max_age)?
        };
        Ok(TransactionScanInput {
            payload,
            enforced,
            addresses,
            assets,
            verdicts,
            safe,
            required_successes: self.config.required_successes,
        })
    }

    fn get_safe_cache_keys(&self, subjects: &[ScanSubject]) -> Result<Vec<SafeCacheKey>, Box<dyn Error + Send + Sync>> {
        let mut database = self.database.client()?;
        let mut keys = Vec::new();
        for subject in subjects.iter().filter(|subject| subject.scan_type.is_safe_cacheable()) {
            let ttl = database.get_config_param_duration(ConfigParamKey::ScanSafeCacheDuration(subject.scan_type))?.as_secs();
            if ttl > 0 {
                keys.push(SafeCacheKey {
                    scan_type: subject.scan_type,
                    target: subject.cache_key(),
                    ttl,
                });
            }
        }
        Ok(keys)
    }

    async fn get_cached_safe(&self, keys: &[SafeCacheKey]) -> Result<HashSet<ScanType>, Box<dyn Error + Send + Sync>> {
        let mut safe = HashSet::new();
        for key in keys {
            if self.cacher.get_cached_optional::<bool>(key.key()).await?.is_some() {
                safe.insert(key.scan_type);
            }
        }
        Ok(safe)
    }

    async fn run_checks(&self, targets: &ScanTargets) -> Result<Vec<ProviderCheck>, Box<dyn Error + Send + Sync>> {
        let mut enabled = Vec::new();
        let mut database = self.database.client()?;
        for provider in ScanProvider::all() {
            if database.get_config_param_bool(ConfigParamKey::ScanProviderEnable(provider))? {
                enabled.push(provider);
            }
        }
        let providers = self.config.providers.filter_enabled(&enabled);
        let (addresses, poisoning, websites) = future::join3(
            future::join_all(targets.address.iter().flat_map(|target| {
                providers
                    .addresses
                    .iter()
                    .filter(|provider| provider.supports_chain(target.chain))
                    .map(move |provider| self.run_check(provider.provider(), ScanType::Address, provider.scan_address(target)))
            })),
            future::join_all(targets.poisoning.iter().flat_map(|target| {
                providers
                    .poisoning
                    .iter()
                    .filter(|provider| provider.supports_chain(target.target.chain))
                    .map(move |provider| self.run_check(provider.provider(), ScanType::AddressPoisoning, provider.scan_address_poisoning(target)))
            })),
            future::join_all(
                targets
                    .website
                    .iter()
                    .flat_map(|target| providers.websites.iter().map(move |provider| self.run_check(provider.provider(), ScanType::Website, provider.scan_website(target)))),
            ),
        )
        .await;
        Ok([addresses, poisoning, websites].concat())
    }

    async fn run_check<T>(&self, provider: ScanProvider, scan_type: ScanType, request: impl Future<Output = Result<ScanResult<T>, Box<dyn Error + Send + Sync>>>) -> ProviderCheck {
        let start = Instant::now();
        let result = request.await;
        let latency = start.elapsed();
        self.metrics.record_scan(provider, scan_type, result.as_ref().ok().map(|scan| scan.is_malicious), latency);
        ProviderCheck::new(provider, scan_type, result, latency)
    }

    fn log(payload: &ScanTransactionPayload, result: &TransactionScanResult) {
        let website_host = website_host(payload);
        let target = if payload.target.address.is_empty() {
            website_host.clone().unwrap_or_default()
        } else {
            payload.target.address.clone()
        };
        let scan = ScanTransaction {
            malicious_website: result.scan.malicious_website.as_ref().and(website_host.clone()),
            ..result.scan.clone()
        };
        let message = if result.scan.is_malicious == Some(true) { "security transaction blocked" } else { "security transaction result" };
        let dry_run = result.dry_run();
        info_with_fields!(
            message,
            transaction_type = payload.transaction_type.as_ref(),
            chain = payload.target.asset_id.chain.as_ref(),
            source = result.source.as_ref(),
            target = format!("{target:?}"),
            findings = format!("{:?}", result.findings()),
            errors = format!("{:?}", result.errors()),
            malicious = result.scan.is_malicious == Some(true),
            dry_run_malicious = !dry_run.is_empty(),
            provider_errors = result.checks.iter().filter(|check| check.error.is_some()).count(),
            origin_asset_id = payload.origin.asset_id,
            target_asset_id = payload.target.asset_id,
            address = format!("{:?}", payload.target.address),
            website_host = json!(website_host),
            scan = json!(scan),
            dry_run = json!(dry_run),
            cached_safe = json!(result.safe),
            providers = json!(result.providers())
        );
    }
}
