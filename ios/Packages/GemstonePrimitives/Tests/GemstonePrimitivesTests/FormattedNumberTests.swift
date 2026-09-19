// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Foundation
import func Gemstone.formattedAmount
@testable import GemstonePrimitives
import Testing

struct FormattedNumberTests {
    @Test
    func anAmountReadsLikeTheValueFormatter() throws {
        let formatter = ValueFormatter(locale: .US, style: .auto)
        let values: [(BigInt, Int)] = [(5_205_516, 6), (99999, 6), (1992, 4), (1_239_999_000_000, 6), (546, 8)]

        for (value, decimals) in values {
            let number = try formattedAmount(value: formatter.double(from: value, decimals: decimals), symbol: "ATOM", style: .auto)
            #expect(number.text(locale: .US) == formatter.string(value, decimals: decimals, currency: "ATOM"))
        }
    }
}
