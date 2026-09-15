package com.gemwallet.android.features.settings.settings.presents.style

import androidx.annotation.DrawableRes
import com.gemwallet.android.features.settings.settings.presents.views.SettingsSceneAction
import com.gemwallet.android.ui.R
import uniffi.gemstone.GemSettingsRow

@DrawableRes
internal fun GemSettingsRow.icon(): Int = when (this) {
    GemSettingsRow.WALLETS -> R.drawable.settings_wallets
    GemSettingsRow.SECURITY -> R.drawable.settings_security
    GemSettingsRow.NOTIFICATIONS -> R.drawable.settings_notifications
    GemSettingsRow.PREFERENCES -> R.drawable.settings_preferences
    GemSettingsRow.WALLET_CONNECT -> R.drawable.settings_wc
    GemSettingsRow.SUPPORT -> R.drawable.settings_support
    GemSettingsRow.REWARDS -> R.drawable.settings_wallets
    GemSettingsRow.ABOUT_US -> R.drawable.settings_about_us
    GemSettingsRow.DEVELOPER -> R.drawable.settings_developer
}

internal fun GemSettingsRow.action(): SettingsSceneAction = when (this) {
    GemSettingsRow.WALLETS -> SettingsSceneAction.Wallets
    GemSettingsRow.SECURITY -> SettingsSceneAction.Security
    GemSettingsRow.NOTIFICATIONS -> SettingsSceneAction.Notifications
    GemSettingsRow.PREFERENCES -> SettingsSceneAction.Preferences
    GemSettingsRow.WALLET_CONNECT -> SettingsSceneAction.Bridges
    GemSettingsRow.SUPPORT -> SettingsSceneAction.Support
    GemSettingsRow.REWARDS -> SettingsSceneAction.Referral
    GemSettingsRow.ABOUT_US -> SettingsSceneAction.AboutUs
    GemSettingsRow.DEVELOPER -> SettingsSceneAction.Develop
}
