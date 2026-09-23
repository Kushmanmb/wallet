// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import enum Gemstone.GemTransactionsFilterSummary
import Localization
import PrimitivesComponents
import Style
import SwiftUI

public struct TransactionsFilterTypeViewModel: FilterTypeRepresentable {
    private let summary: GemTransactionsFilterSummary

    public init(summary: GemTransactionsFilterSummary) {
        self.summary = summary
    }

    public var value: String {
        switch summary {
        case .all: Localized.Common.all
        case let .filter(filter): filter.title
        case let .count(count): "\(count)"
        }
    }

    public var title: String {
        Localized.Filter.types
    }

    public var image: AssetImage {
        AssetImage.image(Images.System.textPageFill)
    }
}
