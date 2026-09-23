// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemPriceAlertItem
import GemstonePrimitives
import Primitives

struct PriceAlertItem: Identifiable {
    let data: PriceAlertData
    let model: PriceAlertItemViewModel

    init(item: GemPriceAlertItem) {
        data = item.data.toPrimitives()
        model = PriceAlertItemViewModel(row: item.row)
    }

    var id: String {
        data.id
    }
}
