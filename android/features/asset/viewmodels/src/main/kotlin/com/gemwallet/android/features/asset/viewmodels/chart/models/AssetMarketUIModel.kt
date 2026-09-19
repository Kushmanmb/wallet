package com.gemwallet.android.features.asset.viewmodels.chart.models

import com.gemwallet.android.ui.components.list_item.ListItemModel
import com.wallet.core.primitives.Chain
import com.wallet.core.primitives.Currency
import uniffi.gemstone.GemListRow

class AssetMarketUIModel(
    val chain: Chain,
    val currency: Currency,
    val sections: List<ChartSectionUIModel>,
)

sealed interface ChartSectionUIModel {
    data class PriceAlerts(val model: ListItemModel) : ChartSectionUIModel
    data class SetPriceAlert(val model: ListItemModel) : ChartSectionUIModel
    data class Market(val rows: List<MarketRowUIModel>) : ChartSectionUIModel
    data class Links(val title: String, val row: GemListRow) : ChartSectionUIModel
}
