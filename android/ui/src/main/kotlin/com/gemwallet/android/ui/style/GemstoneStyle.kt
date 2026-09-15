package com.gemwallet.android.ui.style

import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.vector.ImageVector
import com.gemwallet.android.ui.icons.AppIcons
import uniffi.gemstone.GemHeaderButtonKind

@Composable
fun GemHeaderButtonKind.icon(): ImageVector = when (this) {
    GemHeaderButtonKind.SEND -> AppIcons.Send
    GemHeaderButtonKind.RECEIVE -> AppIcons.Receive
    GemHeaderButtonKind.BUY -> AppIcons.Buy
    GemHeaderButtonKind.SWAP -> AppIcons.SwapVert
    GemHeaderButtonKind.DEPOSIT -> AppIcons.Deposit
    GemHeaderButtonKind.WITHDRAW -> AppIcons.Withdraw
    GemHeaderButtonKind.MORE -> AppIcons.MoreVert
}
