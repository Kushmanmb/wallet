package com.gemwallet.android.ui.models.chart

import com.gemwallet.android.ext.toGem
import com.gemwallet.android.model.text
import com.wallet.core.primitives.ChartCandleStick
import uniffi.gemstone.GemValueTone
import uniffi.gemstone.candleTooltip

data class CandlestickTooltipUIModel(
    val open: String,
    val high: String,
    val low: String,
    val close: String,
    val changeText: String,
    val changeTone: GemValueTone,
    val volumeText: String,
) {
    companion object {
        fun from(candle: ChartCandleStick): CandlestickTooltipUIModel {
            val tooltip = candleTooltip(candle.toGem())
            return CandlestickTooltipUIModel(
                open = tooltip.open.text(),
                high = tooltip.high.text(),
                low = tooltip.low.text(),
                close = tooltip.close.text(),
                changeText = tooltip.change.text(),
                changeTone = tooltip.change.tone,
                volumeText = tooltip.volume.text(),
            )
        }
    }
}
