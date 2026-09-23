// Copyright (c). Gem Wallet. All rights reserved.

import Components
import struct Gemstone.GemFiatQuoteRow
import enum Gemstone.GemListRow

struct FiatProviderViewModel {
    let quotesState: StateViewType<[GemFiatQuoteRow]>
    let emptyTitle: String
    let selectedQuote: GemFiatQuoteRow?
    let allowSelectProvider: Bool
    let rateRow: GemListRow?
}
