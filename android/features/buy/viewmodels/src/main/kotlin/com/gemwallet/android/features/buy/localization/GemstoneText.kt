package com.gemwallet.android.features.buy.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.FiatQuoteType
import uniffi.gemstone.GemFiatButtonAction

@StringRes
fun FiatQuoteType.titleRes(): Int = when (this) {
    FiatQuoteType.Buy -> R.string.buy_title
    FiatQuoteType.Sell -> R.string.sell_title
}

@StringRes
fun FiatQuoteType.actionRes(): Int = when (this) {
    FiatQuoteType.Buy -> R.string.wallet_buy
    FiatQuoteType.Sell -> R.string.wallet_sell
}

@StringRes
fun GemFiatButtonAction.stringRes(): Int = when (this) {
    GemFiatButtonAction.CONTINUE -> R.string.common_continue
    GemFiatButtonAction.RETRY_QUOTE -> R.string.common_try_again
}
