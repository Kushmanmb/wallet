package com.gemwallet.android.ui.components.list_item

import androidx.annotation.DrawableRes
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.Dp
import com.gemwallet.android.ui.components.InfoSheetEntity
import com.gemwallet.android.ui.components.image.ListItemImageView
import com.gemwallet.android.ui.components.list_item.property.PropertyDataText
import com.gemwallet.android.ui.components.list_item.property.PropertyItem
import com.gemwallet.android.ui.components.list_item.property.PropertyTitleText
import com.gemwallet.android.ui.models.ListPosition
import com.gemwallet.android.ui.theme.alpha10
import com.gemwallet.android.ui.theme.listItemIconSize
import com.gemwallet.android.ui.theme.space2
import com.gemwallet.android.ui.theme.space6
import com.wallet.core.primitives.AssetId

data class ListItemModel(
    val title: String,
    val titleStyle: ListItemTextStyle = ListItemTextStyle.Body,
    val titleTag: String? = null,
    val titleTagStyle: ListItemTextStyle = ListItemTextStyle.Secondary,
    val titleExtra: String? = null,
    val subtitle: String? = null,
    val subtitleStyle: ListItemTextStyle = ListItemTextStyle.Secondary,
    val subtitleExtra: String? = null,
    val image: ListItemImage? = null,
    val info: InfoSheetEntity? = null,
)

enum class ListItemTextStyle {
    Body,
    Secondary,
    Positive,
    Negative,
    Primary,
}

sealed interface ListItemImage {
    data class Asset(val assetId: AssetId) : ListItemImage
    data class Url(val url: String, val placeholder: String? = null) : ListItemImage
    data class Stored(val name: String, val placeholder: String? = null) : ListItemImage
    data class Emoji(val glyph: String, val backgroundColor: Int? = null) : ListItemImage
    data class Initials(val text: String) : ListItemImage
    data class Drawable(@DrawableRes val id: Int) : ListItemImage
}

@Composable
fun ListItemTextStyle.color(): Color = when (this) {
    ListItemTextStyle.Body -> MaterialTheme.colorScheme.onSurface
    ListItemTextStyle.Secondary -> MaterialTheme.colorScheme.secondary
    ListItemTextStyle.Positive -> MaterialTheme.colorScheme.tertiary
    ListItemTextStyle.Negative -> MaterialTheme.colorScheme.error
    ListItemTextStyle.Primary -> MaterialTheme.colorScheme.primary
}

@Composable
fun ListItem(
    model: ListItemModel,
    listPosition: ListPosition,
    modifier: Modifier = Modifier,
    minHeight: Dp = Dp.Unspecified,
    accessory: (@Composable () -> Unit)? = null,
) {
    if (model.image == null && model.titleExtra == null && model.titleTag == null && model.subtitleExtra == null) {
        PropertyItem(
            modifier = modifier,
            title = { PropertyTitleText(text = model.title, color = model.titleStyle.color(), info = model.info) },
            data = if (model.subtitle == null && accessory == null) {
                null
            } else {
                { PropertyDataText(text = model.subtitle ?: "", color = model.subtitleStyle.color(), badge = accessory) }
            },
            listPosition = listPosition,
        )
        return
    }
    ListItem(
        modifier = modifier,
        listPosition = listPosition,
        minHeight = minHeight,
        leading = model.image?.let { image -> { ListItemImageView(image = image, size = listItemIconSize) } },
        title = { ListItemTitleText(text = model.title, color = model.titleStyle.color(), titleBadge = model.titleTag?.let { { TitleTag(it, model.titleTagStyle) } }) },
        subtitle = model.titleExtra?.let { { ListItemSupportText(it) } },
        trailing = if (model.subtitle == null && model.subtitleExtra == null && accessory == null) {
            null
        } else {
            {
                if (model.subtitle != null || model.subtitleExtra != null) {
                    Column(horizontalAlignment = Alignment.End) {
                        model.subtitle?.let {
                            Text(
                                text = it,
                                style = MaterialTheme.typography.bodyLarge,
                                color = model.subtitleStyle.color(),
                                maxLines = 1,
                            )
                        }
                        model.subtitleExtra?.let { ListItemSupportText(it) }
                    }
                }
                accessory?.invoke()
            }
        },
    )
}

@Composable
private fun TitleTag(text: String, style: ListItemTextStyle) {
    when (style) {
        ListItemTextStyle.Primary -> Text(
            modifier = Modifier
                .background(
                    color = MaterialTheme.colorScheme.primary.copy(alpha = alpha10),
                    shape = RoundedCornerShape(space6),
                )
                .padding(horizontal = space6, vertical = space2),
            text = text,
            color = MaterialTheme.colorScheme.primary,
            style = MaterialTheme.typography.bodyMedium,
        )
        ListItemTextStyle.Body,
        ListItemTextStyle.Secondary,
        ListItemTextStyle.Positive,
        ListItemTextStyle.Negative -> Badge(text)
    }
}
