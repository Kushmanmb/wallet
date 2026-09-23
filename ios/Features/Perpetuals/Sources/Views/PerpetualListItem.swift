// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Primitives
import PrimitivesComponents
import SwiftUI

struct PerpetualListItem: View {
    let perpetualData: PerpetualData
    let model: PerpetualItemViewModel
    let onPin: (PerpetualData) -> Void
    let onSelect: (Asset) -> Void

    var body: some View {
        NavigationCustomLink(
            with: ListAssetItemView(model: model),
            action: { onSelect(perpetualData.asset) },
        )
        .listRowInsets(.assetListRowInsets)
        .contextMenu(
            [
                .pin(
                    isPinned: perpetualData.metadata.isPinned,
                    onPin: {
                        onPin(perpetualData)
                    },
                ),
            ],
        )
    }
}
