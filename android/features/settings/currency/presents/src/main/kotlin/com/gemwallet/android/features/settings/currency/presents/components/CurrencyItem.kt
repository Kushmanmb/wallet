package com.gemwallet.android.features.settings.currency.presents.components

import androidx.compose.foundation.clickable
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.gemwallet.android.features.settings.currency.viewmodels.models.CurrencyRowUIModel
import com.gemwallet.android.ui.components.list_item.ListItem
import com.gemwallet.android.ui.components.list_item.ListItemDefaults
import com.gemwallet.android.ui.components.list_item.ListItemTitleText
import com.gemwallet.android.ui.components.list_item.SelectionCheckmark
import com.gemwallet.android.ui.models.ListPosition
import com.wallet.core.primitives.Currency

@Composable
fun CurrencyItem(
    row: CurrencyRowUIModel,
    listPosition: ListPosition,
    onSelect: (Currency) -> Unit,
) {
    ListItem(
        modifier = Modifier.clickable { onSelect(row.currency) },
        minHeight = ListItemDefaults.plainMinHeight,
        title = { ListItemTitleText(row.title) },
        listPosition = listPosition,
        trailing = if (row.isSelected) {
            @Composable { SelectionCheckmark() }
        } else {
            null
        },
    )
}
