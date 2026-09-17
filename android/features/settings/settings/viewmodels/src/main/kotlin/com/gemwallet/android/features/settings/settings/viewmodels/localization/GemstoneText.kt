package com.gemwallet.android.features.settings.settings.viewmodels.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.Appearance
import uniffi.gemstone.GemPreferencesRow

@StringRes
internal fun GemPreferencesRow.stringRes(): Int = when (this) {
    GemPreferencesRow.CURRENCY -> R.string.settings_currency
    GemPreferencesRow.LANGUAGE -> R.string.settings_language
    GemPreferencesRow.APPEARANCE -> R.string.settings_appearance_title
    GemPreferencesRow.NETWORKS -> R.string.settings_networks_title
    GemPreferencesRow.CONTACTS -> R.string.contacts_title
    GemPreferencesRow.PERPETUALS -> R.string.perpetuals_title
    GemPreferencesRow.PERPETUAL_LEVERAGE -> R.string.settings_preferences_perpetual_default_leverage
    GemPreferencesRow.PERPETUAL_TAKE_PROFIT -> R.string.settings_preferences_perpetual_default_take_profit
    GemPreferencesRow.PERPETUAL_STOP_LOSS -> R.string.settings_preferences_perpetual_default_stop_loss
}

@StringRes
fun Appearance.stringRes(): Int = when (this) {
    Appearance.System -> R.string.settings_appearance_system
    Appearance.Light -> R.string.settings_appearance_light
    Appearance.Dark -> R.string.settings_appearance_dark
}
