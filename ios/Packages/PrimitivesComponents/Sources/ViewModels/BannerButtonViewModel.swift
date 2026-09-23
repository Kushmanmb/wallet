// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import enum Gemstone.GemBannerButton
import struct Gemstone.GemBannerKey
import Localization
import Primitives
import Style

struct BannerButtonViewModel: Identifiable {
    let button: GemBannerButton
    let key: GemBannerKey

    var id: String {
        String(describing: button)
    }

    var title: String {
        switch button {
        case .buy: Localized.Wallet.buy
        case .receive: Localized.Wallet.receive
        }
    }

    @MainActor
    var style: ColorButtonStyle {
        switch button {
        case .buy: .blue(paddingVertical: .small)
        case .receive: .empty(paddingVertical: .small)
        }
    }

    var action: BannerAction {
        BannerAction(key: key, type: .button(button))
    }
}
