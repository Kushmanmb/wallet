// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemChartSection
import Localization

extension GemChartSection {
    var title: String? {
        switch self {
        case .priceAlerts: Localized.Settings.PriceAlerts.title
        case .setPriceAlert: Localized.PriceAlerts.SetAlert.title
        case .links: Localized.Social.links
        case .market: nil
        }
    }
}
