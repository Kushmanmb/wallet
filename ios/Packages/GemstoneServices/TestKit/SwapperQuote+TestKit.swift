// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Foundation
import struct Gemstone.SwapperQuote
import enum Gemstone.SwapProvider
import struct Gemstone.SwapQuote
import func Gemstone.swapQuote

public extension SwapperQuote {
    var swapQuote: SwapQuote {
        Gemstone.swapQuote(quote: self)
    }

    static func mock(
        fromValue: BigUInt = 1_000_000_000_000_000_000,
        minFromValue: BigUInt? = nil,
        toValue: BigUInt = 250_000_000_000,
        provider: SwapProvider = .pancakeswapV3,
        etaInSeconds: UInt32? = nil,
    ) -> SwapperQuote {
        SwapperQuote(
            fromValue: fromValue,
            minFromValue: minFromValue,
            toValue: toValue,
            data: .mock(provider: provider),
            request: .mock(),
            etaInSeconds: etaInSeconds,
        )
    }
}
