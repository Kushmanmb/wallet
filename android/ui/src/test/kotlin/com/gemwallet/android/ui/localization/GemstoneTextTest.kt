package com.gemwallet.android.ui.localization

import android.content.Context
import com.gemwallet.android.model.text
import com.gemwallet.android.testkit.mockFormattedNumber
import com.gemwallet.android.testkit.mockGemSimulationWarningRow
import io.mockk.every
import io.mockk.mockk
import com.gemwallet.android.ui.R
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import uniffi.gemstone.FeeUnitType
import uniffi.gemstone.GemLocalizedText
import uniffi.gemstone.GemNumberUnit
import uniffi.gemstone.GemSimulationWarningKind
import uniffi.gemstone.SimulationSeverity

class GemstoneTextTest {

    @Test
    fun unlimitedApproval_usesUnlimitedWarningCopy() {
        val row = mockGemSimulationWarningRow(GemSimulationWarningKind.UNLIMITED_APPROVAL)

        assertEquals(R.string.simulation_warning_unlimited_token_approval_title, row.titleRes())
        assertEquals(R.string.simulation_warning_unlimited_token_approval_description, row.descriptionRes())
    }

    @Test
    fun validationWarning_keepsExistingWarningBehavior() {
        val row = mockGemSimulationWarningRow(GemSimulationWarningKind.VALIDATION_ERROR, message = "Chain ID mismatch")

        assertEquals(R.string.common_warning, row.titleRes())
        assertNull(row.descriptionRes())
    }

    @Test
    fun criticalValidationError_usesErrorCopy() {
        val row = mockGemSimulationWarningRow(GemSimulationWarningKind.VALIDATION_ERROR, severity = SimulationSeverity.CRITICAL)

        assertEquals(R.string.errors_error_occurred, row.titleRes())
        assertEquals(R.string.errors_error_occurred, row.descriptionRes())
    }

    @Test
    fun feeRate_readsTheCoreRateWithItsLocalizedUnit() {
        val context = mockk<Context> {
            every { getString(R.string.fee_rate_satvB) } returns "sat/vB"
            every { getString(R.string.fee_rate_gwei) } returns "gwei"
        }
        val rate = mockFormattedNumber(value = 2.5, unit = GemNumberUnit.Plain)
        val sol = mockFormattedNumber(value = 2.5, unit = GemNumberUnit.Symbol(symbol = "SOL"))

        assertEquals("${rate.text()} sat/vB", GemLocalizedText.FeeRate(rate, FeeUnitType.SAT_VB).string(context))
        assertEquals("${rate.text()} gwei", GemLocalizedText.FeeRate(rate, FeeUnitType.GWEI).string(context))
        assertEquals(sol.text(), GemLocalizedText.FeeRate(sol, FeeUnitType.NATIVE).string(context))
    }

    @Test
    fun externallyOwnedSpender_usesSpecificWarningDescription() {
        val row = mockGemSimulationWarningRow(GemSimulationWarningKind.EXTERNALLY_OWNED_SPENDER)

        assertEquals(R.string.common_warning, row.titleRes())
        assertEquals(R.string.simulation_warning_externally_owned_spender_description, row.descriptionRes())
    }
}
