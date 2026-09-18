// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import Localization
import Primitives
import PrimitivesComponents

struct AddAssetViewModel {
    let link: BlockExplorerLink?

    var explorerListItem: ListItemModel? {
        explorerText.map { ListItemModel(title: $0) }
    }

    var explorerText: String? {
        link.map { Localized.Transaction.viewOn($0.name) }
    }

    var explorerUrl: URL? {
        link?.url
    }
}
