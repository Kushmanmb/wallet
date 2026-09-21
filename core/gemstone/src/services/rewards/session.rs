use chrono::{DateTime, Utc};
use primitives::{Rewards, WalletId};

use super::model::{GemRewardsPhase, GemRewardsResult, GemRewardsViewState};
use super::rules;
use crate::services::error::GemServiceError;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct GemRewardsSession {
    pub wallet_id: Option<WalletId>,
    pub rewards: Option<Rewards>,
    pub error: Option<GemServiceError>,
    pub is_loading: bool,
}

#[uniffi::export]
impl GemRewardsSession {
    pub fn on_select_wallet(&self, wallet_id: WalletId) -> Self {
        if self.wallet_id.as_ref() == Some(&wallet_id) {
            return self.clone();
        }
        Self {
            wallet_id: Some(wallet_id),
            rewards: None,
            error: None,
            is_loading: true,
        }
    }

    pub fn on_result(&self, result: GemRewardsResult) -> Self {
        if self.wallet_id.as_ref() != Some(&result.wallet_id) {
            return self.clone();
        }
        match result.rewards {
            Some(rewards) => self.on_rewards(rewards),
            None => Self {
                error: self.rewards.is_none().then_some(result.error).flatten(),
                is_loading: false,
                ..self.clone()
            },
        }
    }

    pub fn on_rewards(&self, rewards: Rewards) -> Self {
        Self {
            rewards: Some(rewards),
            error: None,
            is_loading: false,
            ..self.clone()
        }
    }

    pub fn request(&self) -> Option<WalletId> {
        self.wallet_id.clone()
    }

    pub fn view_state(&self, now: DateTime<Utc>) -> GemRewardsViewState {
        GemRewardsViewState {
            phase: self.phase(),
            rewards: rules::state(self.rewards.as_ref(), now),
        }
    }
}

impl GemRewardsSession {
    fn phase(&self) -> GemRewardsPhase {
        if self.is_loading {
            return GemRewardsPhase::Loading;
        }
        match (&self.rewards, &self.error) {
            (Some(_), _) => GemRewardsPhase::Data,
            (None, Some(error)) => GemRewardsPhase::Failed { error: error.clone() },
            (None, None) => GemRewardsPhase::Data,
        }
    }
}

#[uniffi::export]
pub fn rewards_session() -> GemRewardsSession {
    GemRewardsSession {
        wallet_id: None,
        rewards: None,
        error: None,
        is_loading: true,
    }
}

#[cfg(test)]
mod tests {
    use primitives::RewardStatus;

    use super::*;

    fn now() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn wallet() -> WalletId {
        WalletId::Multicoin("0x1".to_string())
    }

    fn offline() -> GemServiceError {
        GemServiceError::Gateway { msg: "offline".to_string() }
    }

    fn invited() -> Rewards {
        Rewards {
            code: Some("GEM123".to_string()),
            ..Rewards::mock(None, RewardStatus::Verified)
        }
    }

    #[test]
    fn test_a_failed_load_reads_as_a_failure_instead_of_a_wallet_without_a_code() {
        let session = rewards_session().on_select_wallet(wallet());

        let failed = session.on_result(GemRewardsResult {
            wallet_id: wallet(),
            rewards: None,
            error: Some(offline()),
        });

        assert!(matches!(failed.view_state(now()).phase, GemRewardsPhase::Failed { .. }), "a wallet with a code must not be offered the create-code screen");
        assert!(!failed.view_state(now()).rewards.can_invite);
    }

    #[test]
    fn test_a_failed_refresh_keeps_the_code_already_on_screen() {
        let shown = rewards_session().on_select_wallet(wallet()).on_rewards(invited());

        let kept = shown.on_result(GemRewardsResult {
            wallet_id: wallet(),
            rewards: None,
            error: Some(offline()),
        });

        assert_eq!(kept.view_state(now()).phase, GemRewardsPhase::Data);
        assert_eq!(kept.view_state(now()).rewards, shown.view_state(now()).rewards);
    }

    #[test]
    fn test_a_result_for_a_wallet_that_is_no_longer_shown_is_dropped() {
        let second = WalletId::Multicoin("0x2".to_string());
        let shown = rewards_session().on_select_wallet(second);
        let late = GemRewardsResult {
            wallet_id: wallet(),
            rewards: Some(invited()),
            error: None,
        };

        assert_eq!(shown.on_result(late), shown, "the wallet moved on before the answer arrived");
    }

    #[test]
    fn test_selecting_another_wallet_starts_over_and_reselecting_the_same_one_does_not() {
        let shown = rewards_session().on_select_wallet(wallet()).on_rewards(invited());

        assert_eq!(shown.on_select_wallet(wallet()), shown);
        let switched = shown.on_select_wallet(WalletId::Multicoin("0x2".to_string()));
        assert_eq!(switched.view_state(now()).phase, GemRewardsPhase::Loading);
        assert_eq!(switched.request(), Some(WalletId::Multicoin("0x2".to_string())));
    }
}
