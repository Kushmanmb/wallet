// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemPriceAlertRow
import enum Gemstone.GemValueTone
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

extension GemValueTone {
    public var color: Color {
        switch self {
        case .plain: Colors.black
        case .neutral: Colors.gray
        case .positive: Colors.green
        case .negative: Colors.red
        }
    }
}

extension GemPriceAlertRow {
    public var directionColor: Color {
        direction?.color ?? Colors.gray
    }
}
