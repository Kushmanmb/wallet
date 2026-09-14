// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemPriceAlertRow
import enum Gemstone.PriceAlertDirection
import Style
import SwiftUI

extension PriceAlertDirection {
    public var color: Color {
        switch self {
        case .up: Colors.green
        case .down: Colors.red
        }
    }
}

extension GemPriceAlertRow {
    public var directionColor: Color {
        direction?.color ?? Colors.gray
    }
}
