// Copyright (c). Gem Wallet. All rights reserved.

import Localization
import Primitives

extension FiatQuoteType {
    func title(asset: String) -> String {
        switch self {
        case .buy: Localized.Buy.title(asset)
        case .sell: Localized.Sell.title(asset)
        }
    }

    var action: String {
        switch self {
        case .buy: Localized.Wallet.buy
        case .sell: Localized.Wallet.sell
        }
    }
}
