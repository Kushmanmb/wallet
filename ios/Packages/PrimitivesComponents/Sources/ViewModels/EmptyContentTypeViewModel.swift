// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import func Gemstone.emptyState
import struct Gemstone.GemEmptyState
import enum Gemstone.GemEmptyStateAction
import struct Gemstone.GemEmptyStateInput
import enum Gemstone.GemEmptyStateKind
import Localization
import Primitives
import Style
import SwiftUI

public struct EmptyContentTypeViewModel: EmptyContentViewable {
    private let type: EmptyContentType
    private let state: GemEmptyState

    public init(type: EmptyContentType) {
        self.type = type
        state = emptyState(
            input: GemEmptyStateInput(
                kind: type.kind,
                isViewOnly: type.isViewOnly,
                offeredActions: Array(type.actions.keys),
            ),
        )
    }

    public var title: String {
        state.title.text(symbol: type.symbol)
    }

    public var description: String? {
        state.description?.text(symbol: type.symbol)
    }

    public var image: Image? {
        state.image.image
    }

    public var buttons: [EmptyAction] {
        let actions = type.actions
        return state.actions.compactMap { action in
            actions[action].map { EmptyAction(title: action.title, action: $0) }
        }
    }
}
