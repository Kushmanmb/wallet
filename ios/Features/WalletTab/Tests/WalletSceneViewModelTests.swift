// Copyright (c). Gem Wallet. All rights reserved.

import GemstonePrimitivesTestKit
import GemstoneServicesTestKit
import Observation
import Primitives
import PrimitivesTestKit
@testable import Store
import StoreTestKit
import Testing
@testable import WalletTab
import WalletTabTestKit

@MainActor
struct WalletSceneViewModelTests {
    @Test
    func renameNotifiesWalletBar() async throws {
        let wallet = Wallet.mock(id: .multicoin(address: "0x1"), name: "First")
        let db = DB.mock()
        let store = WalletStore.mock(db: db)
        try store.addWallet(wallet)

        let model = WalletSceneViewModel.mock(wallet: wallet)
        model.walletQuery.bind(dbQueue: db.dbQueue)

        #expect(model.walletBarModel.name == "First")

        await confirmation(expectedCount: 1...) { changed in
            withObservationTracking {
                _ = model.wallet
            } onChange: {
                changed()
            }

            try? store.renameWallet(wallet.id, name: "Renamed")
            for _ in 0 ..< 100 where model.wallet.name != "Renamed" {
                try? await Task.sleep(for: .milliseconds(10))
            }
        }

        #expect(model.walletBarModel.name == "Renamed")
    }

    @Test
    func onboardingBannerShowsOnlyWhileEveryBalanceIsZero() throws {
        let funded = try onboardingModel(db: DB.mockAssets())
        let empty = try onboardingModel(db: DB.mockAssets(assets: [.mock()]))

        #expect(funded.homeState.visibleBanners.map(\.event) == [])
        #expect(empty.homeState.visibleBanners.map(\.event) == [.onboarding])
    }

    @Test
    func homeStateIsDerivedOncePerInput() throws {
        let db = DB.mockAssets()
        let service = GemWalletHomeServiceMock()
        let wallet = Wallet.mock()
        let model = WalletSceneViewModel.mock(wallet: wallet, service: service)
        model.assetsQuery.bind(dbQueue: db.dbQueue)
        model.bannersQuery.bind(dbQueue: db.dbQueue)

        _ = model.homeState
        _ = model.homeState
        #expect(service.viewStateCalls == 1, "reading the state twice asks Core once")

        model.fiatValuesQuery.value = [AssetFiatValue(amount: 1, price: 2, priceChangePercentage24h: 0)]
        _ = model.homeState
        #expect(service.viewStateCalls == 2, "changed balances derive a new state")

        model.observablePreferences.currency = .eur
        _ = model.homeState
        #expect(service.viewStateCalls == 3, "a changed preference derives a new state")
    }

    private func onboardingModel(db: DB) throws -> WalletSceneViewModel {
        let wallet = Wallet.mock()
        try BannerStore(db: db).addBanners([NewBanner(id: "onboarding", walletId: wallet.id.id, event: .onboarding, state: .active)])
        let model = WalletSceneViewModel.mock(wallet: wallet)
        model.assetsQuery.bind(dbQueue: db.dbQueue)
        model.bannersQuery.bind(dbQueue: db.dbQueue)
        return model
    }
}
