package com.gemwallet.android.features.settings.settings.viewmodels

import com.gemwallet.android.ext.toGem
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.gemwallet.android.application.device.cases.GetPushEnabled
import com.gemwallet.android.application.device.cases.SwitchPushEnabled
import com.gemwallet.android.data.services.gemstone.config.UserConfig
import com.gemwallet.android.application.session.cases.GetSession
import com.gemwallet.android.application.wallet.cases.GetWallets
import com.gemwallet.android.model.NotificationsAvailable
import com.wallet.core.primitives.Appearance
import com.wallet.core.primitives.WalletType
import dagger.hilt.android.lifecycle.HiltViewModel
import uniffi.gemstone.GemSettingsServiceInterface
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import javax.inject.Inject
import uniffi.gemstone.GemCurrencyRow
import uniffi.gemstone.GemCurrencyServiceInterface

@HiltViewModel
class SettingsViewModel @Inject constructor(
    private val userConfig: UserConfig,
    private val getWallets: GetWallets,
    private val getSession: GetSession,
    private val currencyService: GemCurrencyServiceInterface,
    private val switchPushEnabled: SwitchPushEnabled,
    private val getPushEnabled: GetPushEnabled,
    val notificationsAvailable: NotificationsAvailable,
    private val settingsService: GemSettingsServiceInterface,
) : ViewModel() {

    private val session = getSession()
    private val wallets = getWallets()
    private val state = MutableStateFlow(SettingsViewModelState(currency = currencyService.currencies(null).selected))
    val uiState = state.asStateFlow()

    private val walletConnectAvailable = MutableStateFlow(true)

    val sections = combine(wallets, state, walletConnectAvailable) { wallets, _, walletConnect ->
        settingsService.sections(
            wallets = wallets.map { it.toGem() },
            notificationsAvailable = notificationsAvailable,
            walletConnectAvailable = walletConnect,
        )
    }
        .stateIn(viewModelScope, SharingStarted.Eagerly, emptyList())

    fun setWalletConnectAvailable(available: Boolean) {
        walletConnectAvailable.value = available
    }

    val walletsCount = wallets.map { it.size }
        .stateIn(viewModelScope, SharingStarted.Eagerly, 0)

    val pushEnabled = getPushEnabled.getPushEnabled()
        .stateIn(viewModelScope, SharingStarted.Eagerly, true)

    val isPerpetualEnabled = userConfig.isPerpetualEnabled()
        .stateIn(viewModelScope, SharingStarted.Eagerly, false)

    val preferencesSections = isPerpetualEnabled
        .map { enabled -> settingsService.preferencesSections(enabled) }
        .stateIn(viewModelScope, SharingStarted.Eagerly, settingsService.preferencesSections(false))

    val appearance = userConfig.appearance()
        .stateIn(viewModelScope, SharingStarted.Eagerly, Appearance.System)

    fun setAppearance(appearance: Appearance) = viewModelScope.launch(Dispatchers.IO) {
        userConfig.setAppearance(appearance)
    }

    val perpetualLeverage = userConfig.perpetualLeverage()
        .stateIn(viewModelScope, SharingStarted.Eagerly, userConfig.perpetualLeverage().value)

    fun setPerpetualEnabled(enabled: Boolean) = viewModelScope.launch(Dispatchers.IO) {
        userConfig.setPerpetualEnabled(enabled)
    }

    fun setPerpetualLeverage(value: Int) = viewModelScope.launch(Dispatchers.IO) {
        userConfig.setPerpetualLeverage(value)
    }

    val perpetualTakeProfit = userConfig.perpetualTakeProfit()
        .stateIn(viewModelScope, SharingStarted.Eagerly, userConfig.perpetualTakeProfit().value)

    fun setPerpetualTakeProfit(value: Int) = viewModelScope.launch(Dispatchers.IO) {
        userConfig.setPerpetualTakeProfit(value)
    }

    val perpetualStopLoss = userConfig.perpetualStopLoss()
        .stateIn(viewModelScope, SharingStarted.Eagerly, userConfig.perpetualStopLoss().value)

    fun setPerpetualStopLoss(value: Int) = viewModelScope.launch(Dispatchers.IO) {
        userConfig.setPerpetualStopLoss(value)
    }

    init {
        viewModelScope.launch {
            session.collectLatest {
                refresh()
            }
        }
        refresh()
    }

    private fun refresh() = viewModelScope.launch(Dispatchers.IO) {
        state.update {
            it.copy(
                currency = currencyService.currencies(null).selected,
                developEnabled = userConfig.developEnabled(),
            )
        }
    }

    fun developEnable() {
        userConfig.developEnabled(!userConfig.developEnabled())
        refresh()
    }

    fun enableNotifications() {
        viewModelScope.launch(Dispatchers.IO) {
            userConfig.stopAskNotifications()
            switchPushEnabled.switchPushEnabled(true)
        }
    }

    fun disableNotifications() {
        viewModelScope.launch(Dispatchers.IO) {
            userConfig.stopAskNotifications()
            switchPushEnabled.switchPushEnabled(false)
        }
    }

}

data class SettingsViewModelState(
    val currency: GemCurrencyRow,
    val developEnabled: Boolean = false,
)
