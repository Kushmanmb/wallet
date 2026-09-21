// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Formatters
import Foundation
import func Gemstone.assetRowText
import struct Gemstone.GemAssetRowStyle
import struct Gemstone.GemAssetRowText
import GemstonePrimitives
import Primitives
import Style
import SwiftUI

public struct ListAssetItemViewModel: ListAssetItemViewable {
    let assetDataModel: AssetDataViewModel
    let rowStyle: GemAssetRowStyle
    private let text: GemAssetRowText

    public let showBalancePrivacy: Binding<Bool>
    public var action: ((ListAssetItemAction) -> Void)?

    public init(
        showBalancePrivacy: Binding<Bool>,
        assetDataModel: AssetDataViewModel,
        rowStyle: GemAssetRowStyle,
        action: ((ListAssetItemAction) -> Void)? = nil,
    ) {
        self.showBalancePrivacy = showBalancePrivacy
        self.assetDataModel = assetDataModel
        self.rowStyle = rowStyle
        self.action = action
        text = assetRowText(asset: assetDataModel.asset.toGem(), style: rowStyle)
    }

    public init(
        showBalancePrivacy: Binding<Bool>,
        assetData: AssetData,
        formatter: ValueFormatter,
        currency: Currency,
        rowStyle: GemAssetRowStyle,
    ) {
        let model = AssetDataViewModel(
            assetData: assetData,
            formatter: formatter,
            currency: currency,
        )
        self.init(
            showBalancePrivacy: showBalancePrivacy,
            assetDataModel: model,
            rowStyle: rowStyle,
            action: nil,
        )
    }

    public var name: String {
        text.title
    }

    public var symbol: String? {
        text.symbol
    }

    public var subtitleView: ListAssetItemSubtitleView {
        switch rowStyle.subtitle {
        case .price:
            .price(
                price: TextValue(
                    text: assetDataModel.priceAmountText,
                    style: TextStyle(font: .footnote, color: Colors.gray),
                ),
                priceChangePercentage24h: TextValue(
                    text: assetDataModel.priceChangeText,
                    style: TextStyle(font: .footnote, color: assetDataModel.priceChangeTextColor),
                ),
            )
        case .network:
            text.network.map { .type(TextValue(text: $0, style: .calloutSecondary)) } ?? .none
        }
    }

    public var rightView: ListAssetItemRightView {
        switch rowStyle.trailing {
        case .balance:
            .balance(
                balance: TextValue(
                    text: assetDataModel.totalBalanceTextWithSymbol,
                    style: TextStyle(font: .callout, color: assetDataModel.balanceTextColor, fontWeight: .semibold),
                ),
                totalFiat: TextValue(
                    text: assetDataModel.fiatBalanceText,
                    style: TextStyle(font: .footnote, color: Colors.gray),
                ),
            )
        case .toggle:
            .toggle(assetDataModel.isEnabled)
        case .copy:
            .copy
        case .none:
            .none
        }
    }

    public var assetImage: AssetImage {
        AssetViewModel(asset: assetDataModel.asset).assetImage
    }
}
