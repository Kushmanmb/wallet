package com.gemwallet.android.ui.components.list_item

import com.gemwallet.android.ui.R
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import uniffi.gemstone.GemWalletPlaceholder
import uniffi.gemstone.GemWalletRow
import uniffi.gemstone.GemWalletSubtitle

class WalletItemTest {

    @Test
    fun `a row with the watch badge points at the badge drawable`() {
        val row = GemWalletRow(id = "1", name = "Wallet", subtitle = GemWalletSubtitle.Multicoin, placeholder = GemWalletPlaceholder.Multicoin, showsWatchBadge = true, hasAvatar = false, imageUrl = null)

        assertEquals(
            "android.resource://com.gemwallet.android/drawable/${R.drawable.watch_badge}",
            row.supportIcon(),
        )
    }

    @Test
    fun `a row without the watch badge has no support icon`() {
        val row = GemWalletRow(id = "1", name = "Wallet", subtitle = GemWalletSubtitle.Multicoin, placeholder = GemWalletPlaceholder.Multicoin, showsWatchBadge = false, hasAvatar = false, imageUrl = null)

        assertNull(row.supportIcon())
    }
}
