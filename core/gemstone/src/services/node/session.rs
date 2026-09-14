use primitives::Chain;

use super::model::GemNodeCheck;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum GemAddNodeFailure {
    InvalidUrl,
    InvalidNetworkId,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, uniffi::Enum)]
pub enum GemAddNodePhase {
    Idle,
    Checking,
    Ready { check: GemNodeCheck },
    Failed { failure: GemAddNodeFailure },
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct GemAddNodeViewState {
    pub phase: GemAddNodePhase,
    pub can_import: bool,
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct GemAddNodeSession {
    pub chain: Chain,
    pub url: String,
    pub check: Option<GemNodeCheck>,
    pub failure: Option<GemAddNodeFailure>,
    pub is_checking: bool,
}

impl GemAddNodeSession {
    pub fn new(chain: Chain) -> Self {
        Self {
            chain,
            url: String::new(),
            check: None,
            failure: None,
            is_checking: false,
        }
    }
}

#[uniffi::export]
impl GemAddNodeSession {
    pub fn on_input(&self, url: String) -> Self {
        Self {
            url: url.trim().to_string(),
            check: None,
            failure: None,
            is_checking: false,
            ..self.clone()
        }
    }

    pub fn on_checking(&self) -> Self {
        Self {
            check: None,
            failure: None,
            is_checking: !self.url.is_empty(),
            ..self.clone()
        }
    }

    pub fn on_checked(&self, check: GemNodeCheck) -> Self {
        Self {
            check: Some(check),
            failure: None,
            is_checking: false,
            ..self.clone()
        }
    }

    pub fn on_failed(&self, failure: GemAddNodeFailure) -> Self {
        Self {
            check: None,
            failure: Some(failure),
            is_checking: false,
            ..self.clone()
        }
    }

    pub fn on_imported(&self) -> Self {
        Self::new(self.chain)
    }

    pub fn checks_url(&self) -> bool {
        !self.url.is_empty()
    }

    pub fn view_state(&self) -> GemAddNodeViewState {
        GemAddNodeViewState {
            phase: self.phase(),
            can_import: self.check.is_some(),
        }
    }
}

impl GemAddNodeSession {
    fn phase(&self) -> GemAddNodePhase {
        if self.is_checking {
            return GemAddNodePhase::Checking;
        }
        match (&self.check, self.failure) {
            (Some(check), _) => GemAddNodePhase::Ready { check: check.clone() },
            (None, Some(failure)) => GemAddNodePhase::Failed { failure },
            (None, None) => GemAddNodePhase::Idle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::Latency;

    fn check() -> GemNodeCheck {
        GemNodeCheck {
            url: "https://node".to_string(),
            chain_id: None,
            latest_block_number: 1,
            is_in_sync: true,
            latency: Latency::from_milliseconds(10),
        }
    }

    #[test]
    fn test_an_empty_url_is_idle_and_never_starts_a_check() {
        let session = GemAddNodeSession::new(Chain::Ethereum).on_input("   ".to_string());

        assert!(!session.checks_url());
        assert_eq!(session.on_checking().view_state().phase, GemAddNodePhase::Idle);
    }

    #[test]
    fn test_a_new_url_clears_the_previous_answer() {
        let checked = GemAddNodeSession::new(Chain::Ethereum).on_input("https://node".to_string()).on_checked(check());
        assert!(checked.view_state().can_import);

        let retyped = checked.on_input("https://other".to_string());
        assert_eq!(retyped.view_state().phase, GemAddNodePhase::Idle);
        assert!(!retyped.view_state().can_import, "a url that was never checked cannot be imported");
    }

    #[test]
    fn test_a_failure_replaces_the_answer_and_blocks_the_import() {
        let failed = GemAddNodeSession::new(Chain::Ethereum)
            .on_input("https://node".to_string())
            .on_checked(check())
            .on_failed(GemAddNodeFailure::InvalidNetworkId);

        assert_eq!(
            failed.view_state().phase,
            GemAddNodePhase::Failed {
                failure: GemAddNodeFailure::InvalidNetworkId
            }
        );
        assert!(!failed.view_state().can_import);
    }

    #[test]
    fn test_importing_leaves_the_screen_ready_for_the_next_url() {
        let imported = GemAddNodeSession::new(Chain::Ethereum).on_input("https://node".to_string()).on_checked(check()).on_imported();

        assert_eq!(imported, GemAddNodeSession::new(Chain::Ethereum));
    }
}
