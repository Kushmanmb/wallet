// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemSlippageSelection
import protocol Gemstone.GemSwapQuoteServiceProtocol
import GemstoneServicesTestKit
import Primitives
import Swap

public extension SwapSlippageViewModel {
    static func mock(
        service: any GemSwapQuoteServiceProtocol = GemSwapQuoteServiceMock(),
        slippage: GemSlippageSelection = .auto,
        onSelect: @escaping (GemSlippageSelection) -> Void = { _ in },
    ) -> SwapSlippageViewModel {
        SwapSlippageViewModel(service: service, chain: .ethereum, slippage: slippage, onSelect: onSelect)
    }
}
