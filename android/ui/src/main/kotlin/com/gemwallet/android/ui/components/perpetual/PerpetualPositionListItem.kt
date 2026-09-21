package com.gemwallet.android.ui.components.perpetual

import android.content.Context
import com.gemwallet.android.domains.perpetual.aggregates.PerpetualPositionDataAggregate
import com.gemwallet.android.ui.components.list_item.ListItemImage
import com.gemwallet.android.ui.components.list_item.ListItemModel
import com.gemwallet.android.ui.components.list_item.ListItemTextStyle
import com.gemwallet.android.ui.localization.string
import com.gemwallet.android.ui.style.textStyle

fun PerpetualPositionDataAggregate.listItem(context: Context): ListItemModel = ListItemModel(
    title = title,
    titleExtra = positionLabel.string(context),
    titleExtraStyle = direction.textStyle(),
    subtitle = marginAmount,
    subtitleStyle = ListItemTextStyle.Body,
    subtitleExtra = pnl.string(context),
    subtitleExtraStyle = pnlState.textStyle(),
    image = ListItemImage.Asset(asset.id),
)
