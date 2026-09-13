package com.gemwallet.android.ui.models.chart

import com.gemwallet.android.domains.percentage.formatAsPercentage
import com.gemwallet.android.domains.price.ValueDirection
import com.gemwallet.android.domains.price.toValueDirection
import com.gemwallet.android.ext.toGem
import com.wallet.core.primitives.ChartCandleStick
import uniffi.gemstone.candleTooltip

data class CandlestickTooltipUIModel(
    val open: String,
    val high: String,
    val low: String,
    val close: String,
    val changeText: String,
    val changeDirection: ValueDirection,
    val volumeText: String,
) {
    companion object {
        fun from(
            candle: ChartCandleStick,
            priceFormatter: (Double) -> String,
            volumeFormatter: (Double) -> String,
        ): CandlestickTooltipUIModel {
            val tooltip = candleTooltip(candle.toGem())
            return CandlestickTooltipUIModel(
                open = priceFormatter(tooltip.open),
                high = priceFormatter(tooltip.high),
                low = priceFormatter(tooltip.low),
                close = priceFormatter(tooltip.close),
                changeText = tooltip.changePercentage.formatAsPercentage(),
                changeDirection = tooltip.changePercentage.toValueDirection(),
                volumeText = volumeFormatter(tooltip.volume),
            )
        }
    }
}
