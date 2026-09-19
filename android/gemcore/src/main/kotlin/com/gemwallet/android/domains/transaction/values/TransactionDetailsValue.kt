package com.gemwallet.android.domains.transaction.values

import com.gemwallet.android.domains.swap.AssetRatePair
import com.gemwallet.android.model.AssetPriceValue
import com.gemwallet.android.model.Crypto
import com.gemwallet.android.model.CryptoFiatConverter
import com.gemwallet.android.model.ValueFormatter
import com.wallet.core.primitives.AddressType
import com.wallet.core.primitives.Asset
import com.wallet.core.primitives.AssetId
import com.wallet.core.primitives.BlockExplorerLink
import com.wallet.core.primitives.Chain
import com.wallet.core.primitives.Currency
import com.wallet.core.primitives.TransactionNFTTransferMetadata
import uniffi.gemstone.GemListRow
import uniffi.gemstone.GemSwapProgressState
import uniffi.gemstone.GemValueStyle
import java.math.BigInteger

sealed interface TransactionDetailsValue {

    sealed interface Amount : TransactionDetailsValue {
        class Swap(val fromAsset: AssetPriceValue, val fromValue: BigInteger, val toAsset: AssetPriceValue, val toValue: BigInteger, val currency: Currency) : Amount {
            val fromValueText: String get() = ValueFormatter(style = GemValueStyle.AUTO).string(fromValue, fromAsset.asset)
            val toValueText: String get() = ValueFormatter(style = GemValueStyle.AUTO).string(toValue, toAsset.asset)
            val fromEquivalentText: String? get() = fromAsset.price?.price?.price?.let { CryptoFiatConverter.toFiatString(Crypto(fromValue), fromAsset.asset.decimals, it, currency) }
            val toEquivalentText: String? get() = toAsset.price?.price?.price?.let { CryptoFiatConverter.toFiatString(Crypto(toValue), toAsset.asset.decimals, it, currency) }
        }

        class NFT(val metadata: TransactionNFTTransferMetadata) : Amount

        class Plain(val asset: Asset, val value: String, val equivalent: String?) : Amount
    }

    class Fee(val asset: Asset, val value: String, val equivalent: String) : TransactionDetailsValue

    sealed class Destination(val data: String, val chain: Chain? = null, val name: String? = null, val addressType: AddressType? = null, val explorerLink: BlockExplorerLink? = null) : TransactionDetailsValue {
        class Sender(data: String, chain: Chain, name: String? = null, addressType: AddressType? = null, explorerLink: BlockExplorerLink? = null) : Destination(data, chain, name, addressType, explorerLink)
        class Recipient(data: String, chain: Chain, name: String? = null, addressType: AddressType? = null, explorerLink: BlockExplorerLink? = null) : Destination(data, chain, name, addressType, explorerLink)
        class Contract(data: String, chain: Chain, name: String? = null, explorerLink: BlockExplorerLink? = null) : Destination(data, chain = chain, name = name, explorerLink = explorerLink)
        class Validator(data: String, chain: Chain, name: String? = null, explorerLink: BlockExplorerLink? = null) : Destination(data, chain = chain, name = name, explorerLink = explorerLink)
        class ProviderAddress(data: String, chain: Chain, name: String? = null, explorerLink: BlockExplorerLink? = null) : Destination(data, chain = chain, name = name, explorerLink = explorerLink)
    }

    class EstimatedConfirmation(val seconds: UInt) : TransactionDetailsValue

    class Rate(val rate: AssetRatePair) : TransactionDetailsValue

    class SwapProgress(val fromAsset: Asset, val fromValue: BigInteger, val providerName: String, val transfer: GemSwapProgressState, val swap: GemSwapProgressState, val etaInSeconds: UInt?) : TransactionDetailsValue

    class SwapAgain(val fromAssetId: AssetId, val toAssetId: AssetId) : TransactionDetailsValue

    class Row(val row: GemListRow) : TransactionDetailsValue
}
