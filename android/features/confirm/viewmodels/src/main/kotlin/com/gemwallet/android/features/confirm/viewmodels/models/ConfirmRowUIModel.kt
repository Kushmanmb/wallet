package com.gemwallet.android.features.confirm.viewmodels.models

import android.content.Context
import com.gemwallet.android.domains.confirm.ConfirmProperty
import com.gemwallet.android.features.confirm.viewmodels.localization.titleRes
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.list_item.ListItemModel
import com.gemwallet.android.ui.localization.stringRes

sealed interface ConfirmRowUIModel {
    data class Item(val model: ListItemModel) : ConfirmRowUIModel
    data class Property(val property: ConfirmProperty) : ConfirmRowUIModel
}

internal fun ConfirmProperty.uiModel(context: Context): ConfirmRowUIModel = when (this) {
    is ConfirmProperty.Memo -> ConfirmRowUIModel.Item(ListItemModel(title = context.getString(R.string.transfer_memo), subtitle = data))
    is ConfirmProperty.Destination.Provider -> ConfirmRowUIModel.Item(ListItemModel(title = context.getString(titleRes()), subtitle = data))
    is ConfirmProperty.Destination.Generic -> ConfirmRowUIModel.Item(ListItemModel(title = context.getString(titleRes()), subtitle = appName))
    is ConfirmProperty.Destination.Resource -> ConfirmRowUIModel.Item(ListItemModel(title = context.getString(titleRes()), subtitle = context.getString(resource.stringRes())))
    is ConfirmProperty.Destination.Stake -> if (address != null && explorerLink != null) {
        ConfirmRowUIModel.Property(this)
    } else {
        ConfirmRowUIModel.Item(ListItemModel(title = context.getString(titleRes()), subtitle = data))
    }
    is ConfirmProperty.Destination.Transfer,
    is ConfirmProperty.Destination.Contract,
    is ConfirmProperty.Source,
    is ConfirmProperty.Network -> ConfirmRowUIModel.Property(this)
}
