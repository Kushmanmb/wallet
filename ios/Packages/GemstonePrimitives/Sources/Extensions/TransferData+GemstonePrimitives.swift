// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemTransferData
import enum Gemstone.TransactionInputType
import Primitives

public extension GemTransferData {
    var asset: Primitives.Asset {
        inputAsset().toPrimitives()
    }

    var chain: Chain {
        asset.chain
    }

    var applicationMetadata: Primitives.ApplicationMetadata? {
        guard case let .generic(_, metadata, _) = inputType else { return nil }
        return metadata.toPrimitives()
    }

    var id: String {
        identifier()
    }
}
