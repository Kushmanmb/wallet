// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemServiceEndpointType
import enum Gemstone.GemSettingsRow
import Localization
import Primitives

public extension Appearance {
    var title: String {
        switch self {
        case .system: Localized.Settings.appearanceSystem
        case .light: Localized.Settings.appearanceLight
        case .dark: Localized.Settings.appearanceDark
        }
    }
}

extension GemServiceEndpointType {
    var name: String {
        switch self {
        case .api: "API"
        case .gemNode: Localized.Nodes.gemWalletNode
        }
    }
}

extension GemSettingsRow {
    var title: String {
        switch self {
        case .wallets: Localized.Wallets.title
        case .security: Localized.Settings.security
        case .notifications: Localized.Settings.Notifications.title
        case .preferences: Localized.Settings.Preferences.title
        case .walletConnect: Localized.WalletConnect.title
        case .support: Localized.Settings.support
        case .rewards: Localized.Rewards.title
        case .aboutUs: Localized.Settings.aboutus
        case .developer: Localized.Settings.developer
        }
    }
}
