// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemHeaderButtonKind
import enum Gemstone.LinkType
import struct Gemstone.GemPriceAlertRow
import enum Gemstone.GemValueTone
import enum Gemstone.PriceAlertDirection
import Style
import SwiftUI

extension PriceAlertDirection {
    public var color: Color {
        switch self {
        case .up: Colors.green
        case .down: Colors.red
        }
    }
}

extension GemValueTone {
    public var color: Color {
        switch self {
        case .plain: Colors.black
        case .neutral: Colors.gray
        case .positive: Colors.green
        case .negative: Colors.red
        }
    }
}

extension GemPriceAlertRow {
    public var directionColor: Color {
        direction?.color ?? Colors.gray
    }
}

extension GemHeaderButtonKind {
    public var image: Image {
        switch self {
        case .send: Images.System.paperplane
        case .receive: Images.System.qrCode
        case .buy: Images.System.dollar
        case .swap: Images.System.arrowSwap
        case .deposit: Images.Actions.buy
        case .withdraw: Images.Actions.send
        case .more: Images.Actions.more
        }
    }
}

extension LinkType {
    public var image: Image {
        switch self {
        case .x: Images.Social.x
        case .discord: Images.Social.discord
        case .reddit: Images.Social.reddit
        case .telegram: Images.Social.telegram
        case .gitHub: Images.Social.github
        case .youTube: Images.Social.youtube
        case .facebook: Images.Social.facebook
        case .website: Images.Social.website
        case .coingecko: Images.Social.coingecko
        case .coinMarketCap: Images.Social.coinmarketcap
        case .openSea: Images.Social.opensea
        case .instagram: Images.Social.instagram
        case .magicEden: Images.Social.magiceden
        case .tikTok: Images.Social.tiktok
        }
    }
}
