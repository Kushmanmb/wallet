package com.gemwallet.android.ui.components

import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import com.gemwallet.android.ui.icons.AppIcons
import com.gemwallet.android.ui.theme.pendingColor
import uniffi.gemstone.GemVerificationLevel
import uniffi.gemstone.WalletConnectionVerificationStatus
import uniffi.gemstone.verificationLevel

@Composable
fun WalletConnectionVerificationStatus.icon(): ImageVector = when (verificationLevel(this)) {
    GemVerificationLevel.VERIFIED -> AppIcons.Verified
    GemVerificationLevel.UNVERIFIED, GemVerificationLevel.SUSPICIOUS -> AppIcons.Warning
}

@Composable
fun WalletConnectionVerificationStatus.color(): Color = when (verificationLevel(this)) {
    GemVerificationLevel.VERIFIED -> MaterialTheme.colorScheme.tertiary
    GemVerificationLevel.UNVERIFIED -> pendingColor
    GemVerificationLevel.SUSPICIOUS -> MaterialTheme.colorScheme.error
}
