// Copyright (c). Gem Wallet. All rights reserved.

import Foundation

extension NumberFormatStyleConfiguration.Precision {
    static let twoPlaces = fractionLength(2)
    static let upToTwoPlaces = fractionLength(0 ... 2)
    static let upToFourPlaces = fractionLength(0 ... 4)
    static let fourSignificant = significantDigits(1 ... 4)
    static let full = fractionLength(0 ... 32)
}
