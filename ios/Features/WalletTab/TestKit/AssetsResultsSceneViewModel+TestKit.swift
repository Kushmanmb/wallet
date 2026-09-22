// Copyright (c). Gem Wallet. All rights reserved.

import protocol Gemstone.GemAssetSelectionServiceProtocol
import GemstonePrimitivesTestKit
import GemstoneServices
import GemstoneServicesTestKit
import Primitives
import PrimitivesTestKit
import Store
import WalletTab

public extension AssetsResultsSceneViewModel {
    @MainActor
    static func mock(
        service: any GemAssetSelectionServiceProtocol = GemAssetSelectionServiceMock(),
        preferences: ObservablePreferences = .mock(preferencesService: GemPreferencesServiceMock(perpetualEnabled: true)),
        request: WalletSearchRequest = WalletSearchRequest(walletId: .mock(), searchBy: "usdc", types: [.asset]),
        onSelectAsset: @escaping (Asset) -> Void = { _ in },
    ) -> AssetsResultsSceneViewModel {
        AssetsResultsSceneViewModel(
            wallet: .mock(),
            service: service,
            preferences: preferences,
            request: request,
            title: "Results",
            onSelectAsset: onSelectAsset,
        )
    }
}
