package com.gemwallet.android.features.settings.settings.viewmodels.style

import androidx.annotation.DrawableRes
import com.gemwallet.android.ui.R
import uniffi.gemstone.GemPreferencesRow

@DrawableRes
internal fun GemPreferencesRow.icon(): Int? = when (this) {
    GemPreferencesRow.CURRENCY -> R.drawable.settings_currency
    GemPreferencesRow.LANGUAGE -> R.drawable.settings_language
    GemPreferencesRow.APPEARANCE -> R.drawable.settings_appearance
    GemPreferencesRow.NETWORKS -> R.drawable.settings_networks
    GemPreferencesRow.CONTACTS -> R.drawable.settings_contacts
    GemPreferencesRow.PERPETUALS -> R.drawable.settings_pricealert
    GemPreferencesRow.PERPETUAL_LEVERAGE,
    GemPreferencesRow.PERPETUAL_TAKE_PROFIT,
    GemPreferencesRow.PERPETUAL_STOP_LOSS -> null
}
