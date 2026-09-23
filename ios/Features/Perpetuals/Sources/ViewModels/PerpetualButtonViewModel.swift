// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemPerpetualButton
import struct Gemstone.GemPerpetualButtonRow

public struct PerpetualButtonViewModel: Identifiable, Hashable {
    public enum Style {
        case green
        case red
        case blue
    }

    let row: GemPerpetualButtonRow

    public var id: String {
        String(describing: row.button)
    }

    var button: GemPerpetualButton {
        row.button
    }

    public var title: String {
        row.button.title
    }

    public var isDestructive: Bool {
        row.tone == .negative
    }

    public var style: Style {
        switch row.tone {
        case .positive: .green
        case .negative: .red
        case .plain, .neutral, .warning: .blue
        }
    }
}
