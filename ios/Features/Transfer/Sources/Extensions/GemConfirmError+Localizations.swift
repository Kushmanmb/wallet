// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Formatters
import Foundation
import struct Gemstone.Asset
import enum Gemstone.GemConfirmError
import enum Gemstone.GemConfirmErrorDisplay
import GemstonePrimitives
import Localization
import Primitives

extension GemConfirmError: @retroactive LocalizedError {
    public var errorDescription: String? { display().text }
}

extension GemConfirmErrorDisplay {
    var text: String {
        switch self {
        case .offline: Localized.Errors.networkOffline
        case .malicious: Localized.Errors.ScanTransaction.Malicious.description
        case let .memoRequired(symbol): Localized.Errors.ScanTransaction.memoRequired(symbol.boldMarkdown())
        case .feeRatesMissing: Localized.Errors.unableEstimateNetworkFee
        case .cancelled: Localized.Errors.cancelled
        case .accountMissing: Localized.Errors.walletAccountMissing
        case .unknown: Localized.Errors.unknown
        case let .balanceRequired(asset, requirement):
            Localized.Info.balanceRequiredDescription(
                Self.amount(requirement.required, asset: asset).boldMarkdown(),
                Self.amount(requirement.available, asset: asset),
                Self.amount(requirement.shortfall, asset: asset),
            )
        case let .networkFeeRequired(asset, requirement):
            Localized.Info.InsufficientNetworkFeeBalance.description(
                Self.amount(requirement.required, asset: asset).boldMarkdown(),
                asset.toPrimitives().chain.networkName.boldMarkdown(),
                Self.amount(requirement.available, asset: asset),
                Self.amount(requirement.shortfall, asset: asset),
            )
        case let .networkFeeMissing(asset):
            Localized.Transfer.insufficientNetworkFeeBalance(Self.title(asset: asset))
        case let .minimumAccountBalance(asset, required):
            Localized.Transfer.minimumAccountBalance(Self.amount(required, asset: asset).boldMarkdown())
        case let .swapMinimum(asset, _, providerName, requirement):
            Localized.Info.swapMinimumAmountDescription(
                providerName.boldMarkdown(),
                Self.amount(requirement.required, asset: asset).boldMarkdown(),
                Self.amount(requirement.available, asset: asset),
                Self.amount(requirement.shortfall, asset: asset),
            )
        case .dustThreshold: Localized.Errors.dustThresholdShort
        case .insufficientFunds: Localized.Info.InsufficientBalance.title
        case let .message(msg): msg
        }
    }

    static func amount(_ value: BigInt, asset: Gemstone.Asset) -> String {
        ValueFormatter(style: .full).string(value, asset: asset.toPrimitives())
    }

    private static func title(asset: Gemstone.Asset) -> String {
        let title = asset.name == asset.symbol ? asset.name : String(format: "%@ (%@)", asset.name, asset.symbol)
        return title.boldMarkdown()
    }
}
