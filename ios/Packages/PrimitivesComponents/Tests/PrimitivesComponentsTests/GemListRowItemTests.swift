// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import enum Gemstone.GemInfoTopic
import enum Gemstone.GemListRow
import func Gemstone.addressCopy
import func Gemstone.walletRow
import GemstonePrimitives
import GemstonePrimitivesTestKit
import Localization
@testable import Primitives
@testable import PrimitivesComponents
import PrimitivesTestKit
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
    func aNetworkRowShowsTheNameCoreGives() {
        guard case let .network(title, subtitle, _) = GemListRow.network(title: .network, chain: Chain.ethereum.rawValue, name: "Ethereum (ERC20)").item(onInfo: nil) else {
            Issue.record("Expected a network row")
            return
        }
        #expect(title == Localized.Transfer.network)
        #expect(subtitle == "Ethereum (ERC20)")
    }

    @Test
    func anAppRowOpensItsWebsite() {
        guard case let .app(model, website) = GemListRow.app(name: "PancakeSwap", iconUrl: nil, websiteUrl: "https://pancakeswap.finance").item(onInfo: nil) else {
            Issue.record("Expected an app row")
            return
        }
        #expect(model.title == Localized.WalletConnect.app)
        #expect(model.subtitle == "PancakeSwap")
        #expect(website == URL(string: "https://pancakeswap.finance"))
    }

    @Test
    func aWalletRowCarriesItsExplorerContext() {
        let wallet = Wallet.mock()
        let row = GemListRow.wallet(
            wallet: walletRow(wallet: wallet.toGem()),
            copy: addressCopy(chain: Chain.ethereum.rawValue, address: "0x1"),
            explorer: BlockExplorerLink.mock().toGem(),
        )
        guard case let .wallet(model, context) = row.item(onInfo: nil) else {
            Issue.record("Expected a wallet row")
            return
        }
        #expect(model.title == Localized.Common.wallet)
        #expect(model.subtitle == wallet.name)
        #expect(model.imageStyle != nil)
        #expect(context == ExplorerContextData(copyValue: .address(value: "0x1", chain: .ethereum), explorerLink: .mock()))
    }

    @Test
    func aMemoRowCopiesOnlyARealMemo() {
        guard case let .memo(model, copy) = GemListRow.memo(value: "12345", copy: "12345").item(onInfo: nil),
              case let .memo(placeholder, noCopy) = GemListRow.memo(value: "-", copy: nil).item(onInfo: nil)
        else {
            Issue.record("Expected memo rows")
            return
        }
        #expect(model.title == Localized.Transfer.memo)
        #expect(model.subtitle == "12345")
        #expect(copy == "12345")
        #expect(placeholder.subtitle == "-")
        #expect(noCopy == nil)
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

    @Test
    func aPositionRowJoinsItsPnlAndMargin() {
        let pnl = GemListRow.label(
            title: .pnl,
            text: .pnl(amount: .mock(value: 500), percent: .mock(value: 50, unit: .percent)),
            tone: .positive,
            info: nil,
            progress: false,
        )
        let margin = GemListRow.label(title: .margin, text: .margin(amount: .mock(value: 1000, notation: .plain), marginType: .isolated), tone: .plain, info: nil, progress: false)
        guard case let .listItem(pnlModel) = pnl.item(onInfo: nil), case let .listItem(marginModel) = margin.item(onInfo: nil) else {
            Issue.record("Expected list items")
            return
        }
        #expect(pnlModel.subtitle == "+$500.00 (+50.00%)")
        #expect(marginModel.subtitle == "$1,000.00 (Isolated)")
    }

    @Test
    func autocloseLinesOfferTheirExplanation() {
        var opened: GemInfoTopic?
        let row = GemListRow.lines(title: .autoClose, lines: [.text(text: "TP: $120.00")], info: .autoClose)
        guard case let .listItem(model) = row.item(onInfo: { opened = $0 }) else {
            Issue.record("Expected a list item")
            return
        }
        model.infoAction?()
        #expect(model.subtitle == "TP: $120.00")
        #expect(opened == .autoClose)
    }
}
