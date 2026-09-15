package com.gemwallet.android.ui.components

import androidx.annotation.DrawableRes
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.theme.pendingColor
import uniffi.gemstone.GemTransactionStateTone

@DrawableRes
fun GemTransactionStateTone.badgeIconRes(): Int = when (this) {
    GemTransactionStateTone.PENDING -> R.drawable.transaction_state_pending
    GemTransactionStateTone.SUCCESS -> R.drawable.transaction_state_success
    GemTransactionStateTone.ERROR,
    GemTransactionStateTone.REFUNDED -> R.drawable.transaction_state_error
}

@Composable
fun GemTransactionStateTone.color(): Color = when (this) {
    GemTransactionStateTone.PENDING,
    GemTransactionStateTone.REFUNDED -> pendingColor
    GemTransactionStateTone.SUCCESS -> MaterialTheme.colorScheme.tertiary
    GemTransactionStateTone.ERROR -> MaterialTheme.colorScheme.error
}
