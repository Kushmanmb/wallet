package com.gemwallet.android.features.wallets.presents.views

import com.gemwallet.android.domains.wallet.aggregates.WalletDataAggregate
import uniffi.gemstone.GemWalletPlaceholder
import uniffi.gemstone.GemWalletRow
import uniffi.gemstone.GemWalletSubtitle
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class WalletsScreenTest {

    @Test
    fun `toWalletSections partitions wallets from one snapshot`() {
        val pinnedWallet = wallet(id = "pinned", isPinned = true)
        val unpinnedWallet = wallet(id = "unpinned", isPinned = false)
        val secondPinnedWallet = wallet(id = "second-pinned", isPinned = true)

        val sections = listOf(pinnedWallet, unpinnedWallet, secondPinnedWallet).toWalletSections()

        assertEquals(listOf(pinnedWallet, secondPinnedWallet), sections.pinnedWallets)
        assertEquals(listOf(unpinnedWallet), sections.unpinnedWallets)
        assertEquals(
            listOf("pinned", "second-pinned", "unpinned"),
            sections.allWallets.map { it.row.id }
        )
    }

    @Test
    fun `toWalletSections keeps empty section lists stable`() {
        val sections = emptyList<WalletDataAggregate>().toWalletSections()

        assertTrue(sections.pinnedWallets.isEmpty())
        assertTrue(sections.unpinnedWallets.isEmpty())
        assertTrue(sections.allWallets.isEmpty())
    }

    private fun wallet(
        id: String,
        isPinned: Boolean,
    ) = object : WalletDataAggregate {
        override val isCurrent: Boolean = false
        override val row: GemWalletRow = GemWalletRow(
            id = id,
            name = id,
            subtitle = GemWalletSubtitle.Multicoin,
            placeholder = GemWalletPlaceholder.Multicoin,
            showsWatchBadge = false,
            hasAvatar = false,
            imageUrl = null,
        )
        override val isPinned: Boolean = isPinned
    }
}
