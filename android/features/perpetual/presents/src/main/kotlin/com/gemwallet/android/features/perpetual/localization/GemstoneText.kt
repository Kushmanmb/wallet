package com.gemwallet.android.features.perpetual.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.PerpetualMarginType

@StringRes
internal fun PerpetualMarginType.stringRes(): Int = when (this) {
    PerpetualMarginType.Cross -> R.string.perpetual_margin_cross
    PerpetualMarginType.Isolated -> R.string.perpetual_margin_isolated
}
