package com.gemwallet.android.features.settings.currency.viewmodels.models

import com.gemwallet.android.ext.toPrimitives
import com.wallet.core.primitives.Currency
import uniffi.gemstone.GemCurrencies
import uniffi.gemstone.GemCurrencyRow

data class CurrenciesUIModel(
    val recommended: List<CurrencyRowUIModel>,
    val other: List<CurrencyRowUIModel>,
)

data class CurrencyRowUIModel(
    val currency: Currency,
    val title: String,
    val isSelected: Boolean,
)

internal fun GemCurrencies.uiModel(): CurrenciesUIModel = CurrenciesUIModel(
    recommended = recommended.map { it.uiModel(selected) },
    other = other.map { it.uiModel(selected) },
)

private fun GemCurrencyRow.uiModel(selected: GemCurrencyRow): CurrencyRowUIModel {
    val code = currency.toPrimitives().string
    return CurrencyRowUIModel(
        currency = currency.toPrimitives(),
        title = "$flag  $code - ${android.icu.util.Currency.getInstance(code).displayName}",
        isSelected = currency == selected.currency,
    )
}
