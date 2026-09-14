// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemSwapErrorDisplay
import Primitives
import PrimitivesTestKit
@testable import Swap
import Testing

struct GemSwapErrorDisplayTests {
    @Test
    func minimumAmountMessage() {
        #expect(
            GemSwapErrorDisplay.minimumAmount(minAmount: 120_966_091_866_986).message(asset: .mockBNB()) ==
                "Minimum trade amount is **0.0001209 BNB**. Please enter a higher amount.",
        )
        #expect(
            GemSwapErrorDisplay.minimumAmount(minAmount: 123_456).message(asset: .mock(symbol: "USDT", decimals: 6)) ==
                "Minimum trade amount is **0.1234 USDT**. Please enter a higher amount.",
        )
    }

    @Test
    func minimumAmountWithoutAnAssetFallsBackToTheShortMessage() {
        #expect(GemSwapErrorDisplay.minimumAmount(minAmount: 123_456).message(asset: nil) == "Amount too small")
    }

    @Test
    func userFacingMessages() {
        let asset = Asset.mockBNB()

        #expect(GemSwapErrorDisplay.notSupportedAsset.message(asset: asset) == "Not supported asset.")
        #expect(GemSwapErrorDisplay.noQuote.message(asset: asset) == "No quote available.")
        #expect(GemSwapErrorDisplay.amountTooSmall.message(asset: asset) == "Amount too small")
    }
}
