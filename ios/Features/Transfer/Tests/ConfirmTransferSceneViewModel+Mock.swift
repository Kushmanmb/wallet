// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.AddressName
import enum Gemstone.GemConfirmDestination
import struct Gemstone.GemConfirmData
import struct Gemstone.GemConfirmLoad
import enum Gemstone.GemConfirmRowContent
import enum Gemstone.GemExecuteResult
import struct Gemstone.GemTransferData
import func Gemstone.walletRow
import GemstonePrimitives
import GemstonePrimitivesTestKit
import GemstoneServicesTestKit
import Primitives
import PrimitivesComponents
import StoreTestKit
@testable import Transfer
import TransferTestKit
import struct Gemstone.SimulationResult

@MainActor
extension ConfirmTransferSceneViewModel {
    static func mock(
        request: ConfirmTransferRequest? = nil,
        wallet: Wallet? = nil,
        data: GemTransferData = .mock(),
        simulation: SimulationResult? = nil,
        gemConfirmService: GemConfirmServiceMock = GemConfirmServiceMock(),
        load: Result<GemConfirmLoad, any Error> = .success(.mock()),
        execute: Result<GemExecuteResult, any Error> = .success(.signed(data: [])),
        rows: ((Gemstone.AddressName?) -> [GemConfirmRowContent])? = nil,
        onComplete: VoidAction = nil,
    ) -> ConfirmTransferSceneViewModel {
        let wallet = wallet ?? .mock(accounts: [.mock(chain: data.chain)])
        let rows = rows ?? { addressName in
            [
                .sender(wallet: walletRow(wallet: wallet.toGem())),
                .recipient(
                    destination: .recipient(name: addressName?.name, address: data.recipient.address),
                    addressName: addressName,
                    memo: data.recipient.memo,
                    chain: data.chain.rawValue,
                    link: BlockExplorerLink.mock().toGem(),
                ),
                .network(chain: data.chain.rawValue, name: data.chain.rawValue),
                data.recipient.memo.map { GemConfirmRowContent.memo(memo: $0) },
                .details,
            ].compactMap(\.self)
        }
        return ConfirmTransferSceneViewModel(
            request: request ?? ConfirmTransferRequest(data: data, simulation: simulation),
            wallet: wallet,
            confirmation: GemConfirmationMock(
                state: .mock(feeAsset: data.feeAsset().toPrimitives(), simulation: gemConfirmService.simulation, preload: nil),
                load: load,
                execute: execute,
                rows: rows,
            ),
            onComplete: onComplete,
        )
    }
}
