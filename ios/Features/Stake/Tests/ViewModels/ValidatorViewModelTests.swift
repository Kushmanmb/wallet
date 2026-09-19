// Copyright (c). Gem Wallet. All rights reserved.

import GemstonePrimitives
import GemstonePrimitivesTestKit
import Primitives
import PrimitivesTestKit
import Stake
import Testing

struct ValidatorViewModelTests {
    @Test func aprText() {
        #expect(ValidatorViewModel(row: .mock(validator: DelegationValidator.mock(apr: 2.15).toGem())).aprText == "APR 2.15%")
    }
}
