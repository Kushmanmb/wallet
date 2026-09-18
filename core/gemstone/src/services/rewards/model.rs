use crate::formatted_number::GemFormattedNumber;
use crate::models::list::GemListRow;
use primitives::RewardRedemptionOption;

#[derive(Debug, Clone, Default, PartialEq, uniffi::Record)]
pub struct GemRewardsState {
    pub has_referral_code: bool,
    pub has_used_referral_code: bool,
    pub can_invite: bool,
    pub can_use_referral_code: bool,
    pub shows_info: bool,
    pub error_notice: Option<GemListRow>,
    pub status_notice: Option<GemListRow>,
    pub shows_pending_activation: bool,
    pub can_activate_pending_referral: bool,
    pub invite_reward_points_text: String,
    pub referral_code: Option<String>,
    pub referral_link: Option<String>,
    pub used_referral_code: Option<String>,
    pub referral_count_text: String,
    pub points_text: String,
    pub redemptions: Vec<GemRewardsRedemption>,
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct GemRewardsRedemption {
    pub option: RewardRedemptionOption,
    pub can_redeem: bool,
    pub points_text: String,
    pub value: GemFormattedNumber,
}
