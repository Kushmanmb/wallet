// Copyright (c). Gem Wallet. All rights reserved.

import protocol Gemstone.GemAssetSelectionServiceProtocol
import class Gemstone.GemRecentActivityService
import GemstonePrimitivesTestKit
import GemstoneServices
import GemstoneServicesTestKit
import Primitives
import PrimitivesTestKit
import Recents
import StoreTestKit
import WalletTab

public extension WalletSearchSceneViewModel {
    @MainActor
    static func mock(
        wallet: Wallet = .mock(),
        service: any GemAssetSelectionServiceProtocol = GemAssetSelectionServiceMock(),
        preferences: ObservablePreferences = .mock(preferencesService: GemPreferencesServiceMock(perpetualEnabled: true)),
    ) -> WalletSearchSceneViewModel {
        WalletSearchSceneViewModel(
            wallet: wallet,
            service: service,
            preferences: preferences,
            recentModel: RecentAssetsModel(
                walletId: wallet.id,
                types: RecentActivityType.allCases,
                service: GemRecentActivityService(store: GemstoneRecentActivityStore(store: .mock()), session: .mock()),
            ),
            onDismissSearch: {},
            onSelectAssetAction: { _ in },
            onAddToken: {},
        )
    }
}
