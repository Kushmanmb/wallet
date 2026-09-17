package com.gemwallet.android.features.activities.presents.details

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalUriHandler
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.domains.asset.chain
import com.gemwallet.android.domains.transaction.aggregates.TransactionDetailsAggregate
import com.gemwallet.android.domains.transaction.values.TransactionDetailsValue
import com.gemwallet.android.features.activities.presents.details.components.DestinationPropertyItem
import com.gemwallet.android.features.activities.presents.details.components.SwapProgressItem
import com.gemwallet.android.features.activities.presents.details.components.TransactionStatusProperty
import com.gemwallet.android.features.activities.viewmodels.models.TransactionDetailsRowUIModel
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.buttons.MainActionButton
import com.gemwallet.android.ui.components.list_head.AmountListHead
import com.gemwallet.android.ui.components.list_head.NftHead
import com.gemwallet.android.ui.components.list_head.SwapListHead
import com.gemwallet.android.ui.components.list_item.ListItem
import com.gemwallet.android.ui.components.list_item.listSections
import com.gemwallet.android.ui.components.list_item.property.AssetRatePropertyItem
import com.gemwallet.android.ui.components.list_item.property.DataBadgeChevron
import com.gemwallet.android.ui.components.list_item.property.PropertyNetworkFee
import com.gemwallet.android.ui.components.list_item.property.PropertyNetworkItem
import com.gemwallet.android.ui.components.screen.Scene
import com.gemwallet.android.ui.icons.AppIcons
import com.gemwallet.android.ui.localization.string
import com.gemwallet.android.ui.models.ListSection
import com.gemwallet.android.ui.open
import com.gemwallet.android.ui.theme.padding16
import com.gemwallet.android.ui.theme.paddingSmall
import com.wallet.core.primitives.AssetId
import com.wallet.core.primitives.NFTAssetId
import uniffi.gemstone.GemTransactionHeaderAction

@Composable
internal fun TransactionDetailsScene(
    data: TransactionDetailsAggregate,
    sections: List<ListSection<TransactionDetailsRowUIModel>>,
    onAction: (TransactionDetailsAction) -> Unit,
) {
    val uriHandler = LocalUriHandler.current
    val context = LocalContext.current
    Scene(
        title = data.title.string(LocalContext.current),
        actions = {
            IconButton(onClick = { onAction(TransactionDetailsAction.Share) }) {
                Icon(AppIcons.Share, "")
            }
        },
        onClose = { onAction(TransactionDetailsAction.Close) },
    ) {
        LazyColumn(modifier = Modifier.fillMaxSize()) {
            listSections(sections) { position, row ->
                when (row) {
                    is TransactionDetailsRowUIModel.Item -> ListItem(
                        model = row.model,
                        listPosition = position,
                        modifier = row.url?.let { url -> Modifier.clickable { uriHandler.open(context, url) } } ?: Modifier,
                        accessory = row.url?.let { { DataBadgeChevron() } },
                    )
                    is TransactionDetailsRowUIModel.Value -> when (val item = row.value) {
                        is TransactionDetailsValue.Amount.NFT -> NftHead(
                            metadata = item.metadata,
                            onClick = data.headerAction?.let { action -> { onAction(action.navigation()) } },
                        )
                        is TransactionDetailsValue.Amount.Plain -> AmountListHead(
                            icon = item.asset,
                            amount = item.value,
                            equivalent = item.equivalent,
                            onClick = data.headerAction?.let { action -> { onAction(action.navigation()) } },
                        )
                        is TransactionDetailsValue.Amount.Swap -> SwapListHead(
                            fromAsset = item.fromAsset,
                            fromValue = item.fromValue,
                            toAsset = item.toAsset,
                            toValue = item.toValue,
                            currency = item.currency,
                            onSwapClick = data.headerAction?.let { action -> { onAction(action.navigation()) } },
                            onAssetClick = { onAction(TransactionDetailsAction.OpenAsset(it)) },
                        )
                        is TransactionDetailsValue.Destination -> DestinationPropertyItem(item, position)
                        is TransactionDetailsValue.Fee -> PropertyNetworkFee(
                            networkTitle = item.asset.name,
                            networkSymbol = item.asset.symbol,
                            feeCrypto = item.value,
                            feeFiat = item.equivalent,
                            variantsAvailable = true,
                            onClick = { onAction(TransactionDetailsAction.ShowFeeDetails) },
                        )
                        is TransactionDetailsValue.Network -> PropertyNetworkItem(item.data.chain, listPosition = position)
                        is TransactionDetailsValue.Status -> TransactionStatusProperty(data.asset, item, position)
                        is TransactionDetailsValue.Date,
                        is TransactionDetailsValue.Explorer,
                        is TransactionDetailsValue.Memo,
                        is TransactionDetailsValue.ResourceType,
                        is TransactionDetailsValue.Pnl,
                        is TransactionDetailsValue.Price,
                        is TransactionDetailsValue.EstimatedConfirmation -> Unit
                        is TransactionDetailsValue.Rate -> AssetRatePropertyItem(item.rate, position)
                        is TransactionDetailsValue.SwapProgress -> SwapProgressItem(item)
                        is TransactionDetailsValue.SwapAgain -> MainActionButton(
                            title = stringResource(R.string.transaction_swap_again),
                            modifier = Modifier.padding(horizontal = padding16, vertical = paddingSmall),
                            onClick = {
                                onAction(
                                    TransactionDetailsAction.OpenSwap(
                                        fromAssetId = item.fromAssetId,
                                        toAssetId = item.toAssetId,
                                    )
                                )
                            },
                        )
                    }
                }
            }
        }
    }
}

private fun GemTransactionHeaderAction.navigation(): TransactionDetailsAction.Navigation = when (this) {
    is GemTransactionHeaderAction.Asset -> TransactionDetailsAction.OpenAsset(AssetId(assetId))
    is GemTransactionHeaderAction.Nft -> TransactionDetailsAction.OpenNft(NFTAssetId(assetId))
    is GemTransactionHeaderAction.Swap -> TransactionDetailsAction.OpenSwap(fromAssetId = AssetId(fromAssetId), toAssetId = AssetId(toAssetId))
    is GemTransactionHeaderAction.Perpetual -> TransactionDetailsAction.OpenPerpetual(AssetId(assetId))
}
