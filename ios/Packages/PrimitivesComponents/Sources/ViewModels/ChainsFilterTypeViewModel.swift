// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import enum Gemstone.GemChainsFilterSummary
import GemstonePrimitives
import Localization
import Primitives
import Style
import SwiftUI

public struct ChainsFilterTypeViewModel: FilterTypeRepresentable {
    private let summary: GemChainsFilterSummary

    public init(summary: GemChainsFilterSummary) {
        self.summary = summary
    }

    public var value: String {
        switch summary {
        case .all: Localized.Common.all
        case let .chain(chain): Primitives.Chain(core: chain).networkName
        case let .count(count): "\(count)"
        }
    }

    public var title: String {
        Localized.Settings.Networks.title
    }

    public var image: AssetImage {
        AssetImage.image(Images.Settings.networks)
    }
}
