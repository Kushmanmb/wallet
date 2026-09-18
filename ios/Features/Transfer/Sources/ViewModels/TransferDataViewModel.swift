// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemConfirmTitle
import enum Gemstone.TransactionInputType
import GemstonePrimitives
import Primitives
import struct Gemstone.GemTransferData

struct TransferDataViewModel {
    let data: GemTransferData

    var type: TransactionInputType {
        data.inputType
    }

    var asset: Asset {
        data.asset
    }

    var chain: Chain {
        data.chain
    }

    var chainType: ChainType {
        chain.type
    }

    var chainAsset: Asset {
        chain.asset
    }

    var title: String {
        data.title().title
    }
}
