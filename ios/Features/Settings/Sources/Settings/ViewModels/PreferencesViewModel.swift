// Copyright (c). Gem Wallet. All rights reserved.

import Components
import struct Gemstone.GemPreferencesSection
import protocol Gemstone.GemSettingsServiceProtocol
import Foundation
import protocol Gemstone.GemPreferencesServiceProtocol
import GemstonePrimitives
import Localization
import GemstoneServices
import Primitives
import PrimitivesComponents
import Style
import SwiftUI

@Observable
@MainActor
public final class PreferencesViewModel {
    private let preferences: ObservablePreferences
    private let service: any GemPreferencesServiceProtocol
    private let settings: any GemSettingsServiceProtocol
    private let currencyModel: CurrencySceneViewModel

    var isPresentingLeveragePicker = false
    var isPresentingTakeProfitPicker = false
    var isPresentingStopLossPicker = false

    public init(
        currencyModel: CurrencySceneViewModel,
        service: any GemPreferencesServiceProtocol,
        settings: any GemSettingsServiceProtocol,
        preferences: ObservablePreferences,
    ) {
        self.currencyModel = currencyModel
        self.service = service
        self.settings = settings
        self.preferences = preferences
        perpetualLeverage = LeverageOption(value: service.getPerpetualLeverage())
        perpetualTakeProfit = AutocloseOption(value: service.getPerpetualTakeProfitPercent())
        perpetualStopLoss = AutocloseOption(value: service.getPerpetualStopLossPercent())
    }

    var sections: [GemPreferencesSection] {
        settings.preferencesSections(perpetualsEnabled: isPerpetualEnabled)
    }

    var title: String {
        Localized.Settings.Preferences.title
    }


    var currencyValue: String {
        currencyModel.selectedCurrencyValue
    }



    var languageValue: String {
        guard let code = Locale.current.language.languageCode?.identifier else {
            return ""
        }
        return Locale.current.localizedString(forLanguageCode: code)?.capitalized ?? ""
    }








    var appearanceValue: String {
        preferences.appearance.title
    }

    var isPerpetualEnabled: Bool {
        get { preferences.isPerpetualEnabled }
        set { preferences.isPerpetualEnabled = newValue }
    }



    var perpetualLeverage: LeverageOption {
        didSet { persist { try service.setPerpetualLeverage(leverage: perpetualLeverage.value) } }
    }


    var defaultLeverageValue: String {
        "\(perpetualLeverage.value)x"
    }

    var leverageOptions: [LeverageOption] {
        LeverageOption.allOptions
    }

    var perpetualTakeProfit: AutocloseOption {
        didSet { persist { try service.setPerpetualTakeProfitPercent(percent: perpetualTakeProfit.value) } }
    }

    var perpetualStopLoss: AutocloseOption {
        didSet { persist { try service.setPerpetualStopLossPercent(percent: perpetualStopLoss.value) } }
    }

    private func persist(_ write: () throws -> Void) {
        do {
            try write()
        } catch {
            debugLog("preferences write error: \(error)")
        }
    }



    var defaultTakeProfitValue: String {
        perpetualTakeProfit.displayText
    }

    var defaultStopLossValue: String {
        perpetualStopLoss.displayText
    }

    var takeProfitOptions: [AutocloseOption] {
        AutocloseOption.takeProfitOptions
    }

    var stopLossOptions: [AutocloseOption] {
        AutocloseOption.stopLossOptions
    }
}

// MARK: - Actions

extension PreferencesViewModel {
    func onSelectLeverage() {
        isPresentingLeveragePicker = true
    }

    func onSelectTakeProfit() {
        isPresentingTakeProfitPicker = true
    }

    func onSelectStopLoss() {
        isPresentingStopLossPicker = true
    }
}
