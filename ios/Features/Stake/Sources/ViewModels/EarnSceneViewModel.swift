// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import enum Gemstone.GemListRow
import enum Gemstone.GemLoadState
import protocol Gemstone.GemStakeServiceProtocol
import GemstonePrimitives
import GemstoneServices
import Localization
import Primitives
import PrimitivesComponents
import Store

@MainActor
@Observable
public final class EarnSceneViewModel {
    private let service: any GemStakeServiceProtocol
    private let onNavigate: StakeRouteAction
    private var viewState: GemLoadState = .loading

    public let wallet: Wallet
    public let asset: Asset

    public let assetQuery: ObservableQuery<AssetRequest>
    public let positionsQuery: ObservableQuery<DelegationsRequest>
    public let providersQuery: ObservableQuery<ValidatorsRequest>

    public var assetData: AssetData {
        assetQuery.value
    }

    public var positions: [Delegation] {
        positionsQuery.value
    }

    public var providers: [DelegationValidator] {
        selectable(providersQuery.value)
    }

    public init(
        wallet: Wallet,
        asset: Asset,
        service: any GemStakeServiceProtocol,
        onNavigate: StakeRouteAction,
    ) {
        self.wallet = wallet
        self.asset = asset
        self.service = service
        self.onNavigate = onNavigate
        assetQuery = ObservableQuery(AssetRequest(walletId: wallet.id, assetId: asset.id), initialValue: .with(asset: asset))
        positionsQuery = ObservableQuery(
            DelegationsRequest(walletId: wallet.id, assetId: asset.id, providerType: .earn),
            initialValue: [],
        )
        providersQuery = ObservableQuery(
            ValidatorsRequest(chain: asset.id.chain, providerType: .earn),
            initialValue: [],
        )
    }

    var title: String {
        Localized.Common.earn
    }

    private func selectable(_ validators: [DelegationValidator]) -> [DelegationValidator] {
        service.selectableValidators(validators: validators.map { $0.toGem() }).map { $0.toPrimitives() }
    }

    var assetModel: AssetViewModel {
        AssetViewModel(asset: asset)
    }

    var aprRow: GemListRow {
        service.earnAprRow(providers: providers.map { $0.toGem() }, assetApr: assetData.metadata.earnApr)
    }

    var noDataListItem: ListItemModel {
        ListItemModel(title: Localized.Errors.noDataAvailable)
    }

    var depositListItem: ListItemModel {
        ListItemModel(title: Localized.Wallet.deposit)
    }

    var canDeposit: Bool {
        depositRoute != nil
    }

    private var depositRoute: StakeRoute? {
        guard let provider = service.earnActions(walletType: wallet.type.toGem(), providers: providersQuery.value.map { $0.toGem() }).depositProvider else { return nil }
        return .transfer(.amount(AmountInput(type: .earn(.deposit(provider)), asset: asset)))
    }

    var emptyContentModel: EmptyContentTypeViewModel {
        EmptyContentTypeViewModel(type: EmptyContentType(.earn, symbol: asset.symbol))
    }

    var positionItems: [(delegation: Delegation, model: DelegationViewModel)] {
        DelegationViewModel.items(earnPositions, asset: asset, price: assetData.price?.price, currency: service.getCurrency().toPrimitives())
    }

    var hasPositions: Bool {
        earnPositions.isNotEmpty
    }

    private var earnPositions: [Delegation] {
        service.positions(delegations: positions.map { $0.toGem() }).map { Delegation(core: $0) }
    }

    func route(delegation: Delegation) -> StakeRoute {
        service.delegationDestination(walletType: wallet.type.toGem(), asset: asset.toGem(), delegation: delegation.toGem())
            .route(delegation: delegation, validators: [])
    }

    var showEmptyState: Bool {
        !hasPositions && viewState != .loading
    }

    var positionsSectionTitle: String {
        hasPositions ? Localized.Perpetual.positions : .empty
    }

    var providersState: StateViewType<Bool> {
        viewState.stateViewType(providers).map { _ in true }
    }
}

// MARK: - Actions

extension EarnSceneViewModel {
    func onSelect(delegation: Delegation) {
        onNavigate?(route(delegation: delegation))
    }

    func onSelectDeposit() {
        depositRoute.map { onNavigate?($0) }
    }

    func load() async {
        viewState = .loading
        viewState = await service.refreshEarn(assetId: asset.id.identifier, hasRows: hasPositions)
    }
}
