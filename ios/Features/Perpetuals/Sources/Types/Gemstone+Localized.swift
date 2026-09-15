// Copyright (c). Gem Wallet. All rights reserved.

import Localization
import Primitives

extension TpslType {
    var autocloseTitle: String {
        switch self {
        case .takeProfit: Localized.Perpetual.AutoClose.takeProfit
        case .stopLoss: Localized.Perpetual.AutoClose.stopLoss
        }
    }
}
