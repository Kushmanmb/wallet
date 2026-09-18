package com.gemwallet.android.features.asset.viewmodels.details.models

import android.content.Context
import com.gemwallet.android.ext.asset
import com.gemwallet.android.model.AssetInfo
import com.gemwallet.android.model.text
import com.gemwallet.android.testkit.mockAsset
import com.gemwallet.android.testkit.mockAssetInfo
import com.gemwallet.android.testkit.mockChainAssetInfo
import com.gemwallet.android.testkit.mockFormattedNumber
import com.gemwallet.android.ui.R
import com.gemwallet.android.testkit.mockGemAssetDetails
import com.gemwallet.android.testkit.mockGemAssetDetailsState
import com.wallet.core.primitives.Chain
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import io.mockk.unmockkStatic
import java.math.BigInteger
import uniffi.gemstone.GemAssetBalanceRow
import uniffi.gemstone.GemBalanceRow
import uniffi.gemstone.GemBalanceRowValue
import uniffi.gemstone.GemNumberUnit
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test

class AssetInfoUIModelFactoryTest {

    @Before
    fun setUp() {
        mockkStatic("com.gemwallet.android.ext.ChainKt")
        every { Chain.Cosmos.asset() } returns mockAsset(chain = Chain.Cosmos, name = "Cosmos")
        every { Chain.Solana.asset() } returns mockAsset(chain = Chain.Solana, name = "Solana")
        every { Chain.Tron.asset() } returns mockAsset(chain = Chain.Tron, name = "Tron")
        every { Chain.Bitcoin.asset() } returns mockAsset(chain = Chain.Bitcoin, name = "Bitcoin")
    }

    @After
    fun tearDown() = unmockkStatic("com.gemwallet.android.ext.ChainKt")

    @Test
    fun `the row name is the title core decided`() {
        assertEquals("Renamed Cosmos", model(mockAssetInfo(asset = mockAsset(chain = Chain.Cosmos, name = "Renamed Cosmos"), owner = null)).name)
    }

    @Test
    fun `balance rows show the values core formatted`() {
        val staked = mockFormattedNumber(value = 2.0, unit = GemNumberUnit.Symbol(symbol = "ATOM"))
        val apr = mockFormattedNumber(value = 5.0, unit = GemNumberUnit.Percent)
        val balances = model(
            mockAssetInfo(asset = mockAsset(chain = Chain.Cosmos), owner = null),
            balanceRows = listOf(
                GemAssetBalanceRow(GemBalanceRow.Staked(BigInteger("2000000")), GemBalanceRowValue.Amount(staked)),
                GemAssetBalanceRow(GemBalanceRow.Reserved(BigInteger("500000"), "https://reserve"), GemBalanceRowValue.Amount(staked)),
                GemAssetBalanceRow(GemBalanceRow.Staked(BigInteger.ZERO), GemBalanceRowValue.Apr(apr)),
                GemAssetBalanceRow(GemBalanceRow.Staked(BigInteger.ZERO), GemBalanceRowValue.Apr(null)),
            ),
        ).accountInfoUIModel.balances

        assertEquals(
            listOf(
                AssetInfoUIModel.BalanceViewType.Stake,
                AssetInfoUIModel.BalanceViewType.Reserved,
                AssetInfoUIModel.BalanceViewType.Stake,
                AssetInfoUIModel.BalanceViewType.Stake,
            ),
            balances.map { it.type },
        )
        assertEquals(
            listOf(staked.text(), staked.text(), "${R.string.stake_apr}${apr.text()}", "${R.string.stake_apr}"),
            balances.map { it.model.subtitle },
        )
        assertEquals("https://reserve", balances[1].url)
        assertTrue(model(mockAssetInfo(asset = mockAsset(), owner = null)).accountInfoUIModel.balances.isEmpty())
    }

    private val context = mockk<Context> {
        every { getString(any()) } answers { firstArg<Int>().toString() }
        every { getString(any(), *anyVararg()) } answers { "${firstArg<Int>()}${(args[1] as Array<*>).joinToString("")}" }
    }

    private fun model(assetInfo: AssetInfo, balanceRows: List<GemAssetBalanceRow> = emptyList()) = AssetInfoUIModelFactory(context).create(
        mockChainAssetInfo(assetInfo),
        mockGemAssetDetails(assetInfo.asset, mockGemAssetDetailsState(showsBanners = true), balanceRows),
        banners = emptyList(),
    )
}
