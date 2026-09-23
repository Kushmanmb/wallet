// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import struct Gemstone.GemPerpetualButtonRow
import struct Gemstone.GemPerpetualDetails
import enum Gemstone.GemPerpetualSection
import struct Gemstone.PerpetualPosition

public extension GemPerpetualDetails {
    static func mock(
        title: String = "BTC",
        sections: [GemPerpetualSection] = [],
        modifyButtons: [GemPerpetualButtonRow] = [],
        position: PerpetualPosition? = nil,
    ) -> GemPerpetualDetails {
        GemPerpetualDetails(
            title: title,
            sections: sections,
            modifyButtons: modifyButtons,
            position: position,
        )
    }
}
