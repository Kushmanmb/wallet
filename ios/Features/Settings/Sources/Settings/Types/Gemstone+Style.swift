// Copyright (c). Gem Wallet. All rights reserved.

import Components
import enum Gemstone.GemSettingsRow
import Style

extension GemSettingsRow {
    var assetImage: AssetImage {
        switch self {
        case .wallets: AssetImage.image(Images.Settings.wallets)
        case .security: AssetImage.image(Images.Settings.security)
        case .notifications: AssetImage.image(Images.Settings.notifications)
        case .preferences: AssetImage.image(Images.Settings.preferences)
        case .walletConnect: AssetImage.image(Images.Settings.walletConnect)
        case .support: AssetImage.image(Images.Settings.support)
        case .rewards: AssetImage.image(Images.Settings.gem)
        case .aboutUs: AssetImage.image(Images.Settings.aboutUs)
        case .developer: AssetImage.image(Images.Settings.developer)
        }
    }
}
