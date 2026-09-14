// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import enum Gemstone.GemAmountError
import GemstonePrimitives
import Localization
import Primitives

extension GemAmountError: @retroactive LocalizedError {
    public var errorDescription: String? {
        switch display() {
        case .none: nil
        case .invalidAmount: Localized.Errors.invalidAmount
        case let .belowMinimum(asset, minimum):
            Localized.Transfer.minimumAmount(ValueFormatter(style: .auto).string(minimum, asset: asset.toPrimitives()).boldMarkdown())
        case let .insufficientBalance(title):
            Localized.Transfer.insufficientBalance(title.boldMarkdown())
        }
    }
}
