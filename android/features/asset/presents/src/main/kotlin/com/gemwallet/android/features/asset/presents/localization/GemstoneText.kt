package com.gemwallet.android.features.asset.presents.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import uniffi.gemstone.PortfolioChartType
import uniffi.gemstone.PortfolioStatistic

@StringRes
internal fun PortfolioChartType.stringRes(): Int = when (this) {
    PortfolioChartType.VALUE -> R.string.perpetual_value
    PortfolioChartType.PNL -> R.string.perpetual_pnl
}

@StringRes
internal fun PortfolioStatistic.stringRes(): Int = when (this) {
    is PortfolioStatistic.AllTimeHigh -> R.string.asset_all_time_high
    is PortfolioStatistic.AllTimeLow -> R.string.asset_all_time_low
    is PortfolioStatistic.UnrealizedPnl -> R.string.perpetual_unrealized_pnl
    is PortfolioStatistic.AccountLeverage -> R.string.perpetual_account_leverage
    is PortfolioStatistic.MarginUsage -> R.string.perpetual_margin_usage
    is PortfolioStatistic.AllTimePnl -> R.string.perpetual_all_time_pnl
    is PortfolioStatistic.Volume -> R.string.perpetual_volume
}
