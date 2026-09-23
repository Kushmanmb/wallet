// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Primitives
import PrimitivesComponents
import SwiftUI

public struct PerpetualPositionsList: View {
    private let positions: [PerpetualPositionData]
    private let onSelect: AssetAction
    @Binding private var showBalancePrivacy: Bool

    public init(
        positions: [PerpetualPositionData],
        showBalancePrivacy: Binding<Bool> = .constant(false),
        onSelect: AssetAction = nil,
    ) {
        self.positions = positions
        _showBalancePrivacy = showBalancePrivacy
        self.onSelect = onSelect
    }

    public var body: some View {
        ForEach(PerpetualPositionItemViewModel.items(positions, showBalancePrivacy: $showBalancePrivacy), id: \.model.id) { position, model in
            if let onSelect {
                NavigationCustomLink(
                    with: ListAssetItemView(model: model),
                    action: { onSelect(position.perpetualData.asset) },
                )
            } else {
                NavigationLink(value: Scenes.Perpetual(position.perpetualData)) {
                    ListAssetItemView(model: model)
                }
            }
        }
    }
}
