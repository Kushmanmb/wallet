package com.gemwallet.android.features.confirm.viewmodels

import com.gemwallet.android.features.confirm.viewmodels.models.ConfirmHeaderUIModel
import com.gemwallet.android.features.confirm.viewmodels.models.confirmHeader
import com.gemwallet.android.testkit.mockAmountUIModel
import com.gemwallet.android.testkit.mockAssetSolana
import com.gemwallet.android.ui.components.list_head.SimulationHeaderUIModel
import org.junit.After
import org.junit.Assert.assertEquals
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

        assertEquals(
            ConfirmHeaderUIModel.Placeholder(asset),
            confirmHeader(
                amountModel = mockAmountUIModel(),
                simulationHeader = null,
                isPayment = false,
                isLoading = true,
                headerAsset = asset,
                awaitsApprovalHeader = true,
            ),
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
                isPayment = false,
                isLoading = true,
                headerAsset = asset,
                awaitsApprovalHeader = true,
            ),
        )
    }

    @Test
    fun aLoadedTransferKeepsItsAmountHeader() {
        val header = confirmHeader(
            amountModel = mockAmountUIModel(),
            simulationHeader = null,
            isPayment = false,
            isLoading = false,
            headerAsset = mockAssetSolana(),
            awaitsApprovalHeader = false,
        )

        assertEquals("1 SOL", (header as ConfirmHeaderUIModel.Amount).amount)
        assertEquals("", header.equivalent)
    }
}
