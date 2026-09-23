// Copyright (c). Gem Wallet. All rights reserved.

import Primitives
import SwiftUI

public struct PerpetualSectionView: View {
    private let perpetuals: [PerpetualData]
    private let onPin: (PerpetualData) -> Void
    private let onSelect: (Asset) -> Void

    public init(
        perpetuals: [PerpetualData],
        onPin: @escaping (PerpetualData) -> Void,
        onSelect: @escaping (Asset) -> Void,
    ) {
        self.perpetuals = perpetuals
        self.onPin = onPin
        self.onSelect = onSelect
    }

    public var body: some View {
        ForEach(PerpetualItemViewModel.items(perpetuals), id: \.data.id) { data, model in
            PerpetualListItem(
                perpetualData: data,
                model: model,
                onPin: onPin,
                onSelect: onSelect,
            )
        }
    }
}
