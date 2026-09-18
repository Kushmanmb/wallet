// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemSimulationValue
import enum Gemstone.GemListRow
import PrimitivesComponents
@testable import Transfer

extension ConfirmSimulationState {
    static func mock(
        warnings: [GemListRow] = [],
        headerData: GemSimulationValue? = nil,
    ) -> ConfirmSimulationState {
        ConfirmSimulationState(
            result: nil,
            warnings: warnings,
            hasCriticalWarning: false,
            payload: SimulationPayloadModel(primaryFields: [], secondaryFields: []),
            headerData: headerData,
            balanceChanges: [],
        )
    }
}
