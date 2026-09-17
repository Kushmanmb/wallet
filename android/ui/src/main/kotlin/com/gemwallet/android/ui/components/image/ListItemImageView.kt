package com.gemwallet.android.ui.components.image

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.Dp
import com.gemwallet.android.ui.components.list_item.ListItemImage
import com.gemwallet.android.ui.icons.AppIcons
import com.gemwallet.android.ui.theme.iconSize

@Composable
fun ListItemImageView(
    image: ListItemImage,
    size: Dp,
    modifier: Modifier = Modifier,
) {
    when (image) {
        is ListItemImage.Asset -> AsyncImage(model = image.assetId.iconModel(), modifier = modifier, size = size)
        is ListItemImage.Url -> AsyncImage(model = image.url, modifier = modifier, size = size, placeholderText = image.placeholder)
        is ListItemImage.Stored -> AsyncImage(
            model = walletImageModel(LocalContext.current, image.name),
            modifier = modifier,
            size = size,
            placeholderText = image.placeholder,
        )
        is ListItemImage.Emoji -> EmojiView(
            emoji = image.glyph,
            modifier = modifier.size(size),
            background = image.backgroundColor?.let { Color(it) } ?: Color.Transparent,
            scale = AvatarScale.EMOJI,
        )
        is ListItemImage.Initials -> InitialsAvatar(text = image.text, size = size, modifier = modifier, placeholder = AppIcons.Person)
        is ListItemImage.Icon -> Icon(
            imageVector = image.vector,
            contentDescription = null,
            modifier = modifier.size(size),
            tint = MaterialTheme.colorScheme.onSurface,
        )
        is ListItemImage.Drawable -> Image(
            painter = painterResource(image.id),
            contentDescription = null,
            modifier = if (image.isRounded) modifier.size(size).clip(CircleShape) else modifier.size(iconSize),
        )
    }
}
