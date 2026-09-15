package com.gemwallet.android.features.settings.settings.presents.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.Appearance
import uniffi.gemstone.GemSettingsRow

@StringRes
internal fun Appearance.stringRes(): Int = when (this) {
    Appearance.System -> R.string.settings_appearance_system
    Appearance.Light -> R.string.settings_appearance_light
    Appearance.Dark -> R.string.settings_appearance_dark
}

@StringRes
internal fun GemSettingsRow.stringRes(): Int = when (this) {
    GemSettingsRow.WALLETS -> R.string.wallets_title
    GemSettingsRow.SECURITY -> R.string.settings_security
    GemSettingsRow.NOTIFICATIONS -> R.string.settings_notifications_title
    GemSettingsRow.PREFERENCES -> R.string.settings_preferences_title
    GemSettingsRow.WALLET_CONNECT -> R.string.wallet_connect_title
    GemSettingsRow.SUPPORT -> R.string.settings_support
    GemSettingsRow.REWARDS -> R.string.rewards_title
    GemSettingsRow.ABOUT_US -> R.string.settings_aboutus
    GemSettingsRow.DEVELOPER -> R.string.settings_developer
}
