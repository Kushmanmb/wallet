// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemServiceEndpointType
import enum Gemstone.GemAboutRow
import enum Gemstone.GemPreferencesRow
import enum Gemstone.GemSettingsRow
import enum Gemstone.GemChainSettingsSection
import enum Gemstone.GemNodeSubtitle
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

extension GemPreferencesRow {
    var title: String {
        switch self {
        case .currency: Localized.Settings.currency
        case .language: Localized.Settings.language
        case .appearance: Localized.Settings.appearanceTitle
        case .networks: Localized.Settings.Networks.title
        case .contacts: Localized.Contacts.title
        case .perpetuals: Localized.Perpetuals.title
        case .perpetualLeverage: Localized.Settings.Preferences.Perpetual.defaultLeverage
        case .perpetualTakeProfit: Localized.Settings.Preferences.Perpetual.defaultTakeProfit
        case .perpetualStopLoss: Localized.Settings.Preferences.Perpetual.defaultStopLoss
        }
    }
}

extension GemAboutRow {
    var title: String {
        switch self {
        case .termsOfService: Localized.Settings.termsOfServices
        case .privacyPolicy: Localized.Settings.privacyPolicy
        case .website: Localized.Settings.website
        case .community: Localized.Settings.community
        case .version: Localized.Settings.version
        }
    }
}

extension GemChainSettingsSection {
    var title: String {
        switch self {
        case .nodes: Localized.Settings.Networks.source
        case .explorer: Localized.Settings.Networks.explorer
        }
    }
}

extension GemNodeSubtitle {
    var title: String {
        switch self {
        case .latestBlock: Localized.Nodes.ImportNode.latestBlock
        }
    }
}
