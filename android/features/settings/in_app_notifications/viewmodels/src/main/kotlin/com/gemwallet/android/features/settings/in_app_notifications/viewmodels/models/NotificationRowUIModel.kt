package com.gemwallet.android.features.settings.in_app_notifications.viewmodels.models

import com.gemwallet.android.ext.toAssetId
import com.gemwallet.android.ext.toGem
import com.gemwallet.android.ui.components.image.iconModel
import com.wallet.core.primitives.InAppNotification
import uniffi.gemstone.GemNotificationIcon
import uniffi.gemstone.notificationRow

data class NotificationRowUIModel(
    val id: String,
    val createdAt: Long,
    val title: String,
    val subtitle: String?,
    val value: String?,
    val subvalue: String?,
    val url: String?,
    val isUnread: Boolean,
    val icon: NotificationIconUIModel?,
)

sealed interface NotificationIconUIModel {
    data class Emoji(val glyph: String) : NotificationIconUIModel
    data class Image(val model: Any?) : NotificationIconUIModel
}

internal fun InAppNotification.uiModel(): NotificationRowUIModel {
    val row = notificationRow(toGem())
    return NotificationRowUIModel(
        id = item.id,
        createdAt = createdAt,
        title = row.title,
        subtitle = row.subtitle,
        value = row.value,
        subvalue = row.subvalue,
        url = row.url,
        isUnread = row.isUnread,
        icon = row.icon?.uiModel(),
    )
}

private fun GemNotificationIcon.uiModel(): NotificationIconUIModel = when (this) {
    is GemNotificationIcon.Emoji -> NotificationIconUIModel.Emoji(glyph)
    is GemNotificationIcon.Image -> NotificationIconUIModel.Image(url)
    is GemNotificationIcon.Asset -> NotificationIconUIModel.Image(assetId.toAssetId()?.iconModel())
}
