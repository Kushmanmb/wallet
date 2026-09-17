package com.gemwallet.android.ui.components.perpetual

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.list_item.ListItem
import com.gemwallet.android.ui.components.list_item.ListItemModel
import com.gemwallet.android.ui.models.ListPosition
import uniffi.gemstone.GemPerpetual
import uniffi.gemstone.PerpetualProvider

@Composable
fun AutocloseSummaryRow(
    takeProfitText: String?,
    stopLossText: String?,
    listPosition: ListPosition = ListPosition.Single,
) {
    val lines = listOfNotNull(
        takeProfitText?.let { perpetual.triggerOrderText(stringResource(R.string.perpetual_take_profit), it) },
        stopLossText?.let { perpetual.triggerOrderText(stringResource(R.string.perpetual_stop_loss), it) },
    )
    if (lines.isEmpty()) return
    ListItem(
        model = ListItemModel(
            title = stringResource(R.string.perpetual_auto_close),
            subtitle = lines.first(),
            subtitleExtra = lines.getOrNull(1),
        ),
        listPosition = listPosition,
    )
}

private val perpetual = GemPerpetual(PerpetualProvider.HYPERCORE)
