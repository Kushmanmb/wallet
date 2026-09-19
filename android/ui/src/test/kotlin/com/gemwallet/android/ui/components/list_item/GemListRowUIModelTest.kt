package com.gemwallet.android.ui.components.list_item

import android.content.Context
import com.gemwallet.android.model.text
import com.gemwallet.android.testkit.mockFormattedNumber
import com.gemwallet.android.testkit.mockGemWalletRow
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.InfoSheetEntity
import com.gemwallet.android.ui.style.textStyle
import io.mockk.every
import io.mockk.mockk
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import uniffi.gemstone.BlockExplorerLink
import uniffi.gemstone.GemCopy
import uniffi.gemstone.GemCopyKind
import uniffi.gemstone.GemInfoTopic
import uniffi.gemstone.GemListRow
import uniffi.gemstone.GemListRowTitle
import uniffi.gemstone.GemLocalizedText
import uniffi.gemstone.GemNumberUnit
import uniffi.gemstone.GemValueTone

class GemListRowUIModelTest {
    private val context = mockk<Context>(relaxed = true) {
        every { getString(R.string.settings_website) } returns "Visit Website"
        every { getString(R.string.wallet_copy_address) } returns "Copy Address"
        every { getString(R.string.common_copy) } returns "Copy"
        every { getString(R.string.transaction_view_on, "Etherscan") } returns "View on Etherscan"
    }

    @Test
    fun `an app row opens only the website core gives`() {
        val withWebsite = GemListRow.App(name = "PancakeSwap", iconUrl = null, websiteUrl = "https://pancakeswap.finance")
        val withoutWebsite = GemListRow.App(name = "PancakeSwap", iconUrl = null, websiteUrl = null)

        assertEquals(listOf(GemListRowMenuItem.Open("Visit Website", "https://pancakeswap.finance")), withWebsite.menu())
        assertEquals(emptyList<GemListRowMenuItem>(), withoutWebsite.menu())
    }

    @Test
    fun `a wallet row copies its address and opens the explorer`() {
        val row = GemListRow.Wallet(
            wallet = mockGemWalletRow(),
            copy = GemCopy(kind = GemCopyKind.Address("ethereum"), value = "0x1", display = "0x1"),
            explorer = BlockExplorerLink(name = "Etherscan", link = "https://etherscan.io/address/0x1"),
        )

        assertEquals(
            listOf(
                GemListRowMenuItem.Copy("Copy Address", "0x1"),
                GemListRowMenuItem.Open("View on Etherscan", "https://etherscan.io/address/0x1"),
            ),
            row.menu(),
        )
    }

    @Test
    fun `a memo row copies only a real memo`() {
        assertEquals(listOf(GemListRowMenuItem.Copy("Copy", "12345")), GemListRow.Memo(value = "12345", copy = "12345").menu())
        assertEquals(emptyList<GemListRowMenuItem>(), GemListRow.Memo(value = "-", copy = null).menu())
    }

    @Test
    fun `a lines row reads its first line and the next one below it`() {
        every { context.getString(R.string.perpetual_auto_close) } returns "Auto Close"
        val row = GemListRow.Lines(
            title = GemListRowTitle.AUTO_CLOSE,
            lines = listOf(GemLocalizedText.Text("Take Profit: $65,000"), GemLocalizedText.Text("Stop Loss: $55,000")),
            info = GemInfoTopic.AutoClose,
        )

        val model = (row.uiModel(context) as GemListRowUIModel.Item).model
        assertEquals("Auto Close", model.title)
        assertEquals("Take Profit: $65,000", model.subtitle)
        assertEquals("Stop Loss: $55,000", model.subtitleExtra)
        assertEquals(InfoSheetEntity.AutoCloseInfo, model.info)
    }

    @Test
    fun `a ranked row carries its rank tag`() {
        every { context.getString(R.string.asset_market_cap) } returns "Market Cap"

        val model = (GemListRow.Ranked(GemListRowTitle.MARKET_CAP, mockFormattedNumber(1.0), 7).uiModel(context) as GemListRowUIModel.Item).model

        assertEquals("Market Cap", model.title)
        assertEquals("#7", model.titleTag)
    }

    @Test
    fun `an identifier opens the explorer only when core gives a link`() {
        every { context.getString(R.string.asset_contract) } returns "Contract"
        val copy = GemCopy(kind = GemCopyKind.Address("ethereum"), value = "0xdAC17F958D2ee523a2206206994597C13D831ec7", display = "0xdAC1...1ec7")
        val explorer = BlockExplorerLink(name = "Etherscan", link = "https://etherscan.io/token/0xdAC17F958D2ee523a2206206994597C13D831ec7")

        val linked = GemListRow.Identifier(title = GemListRowTitle.CONTRACT, copy = copy, explorer = explorer).uiModel(context) as GemListRowUIModel.Item
        val plain = GemListRow.Identifier(title = GemListRowTitle.CONTRACT, copy = copy, explorer = null).uiModel(context) as GemListRowUIModel.Item

        assertEquals("Contract", linked.model.title)
        assertEquals("0xdAC1...1ec7", linked.model.subtitle)
        assertEquals(explorer.link, linked.url)
        assertEquals(
            listOf(GemListRowMenuItem.Copy("Copy Address", copy.value), GemListRowMenuItem.Open("View on Etherscan", explorer.link)),
            linked.menu,
        )
        assertNull(plain.url)
        assertEquals(listOf(GemListRowMenuItem.Copy("Copy Address", copy.value)), plain.menu)
    }

    @Test
    fun `an all time row shows its change in the change's tone`() {
        val change = mockFormattedNumber(-12.0, GemNumberUnit.Percent).copy(tone = GemValueTone.NEGATIVE)
        val row = GemListRow.AllTime(title = GemListRowTitle.ALL_TIME_HIGH, value = mockFormattedNumber(100.0), date = 0L, change = change)

        val model = (row.uiModel(context) as GemListRowUIModel.Item).model

        assertEquals(mockFormattedNumber(100.0).text(), model.subtitle)
        assertEquals(change.text(), model.subtitleExtra)
        assertEquals(GemValueTone.NEGATIVE.textStyle(), model.subtitleExtraStyle)
    }

    private fun GemListRow.menu(): List<GemListRowMenuItem> = (uiModel(context) as GemListRowUIModel.Item).menu
}
