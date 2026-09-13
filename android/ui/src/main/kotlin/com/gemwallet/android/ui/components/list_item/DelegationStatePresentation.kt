package com.gemwallet.android.ui.components.list_item

import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import com.gemwallet.android.ui.theme.pendingColor
import uniffi.gemstone.GemDelegationTone

@Composable
fun GemDelegationTone.color(): Color = when (this) {
    GemDelegationTone.POSITIVE -> MaterialTheme.colorScheme.tertiary
    GemDelegationTone.PENDING -> pendingColor
    GemDelegationTone.NEGATIVE -> MaterialTheme.colorScheme.error
}
