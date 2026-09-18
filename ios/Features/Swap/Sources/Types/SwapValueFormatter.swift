// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Formatters
import GemstonePrimitives

public struct SwapValueFormatter {
    private let formatter: ValueFormatter

    public init(valueFormatter: ValueFormatter) {
        formatter = valueFormatter
    }

    public func format(value: BigInt, decimals: Int) -> String {
        formatter.string(value, decimals: decimals)
    }
}
