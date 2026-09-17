package com.gemwallet.android.features.activities.viewmodels.models

import android.content.Context
import com.gemwallet.android.domains.duration.formatEstimatedConfirmation
import com.gemwallet.android.domains.transaction.values.TransactionDetailsValue
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.InfoSheetEntity
import com.gemwallet.android.ui.components.list_item.ListItemModel
import com.gemwallet.android.ui.localization.stringRes
import com.gemwallet.android.ui.style.textStyle
import com.wallet.core.primitives.Chain

sealed interface TransactionDetailsRowUIModel {
    data class Item(val model: ListItemModel, val url: String? = null) : TransactionDetailsRowUIModel
    data class Value(val value: TransactionDetailsValue) : TransactionDetailsRowUIModel
}

internal fun TransactionDetailsValue.uiModel(context: Context, chain: Chain): TransactionDetailsRowUIModel = when (this) {
    is TransactionDetailsValue.Date -> TransactionDetailsRowUIModel.Item(ListItemModel(title = context.getString(R.string.transaction_date), subtitle = data))
    is TransactionDetailsValue.Memo -> TransactionDetailsRowUIModel.Item(ListItemModel(title = context.getString(R.string.transfer_memo), subtitle = data))
    is TransactionDetailsValue.ResourceType -> TransactionDetailsRowUIModel.Item(ListItemModel(title = context.getString(R.string.stake_resource), subtitle = context.getString(data.stringRes())))
    is TransactionDetailsValue.Pnl -> TransactionDetailsRowUIModel.Item(ListItemModel(title = context.getString(R.string.perpetual_pnl), subtitle = value, subtitleStyle = direction.textStyle()))
    is TransactionDetailsValue.Price -> TransactionDetailsRowUIModel.Item(ListItemModel(title = context.getString(R.string.asset_price), subtitle = data))
    is TransactionDetailsValue.EstimatedConfirmation -> TransactionDetailsRowUIModel.Item(
        ListItemModel(
            title = context.getString(R.string.transaction_estimated_confirmation),
            subtitle = formatEstimatedConfirmation(seconds),
            info = InfoSheetEntity.EstimatedConfirmationInfo(chain),
        ),
    )
    is TransactionDetailsValue.Explorer -> TransactionDetailsRowUIModel.Item(ListItemModel(title = context.getString(R.string.transaction_view_on, name)), url = url)
    is TransactionDetailsValue.Amount,
    is TransactionDetailsValue.Destination,
    is TransactionDetailsValue.Fee,
    is TransactionDetailsValue.Network,
    is TransactionDetailsValue.Status,
    is TransactionDetailsValue.Rate,
    is TransactionDetailsValue.SwapProgress,
    is TransactionDetailsValue.SwapAgain -> TransactionDetailsRowUIModel.Value(this)
}
