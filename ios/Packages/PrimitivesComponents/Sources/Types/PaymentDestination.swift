// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemTransferData
import GemstonePrimitives
import Primitives

public enum PaymentDestination: Identifiable, Sendable {
    case confirm(GemTransferData)
    case recipient(SelectedAssetInput)
    case selectAsset(SelectAssetType, chains: [Chain])

    public var id: String {
        switch self {
        case let .confirm(data): "confirm-\(data.id)"
        case let .recipient(input): "recipient-\(input.id)"
        case let .selectAsset(type, _): "selectAsset-\(type.id)"
        }
    }
}
