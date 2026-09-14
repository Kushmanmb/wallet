// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemFormattedNumber
import enum Gemstone.GemNumberDisplay
import enum Gemstone.GemNumberUnit
import enum Gemstone.GemPrecision

public extension GemFormattedNumber {
    static func mock(
        value: Double = 1,
        unit: GemNumberUnit = .currency(code: "USD"),
        display: GemNumberDisplay = .number(precision: .fraction(min: 2, max: 2)),
        showsSign: Bool = true,
    ) -> GemFormattedNumber {
        GemFormattedNumber(value: value, unit: unit, display: display, showsSign: showsSign)
    }
}
