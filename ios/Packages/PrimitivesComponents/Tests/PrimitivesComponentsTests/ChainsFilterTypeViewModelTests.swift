// Copyright (c). Gem Wallet. All rights reserved.

import Localization
import Primitives
@testable import PrimitivesComponents
import Testing

struct ChainsFilterTypeViewModelTests {
    @Test
    func noSelectionReadsAsAll() {
        let model = ChainsFilterViewModel(chains: [], selected: []).typeModel

        #expect(model.value == Localized.Common.all)
    }

    @Test
    func oneChainReadsAsItsName() {
        let model = ChainsFilterViewModel(chains: [], selected: [.ethereum]).typeModel

        #expect(model.value == "Ethereum")
    }

    @Test
    func severalChainsReadAsACount() {
        let model = ChainsFilterViewModel(chains: [], selected: [.ethereum, .bitcoin, .solana]).typeModel

        #expect(model.value == "3")
    }
}
