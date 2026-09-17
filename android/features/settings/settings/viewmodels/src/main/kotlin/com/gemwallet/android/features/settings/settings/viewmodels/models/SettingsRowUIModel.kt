package com.gemwallet.android.features.settings.settings.viewmodels.models

import androidx.annotation.DrawableRes
import androidx.annotation.StringRes
import com.gemwallet.android.features.settings.settings.viewmodels.localization.stringRes
import com.gemwallet.android.features.settings.settings.viewmodels.style.icon
import com.gemwallet.android.ui.models.actions.SettingsSceneAction
import uniffi.gemstone.GemSettingsRow

data class SettingsRowUIModel(
    @StringRes val title: Int,
    @DrawableRes val icon: Int,
    val action: SettingsSceneAction,
    val trailing: String? = null,
    val opensDeveloperMenu: Boolean = false,
)

internal fun GemSettingsRow.uiModel(walletsCount: Int): SettingsRowUIModel = when (this) {
    GemSettingsRow.WALLETS -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.Wallets, trailing = walletsCount.toString())
    GemSettingsRow.SECURITY -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.Security)
    GemSettingsRow.NOTIFICATIONS -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.Notifications)
    GemSettingsRow.PREFERENCES -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.Preferences)
    GemSettingsRow.WALLET_CONNECT -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.Bridges)
    GemSettingsRow.SUPPORT -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.Support)
    GemSettingsRow.REWARDS -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.Referral)
    GemSettingsRow.ABOUT_US -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.AboutUs, opensDeveloperMenu = true)
    GemSettingsRow.DEVELOPER -> SettingsRowUIModel(stringRes(), icon(), SettingsSceneAction.Develop)
}
