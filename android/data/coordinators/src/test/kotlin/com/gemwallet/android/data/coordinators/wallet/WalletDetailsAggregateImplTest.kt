package com.gemwallet.android.data.coordinators.wallet

import com.wallet.core.primitives.Account
import com.wallet.core.primitives.Chain
import com.wallet.core.primitives.Wallet
import com.wallet.core.primitives.WalletId
import com.wallet.core.primitives.WalletSource
import com.wallet.core.primitives.WalletType
import org.junit.Assert.assertEquals
import uniffi.gemstone.GemWalletPlaceholder
import uniffi.gemstone.GemWalletRow
import uniffi.gemstone.GemWalletSubtitle
import org.junit.Test

class WalletDetailsAggregateImplTest {

    @Test
    fun accounts_preservesChainAndAddress() {
        val wallet = Wallet(
            id = WalletId("single_ethereum_0x403BC00000000000000000000000000000051bDa"),
            name = "Wallet",
            index = 0,
            type = WalletType.Single,
            accounts = listOf(
                Account(
                    chain = Chain.Ethereum,
                    address = "0x403BC00000000000000000000000000000051bDa",
                    derivationPath = "m/44'/60'/0'/0/0",
                ),
            ),
            isPinned = false,
            source = WalletSource.Create,
        )

        val aggregate = WalletDetailsAggregateImpl(
            wallet,
            GemWalletRow(
                id = wallet.id.id,
                name = wallet.name,
                subtitle = GemWalletSubtitle.Multicoin,
                placeholder = GemWalletPlaceholder.Multicoin,
                showsWatchBadge = false,
                hasAvatar = false,
                imageUrl = null,
            ),
        )

        assertEquals(Chain.Ethereum, aggregate.accounts.single().chain)
        assertEquals("0x403BC00000000000000000000000000000051bDa", aggregate.accounts.single().address)
    }
}
