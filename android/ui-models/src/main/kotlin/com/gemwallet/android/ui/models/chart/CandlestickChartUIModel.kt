package com.gemwallet.android.ui.models.chart

import com.gemwallet.android.model.text
import com.wallet.core.primitives.ChartCandleStick
import uniffi.gemstone.GemPerpetualChartLayout
import uniffi.gemstone.GemPerpetualChartLineKind
import uniffi.gemstone.GemValueTone

enum class CandleDirection {
    Up,
    Down,
    Flat,
}

data class ChartReferenceLineUIModel(val kind: GemPerpetualChartLineKind, val price: Double, val overlapLevel: Int, val label: String)

data class ChartAxisTick(val value: Double, val fraction: Float, val label: String)

data class CandleUIModel(val open: Double, val high: Double, val low: Double, val close: Double, val direction: CandleDirection)

data class CandlestickChartUIModel(
    val candles: List<CandleUIModel>,
    val yMin: Double,
    val yMax: Double,
    val yTicks: List<ChartAxisTick>,
    val xGridlineFractions: List<Float>,
    val referenceLines: List<ChartReferenceLineUIModel>,
    val currentPriceLabel: String,
) {
    val ySpan: Double get() = yMax - yMin

    companion object {

        fun from(candles: List<ChartCandleStick>, layout: GemPerpetualChartLayout, lineLabel: (GemPerpetualChartLineKind) -> String): CandlestickChartUIModel {
            val span = layout.priceHigh - layout.priceLow
            return CandlestickChartUIModel(
                candles = candles.zip(layout.tones, ::candleUIModel),
                yMin = layout.priceLow,
                yMax = layout.priceHigh,
                yTicks = layout.ticks.map { tick ->
                    ChartAxisTick(value = tick.value, fraction = ((tick.value - layout.priceLow) / span).toFloat(), label = tick.text())
                },
                xGridlineFractions = buildXGridlineFractions(layout.xTickCount.toInt()),
                referenceLines = layout.lines.map { line ->
                    ChartReferenceLineUIModel(
                        kind = line.kind,
                        price = line.price.value,
                        overlapLevel = line.overlapLevel.toInt(),
                        label = "${lineLabel(line.kind)} | ${line.price.text()}",
                    )
                },
                currentPriceLabel = layout.currentPrice?.text().orEmpty(),
            )
        }

        private fun buildXGridlineFractions(tickCount: Int): List<Float> {
            if (tickCount < 2) return emptyList()
            return (0 until tickCount).map { tick -> tick.toFloat() / (tickCount - 1) }
        }

        private fun candleUIModel(candle: ChartCandleStick, tone: GemValueTone): CandleUIModel = CandleUIModel(
            open = candle.open,
            high = candle.high,
            low = candle.low,
            close = candle.close,
            direction = when (tone) {
                GemValueTone.POSITIVE -> CandleDirection.Up

                GemValueTone.NEGATIVE -> CandleDirection.Down

                GemValueTone.NEUTRAL,
                GemValueTone.PLAIN,
                GemValueTone.WARNING,
                -> CandleDirection.Flat
            },
        )
    }
}
