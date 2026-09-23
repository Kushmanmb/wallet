// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import Primitives
import PrimitivesTestKit
@testable import Stake
import Testing

struct DelegationViewModelTests {
    @Test
    func itemsFormatEachDelegationWithTheAssetPrice() {
        let delegations: [Delegation] = [
            .mock(base: .mock(state: .active, assetId: .mock(.tron), balance: 1_500_000_000, delegationId: "1")),
            .mock(base: .mock(state: .active, assetId: .mock(.tron), balance: 500_000_000, delegationId: "2")),
        ]

        let items = DelegationViewModel.items(delegations, asset: Chain.tron.asset, price: 2.0, currency: .usd)

        #expect(items.map(\.delegation) == delegations)
        #expect(items.map(\.model.balanceText) == ["1,500 TRX", "500 TRX"])
        #expect(items.map(\.model.fiatValueText) == ["$3,000.00", "$1,000.00"])
    }
}
