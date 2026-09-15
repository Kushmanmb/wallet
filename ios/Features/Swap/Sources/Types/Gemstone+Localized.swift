// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemSwapDetailRow
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

extension GemSwapDetailRow {
    var title: String {
        switch self {
        case .provider: Localized.Common.provider
        case .rate: Localized.Buy.rate
        case .estimatedTime: Localized.Swap.EstimatedTime.title
        case .priceImpact: Localized.Swap.priceImpact
        case .minimumReceive: Localized.Swap.minReceive
        case .slippage: Localized.Swap.slippage
        }
    }
}
