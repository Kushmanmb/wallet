package com.gemwallet.android.features.asset.presents.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import uniffi.gemstone.PortfolioChartType

@StringRes
internal fun PortfolioChartType.stringRes(): Int = when (this) {
    PortfolioChartType.VALUE -> R.string.perpetual_value
    PortfolioChartType.PNL -> R.string.perpetual_pnl
}
