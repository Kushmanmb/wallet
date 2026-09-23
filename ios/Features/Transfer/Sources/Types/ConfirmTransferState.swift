// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import enum Gemstone.GemConfirmError
import struct Gemstone.GemConfirmFee
import struct Gemstone.GemConfirmLoad
import struct Gemstone.GemConfirmMetadata
import struct Gemstone.GemConfirmScreen
import struct Gemstone.GemFeeAsset
import struct Gemstone.GemTransferData
import struct Gemstone.PaymentVerification
import Primitives
import PrimitivesComponents

struct ConfirmTransferState {
    var transfer: GemTransferData
    var feeAsset: Asset
    var load: GemConfirmLoad?
    var simulation: ConfirmSimulationState
    var screen: GemConfirmScreen

    var metadata: GemConfirmMetadata? { load?.metadata }
    var feeAssets: [GemFeeAsset] { load?.feeAssets ?? [] }
    var fee: GemConfirmFee? { load?.fee }
    var addressName: AddressName? { load?.addressName.map { $0.toPrimitives() } }
}

extension ConfirmTransferState {
    init(transfer: GemTransferData, simulation: ConfirmSimulationState, screen: GemConfirmScreen) {
        self.init(
            transfer: transfer,
            feeAsset: transfer.feeAsset().toPrimitives(),
            load: nil,
            simulation: simulation,
            screen: screen,
        )
    }

    init(_ load: GemConfirmLoad, screen: GemConfirmScreen) {
        self.init(
            transfer: load.transfer,
            feeAsset: load.feeAsset.toPrimitives(),
            load: load,
            simulation: ConfirmSimulationState(load.simulation),
            screen: screen,
        )
    }

    var transferAmount: TransferAmountValidation? {
        fee?.amount.toPrimitives()
    }

    var verification: PaymentVerification? {
        transfer.verification()
    }

    var transactionError: ConfirmTransferError? {
        if let failure = screen.failure, failure.stage == .load {
            return ConfirmTransferError(error: failure.error)
        }
        if case let .failure(error)? = transferAmount {
            return ConfirmTransferError(error: error)
        }
        return nil
    }
}
