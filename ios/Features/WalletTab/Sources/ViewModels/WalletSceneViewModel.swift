// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemHeaderButtonKind
import Components
import Formatters
import Foundation
import struct Gemstone.GemBannerContent
import protocol Gemstone.GemWalletHomeServiceProtocol
import func Gemstone.walletRow
import GemstonePrimitives
import GemstoneServices
import InfoSheet
import Localization
import NFT
import struct Gemstone.GemAssetRow
import Primitives
import PrimitivesComponents
import Store
import Style
import SwiftUI

@Observable
@MainActor
public final class WalletSceneViewModel: Sendable, AssetActions {
    private let service: any GemWalletHomeServiceProtocol

    let observablePreferences: ObservablePreferences

    public let collectionsModel: CollectionsViewModel

    public var wallet: Wallet {
        walletQuery.value
    }

    // db queries
    public let walletQuery: ObservableQuery<WalletRequest>
    public let fiatValuesQuery: ObservableQuery<AssetFiatValuesRequest>
    public let perpetualBalanceQuery: ObservableQuery<PerpetualWalletBalanceRequest>
    public let assetsQuery: ObservableQuery<AssetsRequest>
    public let bannersQuery: ObservableQuery<BannersRequest>

    public var isPresentingSelectedAssetInput: Binding<SelectedAssetInput?>
    public var isPresentingScanner = false
    public var isPresentingWallets: Binding<Bool>
    public var isPresentingSheet: WalletSheetType?
    public var isPresentingSearch = false
    public var isPresentingUrl: URL?
    public var isPresentingToastMessage: ToastMessage?

    public var isLoadingAssets = false

    @ObservationIgnored private let derivedHomeState = DerivedValue<HomeStateInput, WalletHomeState>()

    public init(
        service: any GemWalletHomeServiceProtocol,
        observablePreferences: ObservablePreferences,
        collectionsModel: CollectionsViewModel,
        wallet: Wallet,
        isPresentingSelectedAssetInput: Binding<SelectedAssetInput?>,
        isPresentingWallets: Binding<Bool>,
    ) {
        self.service = service
        self.observablePreferences = observablePreferences
        self.collectionsModel = collectionsModel

        walletQuery = ObservableQuery(WalletRequest(walletId: wallet.id), initialValue: wallet)
        fiatValuesQuery = ObservableQuery(
            AssetFiatValuesRequest(walletId: wallet.id),
            initialValue: [],
        )
        perpetualBalanceQuery = ObservableQuery(
            PerpetualWalletBalanceRequest(walletId: wallet.id, assetId: Chain.hyperCore.defaultAsset(type: .perpetual).id),
            initialValue: nil,
        )
        assetsQuery = ObservableQuery(AssetsRequest(walletId: wallet.id, filters: [.enabledBalance]), initialValue: [])
        bannersQuery = ObservableQuery(BannersRequest(walletId: wallet.id, assetId: .none, events: [.accountBlockedMultiSignature, .onboarding]), initialValue: [])
        self.isPresentingSelectedAssetInput = isPresentingSelectedAssetInput
        self.isPresentingWallets = isPresentingWallets
    }

    public var assets: [AssetData] {
        assetsQuery.value
    }

    var manageTokenTitle: String {
        Localized.Wallet.manageTokenList
    }

    var perpetualsTitle: String {
        Localized.Perpetuals.title
    }

    var collectionsTitle: String {
        Localized.Nft.collections
    }

    var collectionsContent: CollectionsContent {
        collectionsModel.content
    }

    public var searchImage: Image {
        Images.System.search
    }

    public var scannerImage: Image {
        Images.System.qrCodeViewfinder
    }

    public var manageImage: Image {
        Images.Actions.manage
    }


    public var walletBarModel: WalletBarViewViewModel {
        let row = walletRow(wallet: wallet.toGem())
        return WalletBarViewViewModel(
            name: row.name,
            image: row.avatarImage,
        )
    }

