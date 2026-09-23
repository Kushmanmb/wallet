// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemConfirmSimulation
import struct Gemstone.GemConfirmSimulationState
import enum Gemstone.GemListRow
import struct Gemstone.GemSimulationBalanceChange
import struct Gemstone.GemSimulationValue
import struct Gemstone.SimulationResult
import func Gemstone.simulationWarningRows
import GemstonePrimitives
import Primitives
import PrimitivesComponents

struct ConfirmSimulationState {
    let warnings: [GemListRow]
    let hasCriticalWarning: Bool
    let payload: SimulationPayloadModel
    let headerData: GemSimulationValue?
    let balanceChanges: [GemSimulationBalanceChange]

    init(
        warnings: [GemListRow],
        hasCriticalWarning: Bool,
        payload: SimulationPayloadModel,
        headerData: GemSimulationValue?,
        balanceChanges: [GemSimulationBalanceChange],
    ) {
        self.warnings = warnings
        self.hasCriticalWarning = hasCriticalWarning
        self.payload = payload
        self.headerData = headerData
        self.balanceChanges = balanceChanges
    }

    init(result: SimulationResult?) {
        self.init(
            warnings: simulationWarningRows(warnings: result?.warnings ?? []),
            hasCriticalWarning: false,
            payload: SimulationPayloadModel(primaryFields: [], secondaryFields: []),
            headerData: nil,
            balanceChanges: [],
        )
    }

    init(_ state: GemConfirmSimulationState) {
        let details = state.simulation
        let payload = SimulationPayloadModel(
            primaryFields: details?.primaryFields ?? [],
            secondaryFields: details?.secondaryFields ?? [],
        )
        self.init(
            warnings: state.warnings,
            hasCriticalWarning: details?.hasCriticalWarning ?? false,
            payload: payload,
            headerData: details?.header,
            balanceChanges: details?.balanceChanges ?? [],
        )
    }
}
