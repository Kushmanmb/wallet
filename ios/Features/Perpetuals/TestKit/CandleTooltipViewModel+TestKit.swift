// Copyright (c). Gem Wallet. All rights reserved.

import Formatters
import Foundation
import GemstonePrimitives
@testable import Perpetuals
import Primitives

public extension CandleTooltipViewModel {
    static func mock(
        candle: ChartCandleStick = .mock(),
        formatter: NumericFormatter = NumericFormatter(locale: Locale(identifier: "en_US")),
    ) -> CandleTooltipViewModel {
        CandleTooltipViewModel(candle: candle, formatter: formatter)
    }
}
