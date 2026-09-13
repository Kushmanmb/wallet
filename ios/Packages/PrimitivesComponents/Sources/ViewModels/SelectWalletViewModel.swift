// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import Localization
import struct Gemstone.GemWalletRow
import Primitives
import Style

public struct SelectWalletViewModel: SelectableListAdoptable {
    public typealias Item = GemWalletRow

    public var title: String {
        Localized.Wallets.title
    }

    public var state: StateViewType<SelectableListType<GemWalletRow>>
    public var selectedItems: Set<GemWalletRow>
    public var selectionType: SelectionType = .checkmark

    public init(
        rows: [GemWalletRow],
        pinnedIds: Set<String>,
        selectedRow: GemWalletRow,
    ) {
        let sections: [ListSection<GemWalletRow>] = [
            (Localized.Common.pinned, Images.System.pin, rows.filter { pinnedIds.contains($0.id) }),
            (nil, nil, rows.filter { !pinnedIds.contains($0.id) }),
        ]
        .filter(\.2.isNotEmpty)
        .map { title, image, items in
            ListSection(id: items.map(\.id).joined(), title: title, image: image, values: items)
        }

        self.init(
            state: .data(.section(sections)),
            selectedItems: [selectedRow],
            selectionType: .checkmark,
        )
    }

    public init(
        state: StateViewType<SelectableListType<GemWalletRow>>,
        selectedItems: [GemWalletRow],
        selectionType _: SelectionType,
    ) {
        self.state = state
        self.selectedItems = Set(selectedItems)
    }
}

extension SelectWalletViewModel: SelectableListNavigationAdoptable {}
