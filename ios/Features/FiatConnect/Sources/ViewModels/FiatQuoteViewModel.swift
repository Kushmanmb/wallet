// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Foundation
import struct Gemstone.GemFiatQuoteRow
import GemstonePrimitives
import Primitives
import PrimitivesComponents
import Style
import SwiftUI

struct FiatQuoteViewModel {
    let row: GemFiatQuoteRow
    let isSelected: Bool

    private let locale: Locale

    init(
        row: GemFiatQuoteRow,
        isSelected: Bool = false,
        locale: Locale = .current,
    ) {
        self.row = row
        self.isSelected = isSelected
        self.locale = locale
    }

    var title: String {
        row.providerName
    }

    var amountText: String {
        row.cryptoAmount.text(locale: locale)
    }
}

extension FiatQuoteViewModel: Identifiable {
    var id: String {
        row.quoteId
    }
}

// MARK: - SimpleListItemViewable

extension FiatQuoteViewModel: SimpleListItemViewable {
    var titleStyle: TextStyle {
        TextStyle(font: .callout, color: Colors.black, fontWeight: .semibold)
    }

    var assetImage: AssetImage {
        AssetImage(
            placeholder: row.provider.toPrimitives().image,
            chainPlaceholder: isSelected ? Images.Wallets.selected : nil,
        )
    }

    var subtitle: String? {
        amountText
    }

    var subtitleExtra: String? {
        row.fiatAmount.text(locale: locale)
    }

    var subtitleStyle: TextStyle {
        TextStyle(font: .callout, color: Colors.black, fontWeight: .semibold)
    }

    var subtitleStyleExtra: TextStyle {
        TextStyle(font: .footnote, color: Colors.gray)
    }
}

// MARK: - Hashable

extension FiatQuoteViewModel: Hashable {
    func hash(into hasher: inout Hasher) {
        hasher.combine(id)
    }
}
