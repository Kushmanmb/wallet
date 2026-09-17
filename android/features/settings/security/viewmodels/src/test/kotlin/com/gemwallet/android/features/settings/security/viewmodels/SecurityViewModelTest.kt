package com.gemwallet.android.features.settings.security.viewmodels

import com.gemwallet.android.data.services.gemstone.config.UserConfig
import com.gemwallet.android.features.settings.security.viewmodels.models.SecurityRowUIModel
import com.gemwallet.android.ui.R
import io.mockk.coVerify
import io.mockk.every
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.advanceUntilIdle
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test
import uniffi.gemstone.GemSecurityRow
import uniffi.gemstone.GemSecuritySection
import uniffi.gemstone.GemSettingsServiceInterface

@OptIn(ExperimentalCoroutinesApi::class)
class SecurityViewModelTest {

    private val dispatcher = StandardTestDispatcher()

    @Before
    fun setUp() = Dispatchers.setMain(dispatcher)

    @After
    fun tearDown() = Dispatchers.resetMain()

    private fun userConfig(authRequired: Boolean = false, lockMinutes: Int = 0, hideBalances: Boolean = false) =
        mockk<UserConfig>(relaxed = true) {
            every { isHideBalances() } returns flowOf(hideBalances)
            every { getLockInterval() } returns flowOf(lockMinutes)
            every { this@mockk.authRequired() } returns authRequired
        }

    private fun settings() = mockk<GemSettingsServiceInterface>(relaxed = true) {
        every { securitySections(any()) } answers {
            listOf(
                GemSecuritySection(listOfNotNull(GemSecurityRow.AUTHENTICATION, GemSecurityRow.LOCK_PERIOD.takeIf { firstArg() })),
                GemSecuritySection(listOf(GemSecurityRow.HIDE_BALANCE)),
            )
        }
    }

    @Test
    fun `the rows come from core with the authentication flag`() {
        val settings = settings()
        SecurityViewModel(userConfig(authRequired = true), settings)

        verify { settings.securitySections(true) }
    }

    @Test
    fun `the stored preferences are what the scene starts from`() = runTest(dispatcher) {
        val model = SecurityViewModel(userConfig(authRequired = true, lockMinutes = 5, hideBalances = true), settings())
        advanceUntilIdle()

        val rows = model.rows.value.flatten()
        assertEquals(SecurityRowUIModel.Authentication(true), rows[0])
        assertEquals(R.string.lock_five_minutes, (rows[1] as SecurityRowUIModel.LockPeriod).current)
        assertEquals(listOf(5), (rows[1] as SecurityRowUIModel.LockPeriod).options.filter { it.isSelected }.map { it.minutes })
        assertEquals(SecurityRowUIModel.HideBalance(true), rows[2])
    }

    @Test
    fun `changing the lock interval and the balance privacy writes through`() = runTest(dispatcher) {
        val config = userConfig()
        val model = SecurityViewModel(config, settings())

        model.setAuthRequired(true)
        model.setLockInterval(15)
        model.setHideBalances()
        advanceUntilIdle()
        Thread.sleep(50)
        advanceUntilIdle()

        verify { config.setAuthRequired(true) }
        coVerify { config.setLockInterval(15) }
        coVerify { config.hideBalances() }
    }
}
