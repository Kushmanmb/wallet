package com.gemwallet.android.features.settings.security.viewmodels.models

import androidx.annotation.StringRes
import com.gemwallet.android.features.settings.security.viewmodels.localization.stringRes
import uniffi.gemstone.GemSecurityRow
import uniffi.gemstone.lockPeriodFromMinutes

sealed interface SecurityRowUIModel {
    data class Authentication(val isEnabled: Boolean) : SecurityRowUIModel
    data class LockPeriod(@StringRes val current: Int, val options: List<LockPeriodOption>) : SecurityRowUIModel
    data class HideBalance(val isEnabled: Boolean) : SecurityRowUIModel
}

data class LockPeriodOption(val minutes: Int, @StringRes val title: Int, val isSelected: Boolean = false)

internal fun GemSecurityRow.uiModel(
    authRequired: Boolean,
    lockInterval: Int,
    hideBalances: Boolean,
    lockPeriods: List<LockPeriodOption>,
): SecurityRowUIModel? = when (this) {
    GemSecurityRow.AUTHENTICATION -> SecurityRowUIModel.Authentication(authRequired)
    GemSecurityRow.LOCK_PERIOD -> SecurityRowUIModel.LockPeriod(
        current = lockPeriodFromMinutes(lockInterval.toUInt()).stringRes(),
        options = lockPeriods.map { it.copy(isSelected = it.minutes == lockInterval) },
    )
    GemSecurityRow.PRIVACY_LOCK -> null
    GemSecurityRow.HIDE_BALANCE -> SecurityRowUIModel.HideBalance(hideBalances)
}
