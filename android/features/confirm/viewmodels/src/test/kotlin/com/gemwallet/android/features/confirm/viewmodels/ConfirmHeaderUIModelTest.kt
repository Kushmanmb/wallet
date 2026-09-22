package com.gemwallet.android.features.confirm.viewmodels

import com.gemwallet.android.features.confirm.viewmodels.models.ConfirmHeaderUIModel
import com.gemwallet.android.features.confirm.viewmodels.models.confirmHeader
import com.gemwallet.android.features.confirm.viewmodels.models.placeholderHeader
import com.gemwallet.android.testkit.mockAmountUIModel
import com.gemwallet.android.testkit.mockAssetSolana
import com.gemwallet.android.ui.components.list_head.SimulationHeaderUIModel
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Before
import org.junit.Test
import java.util.Locale

class ConfirmHeaderUIModelTest {
    private var originalLocale: Locale = Locale.getDefault()

    @Before
    fun setUp() {
        originalLocale = Locale.getDefault()
        Locale.setDefault(Locale.US)
    }

    @After
    fun tearDown() {
        Locale.setDefault(originalLocale)
    }

    @Test
    fun anApprovalHeaderWaitsInPlaceInsteadOfShowingTheTransferAmount() {
        val asset = mockAssetSolana()
        val placeholder = placeholderHeader(isLoading = true, isPayment = false, headerAsset = asset, pendingHeaderAssetId = asset.id)

        assertEquals(ConfirmHeaderUIModel.Placeholder(asset.id), placeholder)
        assertEquals(
            placeholder,
            confirmHeader(amountModel = mockAmountUIModel(), simulationHeader = null, placeholder = placeholder, headerAsset = asset),
        )
    }

    @Test
    fun aResolvedApprovalHeaderReplacesThePlaceholder() {
        val asset = mockAssetSolana()
        val simulationHeader = SimulationHeaderUIModel(asset, "20 USDC")

        assertEquals(
            ConfirmHeaderUIModel.Simulation(simulationHeader),
            confirmHeader(
                amountModel = mockAmountUIModel(),
                simulationHeader = simulationHeader,
                placeholder = ConfirmHeaderUIModel.Placeholder(asset.id),
                headerAsset = asset,
            ),
        )
    }

    @Test
    fun aPaymentHeaderReservesSpaceWithoutShowingTheIcon() {
        val asset = mockAssetSolana()

        assertEquals(
            ConfirmHeaderUIModel.ReservedSpace(asset),
            placeholderHeader(isLoading = true, isPayment = true, headerAsset = asset, pendingHeaderAssetId = asset.id),
        )
    }

    @Test
    fun aLoadedTransferKeepsItsAmountHeader() {
        val asset = mockAssetSolana()

        assertNull(placeholderHeader(isLoading = false, isPayment = false, headerAsset = asset, pendingHeaderAssetId = asset.id))

        val header = confirmHeader(amountModel = mockAmountUIModel(), simulationHeader = null, placeholder = null, headerAsset = asset)

        assertEquals("1 SOL", (header as ConfirmHeaderUIModel.Amount).amount)
        assertEquals("", header.equivalent)
    }
}
