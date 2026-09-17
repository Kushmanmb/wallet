package com.gemwallet.android.ui.components.list_item

import androidx.annotation.DrawableRes
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.RowScope
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.Dp
import com.gemwallet.android.ui.components.InfoSheetEntity
import com.gemwallet.android.ui.components.image.ListItemImageView
import com.gemwallet.android.ui.components.list_item.property.PropertyDataText
import com.gemwallet.android.ui.components.list_item.property.PropertyItem
import com.gemwallet.android.ui.components.list_item.property.PropertyTitleText
import com.gemwallet.android.ui.models.ListPosition
import com.gemwallet.android.ui.theme.listItemIconSize
import com.wallet.core.primitives.AssetId

data class ListItemModel(
    val title: String,
    val titleTag: String? = null,
    val titleExtra: String? = null,
    val subtitle: String? = null,
    val subtitleExtra: String? = null,
    val image: ListItemImage? = null,
    val info: InfoSheetEntity? = null,
)

sealed interface ListItemImage {
    data class Asset(val assetId: AssetId) : ListItemImage
    data class Url(val url: String, val placeholder: String? = null) : ListItemImage
    data class Stored(val name: String, val placeholder: String? = null) : ListItemImage
    data class Emoji(val glyph: String, val backgroundColor: Int) : ListItemImage
    data class Initials(val text: String) : ListItemImage
    data class Drawable(@DrawableRes val id: Int) : ListItemImage
}

@Composable
fun ListItem(
    model: ListItemModel,
    listPosition: ListPosition,
    modifier: Modifier = Modifier,
    minHeight: Dp = Dp.Unspecified,
    trailing: (@Composable RowScope.() -> Unit)? = null,
) {
    if (model.image == null && model.titleExtra == null && model.titleTag == null && trailing == null) {
        PropertyItem(
            modifier = modifier,
            title = { PropertyTitleText(text = model.title, info = model.info) },
            data = model.subtitle?.let { { PropertyDataText(text = it) } },
            listPosition = listPosition,
        )
        return
    }
    ListItem(
        modifier = modifier,
        listPosition = listPosition,
        minHeight = minHeight,
        leading = model.image?.let { image -> { ListItemImageView(image = image, size = listItemIconSize) } },
        title = { ListItemTitleText(text = model.title, titleBadge = model.titleTag?.let { { Badge(it) } }) },
        subtitle = model.titleExtra?.let { { ListItemSupportText(it) } },
        trailing = trailing ?: model.trailingTexts(),
    )
}

private fun ListItemModel.trailingTexts(): (@Composable RowScope.() -> Unit)? {
    if (subtitle == null && subtitleExtra == null) {
        return null
    }
    return {
        Column(horizontalAlignment = Alignment.End) {
            subtitle?.let {
                Text(
                    text = it,
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.onSurface,
                    maxLines = 1,
                )
            }
            subtitleExtra?.let { ListItemSupportText(it) }
        }
    }
}
