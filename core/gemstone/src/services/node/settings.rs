use std::sync::Arc;

use primitives::{Chain, Latency};

use super::model::{GemAddNodeError, GemChainSettingsSection, GemExplorerRow, GemNodeCheck, GemNodeRow, GemNodeSelection, GemNodeStatusState};
use super::rules;
use super::session::GemAddNodeSession;
use crate::gateway::GemGateway;
use crate::services::chain::rules as chain_rules;
use crate::services::error::GemServiceError;
use crate::services::explorer::GemExplorerService;
use crate::services::node::GemNodeService;

#[derive(uniffi::Object)]
pub struct GemChainSettingsService {
    nodes: Arc<GemNodeService>,
    explorer: Arc<GemExplorerService>,
    gateway: Arc<GemGateway>,
}

#[uniffi::export]
impl GemChainSettingsService {
    #[uniffi::constructor]
    pub fn new(nodes: Arc<GemNodeService>, explorer: Arc<GemExplorerService>, gateway: Arc<GemGateway>) -> Self {
        Self { nodes, explorer, gateway }
    }

    pub fn chains(&self, query: String) -> Vec<Chain> {
        chain_rules::matching_chains(chain_rules::chains_by_rank(), &query)
    }

    pub fn sections(&self) -> Vec<GemChainSettingsSection> {
        vec![GemChainSettingsSection::Nodes, GemChainSettingsSection::Explorer]
    }

    pub fn explorers(&self, chain: Chain) -> Vec<String> {
        self.explorer.get_explorers(chain)
    }

    pub fn explorer_rows(&self, chain: Chain) -> Vec<GemExplorerRow> {
        let selected = self.explorer.get_explorer_name(chain);
        self.explorer
            .get_explorers(chain)
            .into_iter()
            .map(|name| GemExplorerRow {
                is_selected: name == selected,
                name,
            })
            .collect()
    }

    pub fn node_row(&self, chain: Chain, node: GemNodeSelection, status: GemNodeStatusState) -> GemNodeRow {
        GemNodeRow {
            title: node.title(),
            subtitle: status.subtitle(),
            latency_status: status.latency_status(),
            can_delete: self.can_delete_node(chain, node.url.clone()),
            node,
        }
    }

    pub fn explorer_name(&self, chain: Chain) -> String {
        self.explorer.get_explorer_name(chain)
    }

    pub fn set_explorer_name(&self, chain: Chain, name: String) -> Result<(), GemServiceError> {
        self.explorer.set_explorer_name(chain, name)
    }

    pub async fn nodes(&self, chain: Chain) -> Result<Vec<GemNodeSelection>, GemServiceError> {
        let nodes = self.nodes.get_nodes(chain).await?;
        let selected_url = self.nodes.selected_node(chain).url;
        Ok(rules::node_selections(self.nodes.sorted_nodes(chain, nodes), &selected_url))
    }

    pub async fn select_node(&self, chain: Chain, url: String) -> Result<(), GemServiceError> {
        self.nodes.select_node(chain, url).await
    }

    pub fn can_delete_node(&self, chain: Chain, url: String) -> bool {
        self.nodes.can_delete_node(chain, url)
    }

    pub async fn delete_node(&self, chain: Chain, url: String) -> Result<(), GemServiceError> {
        self.nodes.delete_node(chain, url).await
    }

    pub async fn add_node(&self, chain: Chain, url: String) -> Result<(), GemServiceError> {
        self.nodes.add_node(chain, url.clone()).await?;
        self.nodes.select_node(chain, url).await
    }

    pub async fn node_status(&self, chain: Chain, url: String) -> GemNodeStatusState {
        match self.gateway.get_node_status(chain, &url).await {
            Ok(status) if status.latest_block_number > 0 => GemNodeStatusState::Result {
                latest_block_number: status.latest_block_number,
                latency: Latency::from_milliseconds(status.latency_ms),
            },
            Ok(_) | Err(_) => GemNodeStatusState::Error,
        }
    }

    pub fn new_add_node_session(&self, chain: Chain) -> GemAddNodeSession {
        GemAddNodeSession::new(chain)
    }

    pub fn node_check_debounce_milliseconds(&self) -> u64 {
        rules::node_check_debounce_milliseconds()
    }

    pub async fn check_node(&self, chain: Chain, url: String) -> Result<GemNodeCheck, GemAddNodeError> {
        let url = rules::node_url(&url).ok_or(GemAddNodeError::InvalidUrl)?;
        Ok(self.gateway.check_node(chain, &url).await?)
    }
}
