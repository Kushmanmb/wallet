package com.gemwallet.android.features.referral.views

import uniffi.gemstone.GemRewardsState

internal fun previewRewardsState(
    referralCode: String? = null,
    referralCountText: String = "0",
    pointsText: String = "0",
    hasReferralCode: Boolean = false,
    canInvite: Boolean = false,
    canUseReferralCode: Boolean = false,
    showsInfo: Boolean = false,
    canActivatePendingReferral: Boolean = false,
    usedReferralCode: String? = null,
) = GemRewardsState(
    hasReferralCode = hasReferralCode,
    canInvite = canInvite,
    canUseReferralCode = canUseReferralCode,
    showsInfo = showsInfo,
    errorNotice = null,
    statusNotice = null,
    showsPendingActivation = false,
    canActivatePendingReferral = canActivatePendingReferral,
    inviteRewardPointsText = "100",
    referralCode = referralCode,
    referralLink = null,
    usedReferralCode = usedReferralCode,
    referralCountText = referralCountText,
    pointsText = pointsText,
    infoRows = emptyList(),
    redemptions = emptyList(),
)
