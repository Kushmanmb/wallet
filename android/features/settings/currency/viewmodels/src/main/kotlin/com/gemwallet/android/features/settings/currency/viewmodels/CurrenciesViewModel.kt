package com.gemwallet.android.features.settings.currency.viewmodels

import android.content.Context
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.gemwallet.android.application.session.cases.GetCurrentCurrency
import com.gemwallet.android.application.session.cases.SetCurrentCurrency
import com.gemwallet.android.ext.toGem
import com.gemwallet.android.features.settings.currency.viewmodels.models.sections
import com.wallet.core.primitives.Currency
import dagger.hilt.android.lifecycle.HiltViewModel
import dagger.hilt.android.qualifiers.ApplicationContext
import java.util.Locale
import javax.inject.Inject
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.mapLatest
import kotlinx.coroutines.flow.stateIn
import uniffi.gemstone.GemCurrencyServiceInterface
import com.gemwallet.android.ext.errorText
import com.gemwallet.android.ext.runCatchingCancellable
import com.gemwallet.android.ui.localization.text
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

@OptIn(ExperimentalCoroutinesApi::class)
@HiltViewModel
class CurrenciesViewModel @Inject constructor(
    private val service: GemCurrencyServiceInterface,
    getCurrentCurrency: GetCurrentCurrency,
    private val setCurrentCurrency: SetCurrentCurrency,
    @param:ApplicationContext private val context: Context,
) : ViewModel() {
    private val localeCurrency: Currency? = runCatching { java.util.Currency.getInstance(Locale.getDefault()).currencyCode }
        .getOrNull()
        ?.let { Currency.entries.firstOrNull { currency -> currency.string == it } }

    private val currency = getCurrentCurrency.getCurrency()

    val sections = currency.mapLatest { service.currencies(localeCurrency?.toGem()).sections(context) }
        .stateIn(viewModelScope, SharingStarted.Eagerly, emptyList())

    private val errorState = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = errorState.asStateFlow()

    fun setCurrency(currency: Currency, onSelected: () -> Unit) = viewModelScope.launch {
        runCatchingCancellable { setCurrentCurrency.setCurrentCurrency(currency) }
            .onSuccess { onSelected() }
            .onFailure { errorState.value = it.errorText().text(context) }
    }

    fun clearError() = errorState.update { null }
}
