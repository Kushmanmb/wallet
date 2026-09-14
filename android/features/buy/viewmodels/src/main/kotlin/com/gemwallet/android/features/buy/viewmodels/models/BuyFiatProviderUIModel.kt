package com.gemwallet.android.features.buy.viewmodels.models

import androidx.compose.runtime.Stable
import com.gemwallet.android.ext.toPrimitives
import com.gemwallet.android.model.CurrencyFormatter
import com.gemwallet.android.model.text
import com.gemwallet.android.ui.models.CryptoFormattedUIModel
import com.wallet.core.primitives.Asset
import com.wallet.core.primitives.Currency
import com.wallet.core.primitives.FiatProviderName
import uniffi.gemstone.GemFiatQuoteRow

@Stable
data class BuyFiatProviderUIModel(
    val row: GemFiatQuoteRow,
    override val asset: Asset,
    val currency: Currency,
) : CryptoFormattedUIModel {

    val provider: FiatProviderName by lazy { row.provider.toPrimitives() }

    val providerName: String get() = row.providerName

    val providerImageUrl: String? get() = row.providerImageUrl

    override val cryptoAmount: Double get() = row.cryptoAmount.value

    override val cryptoFormatted: String by lazy { "≈ $cryptoText" }

    val cryptoText: String by lazy { row.cryptoAmount.text() }

    val fiatFormatted: String by lazy { row.fiatAmount.text() }

    val rate: String by lazy {
        row.rate?.let { it.text(fiatFormatter.string(it.value)) }.orEmpty()
    }

    private val fiatFormatter: CurrencyFormatter
        get() = CurrencyFormatter(type = CurrencyFormatter.Type.Fiat, currency = currency)
}

fun GemFiatQuoteRow.toProviderUIModel(asset: Asset, currency: Currency): BuyFiatProviderUIModel =
    BuyFiatProviderUIModel(row = this, asset = asset, currency = currency)
