// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import protocol Gemstone.GemChainServiceProtocol
import GemstonePrimitives
import Localization
import Primitives
import PrimitivesComponents
import Style

@Observable
@MainActor
public final class ChainListSettingsViewModel {
    private let service: any GemChainServiceProtocol

    public init(service: any GemChainServiceProtocol) {
        self.service = service
    }

    var emptyContent: EmptyContentTypeViewModel {
        EmptyContentTypeViewModel(type: .search(type: .networks))
    }

    func filterChains(for query: String) -> [Chain] {
        service.getChains(query: query).map { Chain(core: $0) }
    }

    var serviceStatusListItem: ListItemModel {
        ListItemModel(
            title: Localized.Transaction.status,
            imageStyle: .asset(assetImage: AssetImage.image(Images.Logo.logo)),
        )
    }
}
