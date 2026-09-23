// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
public import enum Gemstone.GemEmptyStateAction
public import enum Gemstone.GemEmptyStateKind

public struct EmptyContentType {
    let kind: GemEmptyStateKind
    let symbol: String
    let isViewOnly: Bool
    let actions: [GemEmptyStateAction: () -> Void]

    public init(
        _ kind: GemEmptyStateKind,
        symbol: String = "",
        isViewOnly: Bool = false,
        actions: [GemEmptyStateAction: (() -> Void)?] = [:],
    ) {
        self.kind = kind
        self.symbol = symbol
        self.isViewOnly = isViewOnly
        self.actions = actions.compactMapValues { $0 }
    }
}
