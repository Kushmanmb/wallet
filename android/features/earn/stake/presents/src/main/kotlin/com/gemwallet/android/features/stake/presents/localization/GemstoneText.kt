package com.gemwallet.android.features.stake.presents.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import uniffi.gemstone.GemStakeAction

@StringRes
internal fun GemStakeAction.stringRes(): Int = when (this) {
    GemStakeAction.CLAIM_REWARDS -> R.string.transfer_claim_rewards_title
    GemStakeAction.STAKE -> R.string.transfer_stake_title
    GemStakeAction.FREEZE -> R.string.transfer_freeze_title
    GemStakeAction.UNFREEZE -> R.string.transfer_unfreeze_title
}
