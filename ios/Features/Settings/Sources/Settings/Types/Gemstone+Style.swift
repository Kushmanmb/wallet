// Copyright (c). Gem Wallet. All rights reserved.

import Components
import enum Gemstone.GemNodeSyncState
import enum Gemstone.GemPreferencesRow
import Style

extension GemPreferencesRow {
    var assetImage: AssetImage {
        switch self {
        case .currency: AssetImage.image(Images.Settings.currency)
        case .language: AssetImage.image(Images.Settings.language)
        case .appearance: AssetImage.image(Images.Settings.appearance)
        case .networks: AssetImage.image(Images.Settings.networks)
        case .contacts: AssetImage.image(Images.Settings.contacts)
        case .perpetuals: AssetImage.image(Images.Settings.perpetuals)
        case .perpetualLeverage, .perpetualTakeProfit, .perpetualStopLoss: AssetImage()
        }
    }
}

extension GemNodeSyncState {
    var symbol: String {
        switch self {
        case .inSync: Emoji.checkmark
        case .outOfSync: Emoji.reject
        }
    }
}
