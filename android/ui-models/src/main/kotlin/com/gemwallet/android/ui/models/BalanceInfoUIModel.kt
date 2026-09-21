package com.gemwallet.android.ui.models

import com.gemwallet.android.model.AssetInfo
import com.gemwallet.android.model.Crypto
import com.gemwallet.android.model.CryptoFiatConverter
import com.gemwallet.android.model.ValueFormatter
import com.wallet.core.primitives.Asset
import com.wallet.core.primitives.Currency
import uniffi.gemstone.GemValueStyle
import java.math.BigInteger

open class BalanceInfoUIModel(override val asset: Asset, private val balance: BigInteger, val price: Double?, override val currency: Currency) :
    CryptoFormattedUIModel,
    FiatFormattedUIModel {

    override val cryptoAmount: Double by lazy { Crypto(balance).value(asset.decimals).toDouble() }

    override val fiat: Double? by lazy { CryptoFiatConverter.fiatValue(Crypto(balance), asset.decimals, price) }
}

class RewardsInfoUIModel(assetInfo: AssetInfo, balance: BigInteger) :
    BalanceInfoUIModel(
        asset = assetInfo.asset,
        balance = balance,
        price = assetInfo.price?.price?.price,
        currency = assetInfo.price?.currency ?: Currency.USD,
    ) {
    override val cryptoFormatted: String by lazy { ValueFormatter(style = GemValueStyle.AUTO).string(balance, asset) }
}
