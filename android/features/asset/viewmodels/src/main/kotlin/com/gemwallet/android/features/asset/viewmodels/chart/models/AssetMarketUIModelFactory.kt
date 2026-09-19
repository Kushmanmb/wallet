package com.gemwallet.android.features.asset.viewmodels.chart.models

import android.content.Context
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.list_item.ListItemModel
import dagger.hilt.android.qualifiers.ApplicationContext
import uniffi.gemstone.GemChartSection
import uniffi.gemstone.GemListRow
import javax.inject.Inject

class AssetMarketUIModelFactory @Inject constructor(@ApplicationContext private val context: Context) {

    fun create(sections: List<GemChartSection>): AssetMarketUIModel = AssetMarketUIModel(
        sections = sections.map { section ->
            when (section) {
                is GemChartSection.PriceAlerts -> ChartSectionUIModel.PriceAlerts(
                    ListItemModel(title = context.getString(R.string.settings_price_alerts_title), subtitle = section.count.toString()),
                )

                GemChartSection.SetPriceAlert -> ChartSectionUIModel.SetPriceAlert(ListItemModel(title = context.getString(R.string.price_alerts_set_alert_title)))

                is GemChartSection.Market -> ChartSectionUIModel.Market(section.rows)

                is GemChartSection.Links -> ChartSectionUIModel.Links(
                    title = context.getString(R.string.social_links),
                    row = GemListRow.Social(section.links),
                )
            }
        },
    )
}
