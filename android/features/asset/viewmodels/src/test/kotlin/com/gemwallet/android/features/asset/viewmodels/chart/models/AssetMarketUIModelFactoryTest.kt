package com.gemwallet.android.features.asset.viewmodels.chart.models

import android.content.Context
import com.gemwallet.android.testkit.mockFormattedNumber
import com.gemwallet.android.testkit.mockGemSocialLink
import io.mockk.every
import io.mockk.mockk
import org.junit.Assert.assertEquals
import org.junit.Test
import uniffi.gemstone.GemChartSection
import uniffi.gemstone.GemListRow
import uniffi.gemstone.GemListRowTitle

class AssetMarketUIModelFactoryTest {

    private val context = mockk<Context> { every { getString(any()) } answers { firstArg<Int>().toString() } }
    private val factory = AssetMarketUIModelFactory(context)

    @Test
    fun `sections keep their order and pass the market rows core built`() {
        val rows = listOf(GemListRow.Ranked(GemListRowTitle.MARKET_CAP, mockFormattedNumber(1.0), 7), GemListRow.Amount(GemListRowTitle.TRADING_VOLUME, mockFormattedNumber(2.0), null))
        val sections = listOf(
            GemChartSection.PriceAlerts(count = 2u),
            GemChartSection.Market(rows),
            GemChartSection.Links(listOf(mockGemSocialLink())),
        )

        val model = factory.create(sections)

        assertEquals("2", (model.sections[0] as ChartSectionUIModel.PriceAlerts).model.subtitle)
        assertEquals(rows, (model.sections[1] as ChartSectionUIModel.Market).rows)
        assertEquals(GemListRow.Social(listOf(mockGemSocialLink())), (model.sections[2] as ChartSectionUIModel.Links).row)
    }
}
