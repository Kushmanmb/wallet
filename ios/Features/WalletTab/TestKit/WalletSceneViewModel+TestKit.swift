// Copyright (c). Gem Wallet. All rights reserved.

import class Gemstone.GemNftService
import protocol Gemstone.GemWalletHomeServiceProtocol
import GemstoneServicesTestKit
import Foundation
import Primitives
import PrimitivesTestKit
import GemstonePrimitivesTestKit
import NFT
import WalletTab

public extension WalletSceneViewModel {
    static func mock(
        wallet: Wallet = .mock(),
        service: any GemWalletHomeServiceProtocol = GemWalletHomeServiceMock(),
    ) -> WalletSceneViewModel {
        WalletSceneViewModel(
            service: service,
            observablePreferences: .mock(),
            collectionsModel: CollectionsViewModel(service: GemNftService.mock(), wallet: wallet),
            wallet: wallet,
            isPresentingSelectedAssetInput: .constant(.none),
            isPresentingWallets: .constant(false),
        )
    }
}
