// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Formatters
import GemstonePrimitives
import enum Gemstone.GemSwapErrorDisplay
import Localization
import Primitives

extension GemSwapErrorDisplay {
    func message(asset: Asset?) -> String {
        switch self {
        case .notSupportedAsset:
            Localized.Errors.Swap.notSupportedAsset
        case .noQuote:
            Localized.Errors.Swap.noQuoteAvailable
        case let .minimumAmount(minAmount):
            if let asset {
                Localized.Errors.Swap.minimumAmount(
                    ValueFormatter(style: .auto).string(minAmount, decimals: asset.decimals.asInt, currency: asset.symbol).boldMarkdown()
                )
            } else {
                Localized.Errors.Swap.amountTooSmall
            }
        case .amountTooSmall:
            Localized.Errors.Swap.amountTooSmall
        }
    }

    func asError(asset: Asset?) -> any Error {
        AnyError(message(asset: asset))
    }
}