    var homeState: WalletHomeState {
        derivedHomeState(
            HomeStateInput(
                wallet: wallet,
                assets: assets,
                balances: fiatValuesQuery.value,
                perpetual: perpetualBalanceQuery.value,
                banners: bannersQuery.value,
                currency: observablePreferences.currency,
                showPerpetuals: observablePreferences.showPerpetuals(for: wallet),
            ),
        ) { input in
            let viewState = service.viewState(
                wallet: input.wallet,
                balances: input.balances,
                perpetual: input.perpetual,
                banners: input.banners,
                isWalletEmpty: input.assets.allSatisfy(\.balance.total.isZero),
            )
            return WalletHomeState(
                sections: AssetsSections.from(input.assets),
                header: WalletHeaderViewModel(
                    totalValue: viewState.totalValue.toPrimitives(),
                    currency: input.currency,
                    showsPnl: viewState.showsPnl,
                    actions: viewState.headerActions,
                ),
                currency: input.currency,
                showPerpetuals: input.showPerpetuals,
                showCollections: viewState.showCollections,
                visibleBanners: viewState.visibleBanners.map { $0.toPrimitives() },
            )
        }
    }

    func bannerContent(for banner: Banner) -> GemBannerContent {
        service.content(for: banner)
    }

    struct HomeStateInput: Equatable {
        let wallet: Wallet
        let assets: [AssetData]
        let balances: [AssetFiatValue]
        let perpetual: PerpetualBalance?
        let banners: [Banner]
        let currency: Currency
        let showPerpetuals: Bool
    }
}

// MARK: - Business Logic

public extension WalletSceneViewModel {
    internal func load() async {
        await updateWallet()
    }

    internal func loadOnce() async {
        await loadOnce(wallet: wallet)
    }

    func onSelectWalletBar() {
        isPresentingWallets.wrappedValue = true
    }

    func onSelectManage(chains: [Chain] = []) {
        isPresentingSheet = .selectAsset(.manage, chains: chains)
    }

    func onToggleSearch() {
        isPresentingSearch.toggle()
    }

    func onSelectScanner() {
        isPresentingScanner = true
    }

    func onSelectAddCustomToken() {
        isPresentingSheet = .addAsset
    }

    internal func onSelectPortfolio() {
        isPresentingSheet = .portfolio(.wallet)
    }

    internal func onHeaderAction(type: GemHeaderButtonKind) {
        switch type {
        case .buy: isPresentingSheet = .selectAsset(.buy, chains: [])
        case .send: isPresentingSheet = .selectAsset(.send(.none), chains: [])
        case .receive: isPresentingSheet = .selectAsset(.receive(.asset), chains: [])
        case .swap: isPresentingSheet = .swap
        case .more, .deposit, .withdraw: break
        }
    }

    internal func onSelectWatchWalletInfo() {
        isPresentingSheet = .infoSheet(.watchWallet)
    }

    internal func onBanner(action: BannerAction) {
        switch action.type {
        case .event: break
        case .closeBanner:
            Task {
                try await service.close(action.banner)
            }
        case let .button(bannerButton):
            switch bannerButton {
            case .buy: isPresentingSheet = .selectAsset(.buy, chains: [])
            case .receive: isPresentingSheet = .selectAsset(.receive(.asset), chains: [])
            }
        }
        isPresentingUrl = action.url
    }

    internal func onCopyAddress(_ message: String) {
        isPresentingToastMessage = .copy(message)
    }

    func onWalletTabReselected(_: Bool, _: Bool) {
        isPresentingSearch = false
    }

    func onTransferComplete() {
        isPresentingSheet = nil
    }
}

// MARK: - Private

extension WalletSceneViewModel {
    private func loadOnce(wallet: Wallet) async {
        let shouldShowLoadingAssets = shouldShowInitialLoadingAssets

        if shouldShowLoadingAssets {
            isLoadingAssets = true
        }

        await updateWallet()

        if shouldShowLoadingAssets, self.wallet.id == wallet.id {
            isLoadingAssets = false
        }
    }

    private func updateWallet() async {
        do {
            try await service.refresh()
        } catch {
            debugLog("WalletSceneViewModel refresh error: \(error)")
        }
    }

    private var shouldShowInitialLoadingAssets: Bool {
        (try? service.showsInitialLoading()) ?? false
    }

    func setAssetPinned(_ assetId: AssetId, pinned: Bool) async throws {
        try await service.setAssetPinned(assetId: assetId, pinned: pinned)
    }

    func setAssetsEnabled(_ assetIds: [AssetId], enabled: Bool) async throws {
        try await service.setAssetsEnabled(assetIds: assetIds, enabled: enabled)
    }
    var assetRow: GemAssetRow {
        service.assetRow()
    }

}
