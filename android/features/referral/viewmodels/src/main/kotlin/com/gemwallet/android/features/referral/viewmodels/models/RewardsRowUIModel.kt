package com.gemwallet.android.features.referral.viewmodels.models

import android.content.Context
import com.gemwallet.android.domains.duration.formatDuration
import com.gemwallet.android.ext.toPrimitives
import com.gemwallet.android.model.text
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.list_item.ListItemImage
import com.gemwallet.android.ui.components.list_item.ListItemModel
import com.gemwallet.android.ui.components.list_item.ListItemTextStyle
import com.gemwallet.android.ui.models.ButtonState
import com.gemwallet.android.ui.models.buttonState
import uniffi.gemstone.GemRewardsRedemption
import uniffi.gemstone.GemRewardsState

data class RewardRedemptionUIModel(
    val redemption: GemRewardsRedemption,
    val model: ListItemModel,
    val confirmationMessage: String,
)

data class PendingReferralUIModel(
    val model: ListItemModel,
    val code: String,
    val buttonState: ButtonState,
)

internal fun GemRewardsState.errorRow(context: Context): ListItemModel? = disableReason?.let { reason ->
    ListItemModel(
        title = context.getString(R.string.errors_error_occurred),
        titleStyle = ListItemTextStyle.Negative,
        titleExtra = reason,
    )
}

internal fun GemRewardsState.unverifiedRow(context: Context): ListItemModel? = if (!isUnverified) {
    null
} else {
    ListItemModel(
        title = context.getString(R.string.rewards_unverified_title),
        titleStyle = ListItemTextStyle.Warning,
        titleExtra = context.getString(R.string.rewards_unverified_description),
    )
}

internal fun GemRewardsState.pendingReferralRow(context: Context): PendingReferralUIModel? {
    if (!hasPendingReferral) return null
    val code = usedReferralCode ?: return null
    return PendingReferralUIModel(
        model = ListItemModel(
            title = context.getString(R.string.rewards_pending_title),
            titleStyle = ListItemTextStyle.Warning,
            titleExtra = if (canActivatePendingReferral) {
                context.getString(R.string.rewards_pending_description_ready)
            } else {
                context.getString(R.string.rewards_pending_description, pendingCountdown.formatDuration())
            },
        ),
        code = code,
        buttonState = buttonState(enabled = canActivatePendingReferral),
    )
}

internal fun GemRewardsState.infoRows(context: Context): List<ListItemModel> = listOfNotNull(
    referralCode?.let { ListItemModel(title = context.getString(R.string.rewards_my_referral_code), subtitle = it) },
    ListItemModel(title = context.getString(R.string.rewards_referrals), subtitle = referralCountText),
    ListItemModel(title = context.getString(R.string.rewards_points), subtitle = pointsText),
)

internal fun GemRewardsRedemption.uiModel(context: Context): RewardRedemptionUIModel? {
    val asset = option.asset ?: return null
    return RewardRedemptionUIModel(
        redemption = this,
        model = ListItemModel(
            title = context.getString(R.string.rewards_ways_spend_asset_title, value.text()),
            subtitle = pointsText,
            image = ListItemImage.Asset(asset.toPrimitives().id),
        ),
        confirmationMessage = context.getString(R.string.rewards_confirm_redeem, value.text(), pointsText),
    )
}
