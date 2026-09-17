// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import enum Gemstone.GemListRow
import enum Gemstone.GemListRowIcon
import enum Gemstone.GemListRowTitle
import enum Gemstone.GemUrlTarget
import struct Gemstone.GemSocialLink
import GemstonePrimitives
import Localization
import Primitives

struct AddressCardModel {
    let address: String
    let copyModel: CopyTypeViewModel
}

enum GemListRowItem {
    case listItem(ListItemModel)
    case page(ListItemModel, url: URL)
    case external(ListItemModel, url: URL)
    case icon(AssetImage)
    case address(AddressCardModel)
    case social([GemSocialLink])
    case loading
}

extension GemListRow {
    var item: GemListRowItem {
        switch self {
        case let .text(title, value):
            .listItem(ListItemModel(title: title.text, subtitle: value))
        case let .amount(title, amount):
            .listItem(ListItemModel(title: title.text, subtitle: amount.text()))
        case let .link(title, value, icon):
            .listItem(listItem(title: title, value: value, icon: icon))
        case let .url(title, value, icon, url, target):
            urlItem(title: title, value: value, icon: icon, url: url, target: target)
        case let .social(links):
            .social(links)
        case let .error(error):
            .listItem(ListItemModel(title: GemListRowTitle.error.text, subtitle: error.localizedDescription))
        case let .explorer(name, url):
            .page(ListItemModel(title: Localized.Transaction.viewOn(name)), url: URL(string: url) ?? BlockExplorerLink(name: name, link: url).url)
        case let .icon(chain):
            .icon(AssetIdViewModel(assetId: Chain(core: chain).assetId).assetImage)
        case let .address(address, copy):
            .address(AddressCardModel(address: address, copyModel: copy.copyModel))
        case .loading:
            .loading
        }
    }

    private func listItem(title: GemListRowTitle, value: String?, icon: GemListRowIcon) -> ListItemModel {
        ListItemModel(title: title.text, subtitle: value, imageStyle: .settings(assetImage: icon.assetImage))
    }

    private func urlItem(title: GemListRowTitle, value: String?, icon: GemListRowIcon, url: String, target: GemUrlTarget) -> GemListRowItem {
        let model = listItem(title: title, value: value, icon: icon)
        guard let url = URL(string: url) else { return .listItem(model) }
        switch target {
        case .inApp: return .page(model, url: url)
        case .external: return .external(model, url: url)
        }
    }
}
