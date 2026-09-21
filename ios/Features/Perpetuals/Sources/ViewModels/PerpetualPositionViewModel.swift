// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Formatters
import Foundation
import struct Gemstone.GemPerpetualPositionRow
import func Gemstone.perpetualPositionRow
import GemstonePrimitives
import Primitives
import PrimitivesComponents
import SwiftUI

public struct PerpetualPositionViewModel {
    public let data: PerpetualPositionData
    private let row: GemPerpetualPositionRow

    public init(_ data: PerpetualPositionData) {
        self.data = data
        row = perpetualPositionRow(perpetual: data.perpetual.toGem(), asset: data.asset.toGem(), position: data.position.toGem())
    }

    public var assetImage: AssetImage {
        AssetIdViewModel(assetId: data.perpetual.assetId).assetImage
    }

    public var symbolText: String {
        row.title
    }

    public var leverageText: String {
        row.leverage
    }

    public var directionText: String {
        PerpetualDirectionViewModel(direction: data.position.direction).title
    }

    public var positionTypeText: String {
        row.position.text
    }

    public var positionTypeColor: Color {
        PerpetualDirectionViewModel(direction: data.position.direction).color
    }

    public var pnlColor: Color {
        row.pnlTone.color
    }

    public var pnlWithPercentText: String {
        row.pnl.text
    }

    public var marginAmountText: String {
        row.margin.text()
    }
}

extension PerpetualPositionViewModel: Identifiable {
    public var id: String {
        data.position.id
    }
}
