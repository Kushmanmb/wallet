// Copyright (c). Gem Wallet. All rights reserved.

import Components
import GemstonePrimitives
@testable import Perpetuals
import PerpetualsTestKit
import Primitives
import PrimitivesTestKit
import Style
import Testing

struct PerpetualItemViewModelTests {
    @Test
    func itemsKeepTheirMarketsInOrder() {
        let markets: [PerpetualData] = [.mock(perpetual: .mock(name: "BTC-PERP")), .mock(perpetual: .mock(name: "ETH-PERP"))]

        #expect(PerpetualItemViewModel.items(markets).map(\.model.name) == ["BTC-PERP", "ETH-PERP"])
        #expect(PerpetualItemViewModel.items(markets).map(\.data) == markets)
    }

    @Test
    func volumeSitsOnTheRight() {
        guard case let .balance(balance, _) = model(.mock(volume24h: 1_500_000)).rightView else {
            Issue.record("a market always shows its volume")
            return
        }
        #expect(balance.text == "$1.5M")
    }

    @Test
    func priceAndChangeFormTheSubtitle() {
        guard case let .price(price, change) = model(.mock(price: 45000, pricePercentChange24h: -2.5)).subtitleView else {
            Issue.record("a priced market shows its price")
            return
        }
        #expect(price.text == "$45,000.00")
        #expect(change.text == "-2.50%")
        #expect(change.style.color == Colors.red)
    }

    @Test
    func unpricedMarketShowsNoSubtitle() {
        guard case .none = model(.mock(price: 0)).subtitleView else {
            Issue.record("a market with no price shows none")
            return
        }
    }

    private func model(_ perpetual: Perpetual) -> PerpetualItemViewModel {
        PerpetualItemViewModel.items([.mock(perpetual: perpetual)])[0].model
    }
}
