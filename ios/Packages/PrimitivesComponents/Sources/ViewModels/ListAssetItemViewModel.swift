// Copyright (c). Gem Wallet. All rights reserved.

import Components
import Formatters
import Foundation
import func Gemstone.assetListRow
import struct Gemstone.GemAssetBalance
import enum Gemstone.GemAssetBalanceScope
import struct Gemstone.GemAssetListRow
import struct Gemstone.GemAssetListRowInput
import struct Gemstone.GemAssetRowStyle
import GemstonePrimitives
import Primitives
import Style
import SwiftUI

public struct ListAssetItemViewModel: ListAssetItemViewable {
    let assetDataModel: AssetDataViewModel
    let rowStyle: GemAssetRowStyle
    private let row: GemAssetListRow

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
        row = assetListRow(
            input: GemAssetListRowInput(
                asset: assetDataModel.asset.toGem(),
                balance: GemAssetBalance(assetDataModel.assetData.balance, assetId: assetDataModel.asset.id, isActive: assetDataModel.assetData.metadata.isActive),
                scope: .total,
                price: assetDataModel.assetData.price?.price,
                currency: assetDataModel.currency.toGem(),
                style: rowStyle,
            ),
        )
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
        row.text.title
    }

    public var symbol: String? {
        row.text.symbol
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
            row.text.network.map { .type(TextValue(text: $0, style: .calloutSecondary)) } ?? .none
        }
    }

    public var rightView: ListAssetItemRightView {
        switch rowStyle.trailing {
        case .balance:
            .balance(
                balance: TextValue(
                    text: row.amount.text(),
                    style: TextStyle(font: .callout, color: row.hasBalance ? Colors.black : Colors.gray, fontWeight: .semibold),
                ),
                totalFiat: TextValue(
                    text: row.fiat?.text() ?? .empty,
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
