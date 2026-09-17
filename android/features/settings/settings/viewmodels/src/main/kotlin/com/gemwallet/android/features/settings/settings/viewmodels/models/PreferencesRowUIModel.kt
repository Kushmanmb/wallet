package com.gemwallet.android.features.settings.settings.viewmodels.models

import androidx.annotation.DrawableRes
import androidx.annotation.StringRes
import com.gemwallet.android.features.settings.settings.viewmodels.localization.stringRes
import com.gemwallet.android.features.settings.settings.viewmodels.style.icon
import com.gemwallet.android.ui.models.actions.PreferencesAction
import com.wallet.core.primitives.Appearance
import uniffi.gemstone.GemPreferencesRow
import uniffi.gemstone.GemPreferencesState

sealed interface PreferencesRowUIModel {
    @get:StringRes val title: Int

    data class Link(@StringRes override val title: Int, @DrawableRes val icon: Int?, val trailing: String?, val action: PreferencesAction) : PreferencesRowUIModel
    data class Language(@StringRes override val title: Int, @DrawableRes val icon: Int?) : PreferencesRowUIModel
    data class AppearancePicker(@StringRes override val title: Int, @DrawableRes val icon: Int?, val current: Appearance) : PreferencesRowUIModel
    data class PerpetualsSwitch(@StringRes override val title: Int, @DrawableRes val icon: Int?, val isEnabled: Boolean) : PreferencesRowUIModel
    data class Picker(@StringRes override val title: Int, val setting: PerpetualSetting, val current: Int, val options: List<PickerOption>) : PreferencesRowUIModel
}

enum class PerpetualSetting { Leverage, TakeProfit, StopLoss }

data class PickerOption(val value: Int, val label: String)

data class PerpetualOptions(
    val leverage: List<PickerOption>,
    val takeProfit: List<PickerOption>,
    val stopLoss: List<PickerOption>,
)

data class PerpetualValues(val isEnabled: Boolean, val leverage: Int, val takeProfit: Int, val stopLoss: Int)

internal fun GemPreferencesRow.uiModel(
    state: GemPreferencesState,
    appearance: Appearance,
    perpetual: PerpetualValues,
    options: PerpetualOptions,
): PreferencesRowUIModel = when (this) {
    GemPreferencesRow.CURRENCY -> PreferencesRowUIModel.Link(stringRes(), icon(), state.currency.text(), PreferencesAction.Currencies)
    GemPreferencesRow.LANGUAGE -> PreferencesRowUIModel.Language(stringRes(), icon())
    GemPreferencesRow.APPEARANCE -> PreferencesRowUIModel.AppearancePicker(stringRes(), icon(), appearance)
    GemPreferencesRow.NETWORKS -> PreferencesRowUIModel.Link(stringRes(), icon(), null, PreferencesAction.Networks)
    GemPreferencesRow.CONTACTS -> PreferencesRowUIModel.Link(stringRes(), icon(), null, PreferencesAction.Contacts)
    GemPreferencesRow.PERPETUALS -> PreferencesRowUIModel.PerpetualsSwitch(stringRes(), icon(), perpetual.isEnabled)
    GemPreferencesRow.PERPETUAL_LEVERAGE -> PreferencesRowUIModel.Picker(stringRes(), PerpetualSetting.Leverage, perpetual.leverage, options.leverage)
    GemPreferencesRow.PERPETUAL_TAKE_PROFIT -> PreferencesRowUIModel.Picker(stringRes(), PerpetualSetting.TakeProfit, perpetual.takeProfit, options.takeProfit)
    GemPreferencesRow.PERPETUAL_STOP_LOSS -> PreferencesRowUIModel.Picker(stringRes(), PerpetualSetting.StopLoss, perpetual.stopLoss, options.stopLoss)
}
