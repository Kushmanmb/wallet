// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import struct Gemstone.GemSwapProviderRow
import GemstonePrimitives
import Primitives
import PrimitivesComponents
import Style

public struct SwapProviderItem: Sendable {
    public let row: GemSwapProviderRow

    public init(row: GemSwapProviderRow) {
        self.row = row
    }
}

// MARK: - SimpleListItemViewable

extension SwapProviderItem: SimpleListItemViewable {
    public var title: String {
        row.title
    }

    public var titleStyle: TextStyle {
        TextStyle(font: .callout, color: Colors.black, fontWeight: .semibold)
    }

    public var subtitle: String? {
        row.amount.text()
    }

    public var assetImage: AssetImage {
        AssetImage(
            placeholder: row.provider.toPrimitives().image,
            chainPlaceholder: row.isSelected ? Images.Wallets.selected : nil,
        )
    }

    public var subtitleExtra: String? {
        row.fiat?.text()
    }

    public var subtitleStyle: TextStyle {
        TextStyle(font: .callout, color: Colors.black, fontWeight: .semibold)
    }

    public var subtitleStyleExtra: TextStyle {
        TextStyle(font: .footnote, color: Colors.gray)
    }
}

// MARK: - Identifiable

extension SwapProviderItem: Identifiable {
    public var id: String {
        row.provider.toPrimitives().rawValue
    }
}

// MARK: - Hashable

extension SwapProviderItem: Hashable {
    public static func == (lhs: SwapProviderItem, rhs: SwapProviderItem) -> Bool {
        lhs.id == rhs.id
    }

    public func hash(into hasher: inout Hasher) {
        id.hash(into: &hasher)
    }
}
