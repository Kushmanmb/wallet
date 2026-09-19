// Copyright (c). Gem Wallet. All rights reserved.

import Components
import GemstonePrimitives
import InfoSheet
import Localization
import Primitives
import PrimitivesComponents
import Store
import SwiftUI

public struct ChartScene: View {
    @State private var model: ChartSceneViewModel

    public init(model: ChartSceneViewModel) {
        _model = State(initialValue: model)
    }

    public var body: some View {
        ChartListView(model: model) {
            ForEach(model.sections, id: \.self) { section in
                switch section {
                case .priceAlerts:
                    Section {
                        NavigationLink(
                            value: Scenes.AssetPriceAlert(asset: model.asset),
                            label: {
                                ListItemView(model: model.listItem(for: section))
                            },
                        )
                    }
                case .setPriceAlert:
                    Section {
                        NavigationCustomLink(with: ListItemView(model: model.listItem(for: section))) {
                            model.onSelectSetPriceAlerts()
                        }
                    }
                case let .market(rows):
                    Section {
                        ForEach(rows, id: \.self) { row in
                            GemListRowView(row: row, onInfo: model.onInfo)
                        }
                    }
                case let .links(links):
                    Section(section.title ?? "") {
                        SocialLinksView(model: SocialLinksViewModel(links: links))
                    }
                }
            }
        }
        .bindQuery(model.priceQuery)
        .navigationTitle(model.title)
        .sheet(item: $model.isPresentingInfoSheet) {
            InfoSheetScene(type: $0)
        }
    }
}
