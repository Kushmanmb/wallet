package com.gemwallet.android.ui.components.empty

import android.content.Context
import androidx.annotation.DrawableRes
import com.gemwallet.android.ui.localization.text
import com.gemwallet.android.ui.localization.title
import com.gemwallet.android.ui.style.image
import uniffi.gemstone.GemEmptyStateInput
import uniffi.gemstone.emptyState

data class EmptyStateUIModel(val title: String, val description: String?, val image: EmptyStateImage, val buttons: List<EmptyAction>)

sealed interface EmptyStateImage {
    @JvmInline value class Drawable(@DrawableRes val id: Int) : EmptyStateImage

    @JvmInline value class Vector(@DrawableRes val id: Int) : EmptyStateImage
}

fun EmptyContentType.uiModel(context: Context): EmptyStateUIModel {
    val state = emptyState(
        GemEmptyStateInput(
            kind = kind,
            isViewOnly = isViewOnly,
            offeredActions = actions.keys.toList(),
        ),
    )
    return EmptyStateUIModel(
        title = state.title.text(context, symbol),
        description = state.description?.text(context, symbol),
        image = state.image.image(),
        buttons = state.actions.mapIndexedNotNull { index, action ->
            actions[action]?.let {
                EmptyAction(
                    title = context.getString(action.title()),
                    onClick = it,
                    style = if (index == 0) EmptyActionStyle.Primary else EmptyActionStyle.Secondary,
                )
            }
        },
    )
}
