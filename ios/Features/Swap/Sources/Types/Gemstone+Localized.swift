// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemSwapButtonAction
import Localization

extension GemSwapButtonAction {
    func title(symbol: String) -> String {
        switch self {
        case .retryQuote, .retryTransfer: Localized.Common.tryAgain
        case .insufficientBalance: Localized.Transfer.insufficientBalance(symbol)
        case .useMinimumAmount: Localized.Swap.useMinimumAmount
        case .swap: Localized.Wallet.swap
        }
    }
}
