package com.gemwallet.android.features.add_asset.viewmodels.models

import android.content.Context
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.list_item.ListItemImage
import com.gemwallet.android.ui.components.list_item.ListItemModel
import com.gemwallet.android.ui.theme.Emoji

internal fun searchFailedListItem(context: Context): ListItemModel = ListItemModel(
    title = context.getString(R.string.errors_error_occurred),
    titleExtra = context.getString(R.string.errors_token_invalid_id),
    image = ListItemImage.Emoji(Emoji.warning),
)

internal fun verificationWarningListItem(context: Context): ListItemModel = ListItemModel(
    title = context.getString(R.string.asset_verification_warning_title),
    titleExtra = context.getString(R.string.asset_verification_warning_message),
    image = ListItemImage.Emoji(Emoji.warning),
)
