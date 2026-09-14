// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Formatters
import Foundation
import struct Gemstone.Asset
import enum Gemstone.GemAmountError
import enum Gemstone.GemConfirmError
import enum Gemstone.GemConfirmErrorDisplay
import GemstonePrimitives
import Localization
import Primitives

enum ConfirmTransferError {
    case confirm(GemConfirmError)
    case other(Error)

    init(error: Error) {
        switch error {
        case let error as GemConfirmError where error.display().hasInfoSheet():
            self = .confirm(error)
        default:
            self = .other(error)
        }
    }

    var displayError: Error {
        switch self {
        case let .confirm(error): error
        case let .other(error): error
        }
    }
}

extension GemAmountError: @retroactive LocalizedError {
    public var errorDescription: String? {
        switch display() {
        case .none: nil
        case .invalidAmount: Localized.Errors.invalidAmount
        case let .belowMinimum(asset, minimum):
            Localized.Transfer.minimumAmount(ValueFormatter(style: .auto).string(minimum, asset: asset.toPrimitives()).boldMarkdown())
        case let .insufficientBalance(title):
            Localized.Transfer.insufficientBalance(title.boldMarkdown())
        }
    }
}

extension GemConfirmError: @retroactive LocalizedError {
    public var errorDescription: String? { display().errorDescription }
}

extension GemConfirmErrorDisplay: @retroactive Error {}

extension GemConfirmErrorDisplay: @retroactive LocalizedError {
    public var errorDescription: String? {
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
                Self.amount(requirement.available, asset: asset).boldMarkdown(),
                Self.amount(requirement.shortfall, asset: asset).boldMarkdown(),
            )
        case let .networkFeeRequired(asset, requirement):
            Localized.Info.InsufficientNetworkFeeBalance.description(
                Self.amount(requirement.required, asset: asset).boldMarkdown(),
                asset.toPrimitives().chain.networkName.boldMarkdown(),
                Self.amount(requirement.available, asset: asset).boldMarkdown(),
                Self.amount(requirement.shortfall, asset: asset).boldMarkdown(),
            )
        case let .networkFeeMissing(asset):
            Localized.Transfer.insufficientNetworkFeeBalance(Self.title(asset: asset))
        case let .minimumAccountBalance(asset, required):
            Localized.Transfer.minimumAccountBalance(Self.amount(required, asset: asset).boldMarkdown())
        case let .swapMinimum(asset, _, providerName, requirement):
            Localized.Info.swapMinimumAmountDescription(
                providerName.boldMarkdown(),
                Self.amount(requirement.required, asset: asset).boldMarkdown(),
                Self.amount(requirement.available, asset: asset).boldMarkdown(),
                Self.amount(requirement.shortfall, asset: asset).boldMarkdown(),
            )
        case .dustThreshold: Localized.Errors.dustThresholdShort
        case .insufficientFunds: Localized.Info.InsufficientBalance.title
        case let .message(msg): msg
        }
    }

    private static func amount(_ value: BigInt, asset: Gemstone.Asset) -> String {
        ValueFormatter(style: .full).string(value, asset: asset.toPrimitives())
    }

    private static func title(asset: Gemstone.Asset) -> String {
        let title = asset.name == asset.symbol ? asset.name : String(format: "%@ (%@)", asset.name, asset.symbol)
        return title.boldMarkdown()
    }
}
