// Copyright (c). Gem Wallet. All rights reserved.

import Components
import enum Gemstone.GemInfoTopic
import enum Gemstone.GemListRow
import Localization
import Primitives
@testable import PrimitivesComponents
import Style
import Testing

struct GemListRowItemTests {
    @Test
    func aPendingStatusSpinsInItsTone() {
        let row = GemListRow.label(title: .status, text: .transactionState(state: .pending), tone: .warning, info: nil, progress: true)
        guard case let .listItem(model) = row.item(onInfo: nil), case .progressView = model.subtitleTagType else {
            Issue.record("Expected a spinning status row")
            return
        }
        #expect(model.title == Localized.Transaction.status)
        #expect(model.subtitle == Localized.Transaction.Status.pending)
        #expect(model.subtitleStyle.color == Colors.orange)
    }

    @Test
    func aNetworkRowShowsTheChainName() {
        guard case let .network(title, subtitle, _) = GemListRow.network(title: .network, chain: Chain.bitcoin.rawValue).item(onInfo: nil) else {
            Issue.record("Expected a network row")
            return
        }
        #expect(title == Localized.Transfer.network)
        #expect(subtitle == "Bitcoin")
    }

    @Test
    func anInfoTopicBecomesTheRowsInfoAction() {
        var opened: GemInfoTopic?
        let row = GemListRow.label(title: .status, text: .transactionState(state: .confirmed), tone: .positive, info: .stakeApr, progress: false)
        guard case let .listItem(model) = row.item(onInfo: { opened = $0 }) else {
            Issue.record("Expected a list item")
            return
        }
        model.infoAction?()
        #expect(opened == .stakeApr)
    }
}
