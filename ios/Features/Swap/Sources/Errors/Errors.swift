// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import enum Gemstone.GemSwapErrorDisplay
import GemstonePrimitives
import Localization
import Primitives

public enum SwapQuoteInputError: Error {
    case invalidAmount
    case formattingError
    case missingFromAsset
    case missingToAsset
}

extension GemSwapErrorDisplay: @retroactive Error {}

extension GemSwapErrorDisplay: @retroactive LocalizedError {
    public var errorDescription: String? {
        switch self {
        case .notSupportedAsset: Localized.Errors.Swap.notSupportedAsset
        case .noQuote: Localized.Errors.Swap.noQuoteAvailable
        case let .minimumAmount(asset, minAmount):
            Localized.Errors.Swap.minimumAmount(
                ValueFormatter(style: .auto).string(minAmount, asset: asset.toPrimitives()).boldMarkdown()
            )
        case .amountTooSmall: Localized.Errors.Swap.amountTooSmall
        }
    }
}
