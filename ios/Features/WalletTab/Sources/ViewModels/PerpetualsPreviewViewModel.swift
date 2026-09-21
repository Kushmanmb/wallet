// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import struct Gemstone.GemPerpetualBalanceHeader
import func Gemstone.perpetualBalanceHeader
import GemstonePrimitives
import Localization
import Primitives
import Store

@Observable
@MainActor
final class PerpetualsPreviewViewModel {
    private let walletType: WalletType

    let positionsQuery: ObservableQuery<PerpetualPositionsRequest>
    let walletBalanceQuery: ObservableQuery<PerpetualWalletBalanceRequest>

    var positions: [PerpetualPositionData] {
        positionsQuery.value
    }

    var balanceHeader: GemPerpetualBalanceHeader {
        perpetualBalanceHeader(balance: walletBalanceQuery.value?.balance.toGem(), walletType: walletType.toGem())
    }

    init(walletId: WalletId, walletType: WalletType) {
        self.walletType = walletType
        positionsQuery = ObservableQuery(PerpetualPositionsRequest(walletId: walletId), initialValue: [])
        walletBalanceQuery = ObservableQuery(
            PerpetualWalletBalanceRequest(walletId: walletId, assetId: Chain.hyperCore.defaultAsset(type: .perpetual).id),
            initialValue: nil,
        )
    }

    var tradePerpetualsTitle: String {
        Localized.Perpetuals.trade
    }

    var tradePerpetualsSubtitle: String {
        balanceHeader.total.text()
    }

    var hasNoPositions: Bool {
        positions.isEmpty
    }

    func updateWallet(walletId: WalletId) {
        positionsQuery.request = PerpetualPositionsRequest(walletId: walletId)
        walletBalanceQuery.request = PerpetualWalletBalanceRequest(
            walletId: walletId,
            assetId: Chain.hyperCore.defaultAsset(type: .perpetual).id,
        )
    }
}
