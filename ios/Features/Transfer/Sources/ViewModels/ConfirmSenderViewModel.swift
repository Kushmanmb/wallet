// Copyright (c). Gem Wallet. All rights reserved.

import Components
import struct Gemstone.GemWalletRow
import Localization
import Primitives
import PrimitivesComponents

struct ConfirmSenderViewModel {
    private let row: GemWalletRow

    init(row: GemWalletRow) {
        self.row = row
    }
}

// MARK: - ItemModelProvidable

extension ConfirmSenderViewModel: ItemModelProvidable {
    var itemModel: ConfirmTransferItemModel {
        .sender(
            ListItemModel(
                title: Localized.Common.wallet,
                subtitle: row.name,
                imageStyle: ListItemImageStyle.list(assetImage: row.avatarImage),
            ),
        )
    }
}
