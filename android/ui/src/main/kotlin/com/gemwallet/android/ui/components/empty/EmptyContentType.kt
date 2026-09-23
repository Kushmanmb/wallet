package com.gemwallet.android.ui.components.empty

import uniffi.gemstone.GemEmptyStateAction
import uniffi.gemstone.GemEmptyStateKind

class EmptyContentType(val kind: GemEmptyStateKind, val symbol: String = "", val isViewOnly: Boolean = false, actions: Map<GemEmptyStateAction, (() -> Unit)?> = emptyMap()) {
    val actions: Map<GemEmptyStateAction, () -> Unit> = actions.mapNotNull { (action, onClick) -> onClick?.let { action to it } }.toMap()
}
