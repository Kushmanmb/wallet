package com.gemwallet.android.domains.asset.aggregates

import androidx.compose.runtime.Immutable
import com.gemwallet.android.domains.price.values.PriceValue
import com.gemwallet.android.domains.price.values.RowFormatters
import com.gemwallet.android.ext.toGem
import com.gemwallet.android.model.AssetInfo
import com.wallet.core.primitives.Asset
import com.wallet.core.primitives.AssetId
import uniffi.gemstone.GemAssetRowStyle
import uniffi.gemstone.assetRowText
import java.math.BigDecimal

@Immutable
data class AssetInfoDataAggregate(
    val id: AssetId,
    val asset: Asset,
    val title: String,
    val symbol: String?,
    val network: String?,
    val balance: String,
    val balanceEquivalent: String,
    val isZeroBalance: Boolean,
    val price: PriceValue?,
    val pinned: Boolean,
    val balanceEnabled: Boolean,
    val accountAddress: String,
)

fun List<AssetInfo>.toAssetInfoDataAggregates(style: GemAssetRowStyle, hideBalance: Boolean = false): List<AssetInfoDataAggregate> {
    val formatters = RowFormatters()
    return map { it.toAssetInfoDataAggregate(style = style, hideBalance = hideBalance, formatters = formatters) }
}

fun AssetInfo.toAssetInfoDataAggregate(style: GemAssetRowStyle, hideBalance: Boolean = false, displayedAmount: Double = balance.totalAmount, formatters: RowFormatters = RowFormatters()): AssetInfoDataAggregate {
    val assetPrice = price?.price
    val priceValue = assetPrice?.price?.takeIf(Double::isFinite)
    val changePercentage = assetPrice?.priceChangePercentage24h?.takeIf(Double::isFinite)
    val formattedBalance = if (hideBalance) {
        "*****"
    } else {
        formatters.value.string(BigDecimal.valueOf(displayedAmount), asset.symbol)
    }
    val balanceEquivalent = if (hideBalance) {
        "*****"
    } else {
        price?.let { info ->
            priceValue
                ?.takeUnless { it == 0.0 }
                ?.let { formatters.currency(info.currency).string(displayedAmount * it) }
        }.orEmpty()
    }

    val text = assetRowText(asset.toGem(), style)

    return AssetInfoDataAggregate(
        id = asset.id,
        asset = asset,
        title = text.title,
        symbol = text.symbol,
        network = text.network,
        balance = formattedBalance,
        balanceEquivalent = balanceEquivalent,
        isZeroBalance = displayedAmount == 0.0,
        price = price?.let { formatters.price(it.currency, priceValue, changePercentage) },
        pinned = metadata.isPinned,
        balanceEnabled = metadata.isBalanceEnabled,
        accountAddress = owner?.address.orEmpty(),
    )
}
