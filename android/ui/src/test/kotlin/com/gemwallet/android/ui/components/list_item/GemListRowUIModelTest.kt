package com.gemwallet.android.ui.components.list_item

import android.content.Context
import com.gemwallet.android.testkit.mockGemWalletRow
import com.gemwallet.android.ui.R
import io.mockk.every
import io.mockk.mockk
import org.junit.Assert.assertEquals
import org.junit.Test
import uniffi.gemstone.BlockExplorerLink
import uniffi.gemstone.GemCopy
import uniffi.gemstone.GemCopyKind
import uniffi.gemstone.GemListRow

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

    private fun GemListRow.menu(): List<GemListRowMenuItem> = (uiModel(context) as GemListRowUIModel.Item).menu
}
