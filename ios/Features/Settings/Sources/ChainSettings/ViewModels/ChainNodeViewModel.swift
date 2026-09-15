// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Components
import Formatters
import GemstonePrimitives
import struct Gemstone.GemNodeRow
import struct Gemstone.GemNodeSelection
import Localization
import Style

struct ChainNodeViewModel {
    let row: GemNodeRow

    private let formatter: ValueFormatter

    init(row: GemNodeRow, formatter: ValueFormatter) {
        self.row = row
        self.formatter = formatter
    }

    var node: GemNodeSelection {
        row.node
    }

    var url: String {
        row.node.url
    }

    var canDelete: Bool {
        row.canDelete
    }

    var selection: String? {
        row.node.isSelected ? row.node.url : .none
    }

    var title: String {
        switch row.title {
        case let .host(host): host
        case let .gemNode(flag): Localized.Nodes.gemWalletNode + " " + flag
        }
    }

    var titleExtra: String? {
        switch row.subtitle {
        case let .latestBlock(value):
            let text = value.map { formatter.string(BigInt($0), decimals: 0) } ?? "-"
            return "\(row.subtitle.title): \(text)"
        }
    }

    var titleTag: String? {
        statusTag.text
    }

    var titleTagType: TitleTagType {
        statusTag.type
    }

    var titleTagStyle: TextStyle {
        statusTag.style
    }

    private var statusTag: LatencyStatusViewModel {
        LatencyStatusViewModel(status: row.latencyStatus)
    }
}

// MARK: - Identifiable

extension ChainNodeViewModel: Identifiable {
    var id: String {
        row.node.url
    }
}
