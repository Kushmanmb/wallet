// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.PerpetualDirection
import enum Gemstone.FeeOption
import enum Gemstone.GemAssetInfoKind
import enum Gemstone.GemFiatTransactionBadge
import enum Gemstone.GemHeaderButtonKind
import enum Gemstone.GemLocalizedText
import enum Gemstone.GemPriceAlertLabel
import struct Gemstone.GemPriceAlertRow
import enum Gemstone.GemPriceAlertText
import enum Gemstone.GemSimulationWarningKind
import enum Gemstone.GemTransactionTitle
import enum Gemstone.GemWalletSubtitle
import GemstonePrimitives
import Localization
import Primitives
import Style
import SwiftUI


extension FeeOption {
    public var title: String {
        switch self {
        case .tokenAccountCreation: Localized.Banner.AccountActivation.title
        }
    }
}

extension GemLocalizedText {
    public var text: String {
        switch self {
        case let .walletDefaultName(index):
            Localized.Wallet.defaultName(Int(index))
        case let .walletDefaultNameChain(chain, index):
            Localized.Wallet.defaultNameChain(Chain(core: chain).networkName, Int(index))
        }
    }
}

extension GemPriceAlertText {
    public var text: String {
        switch self {
        case .empty: Placeholder.empty
        case let .number(value): value.text()
        case let .label(label): label.text
        }
    }
}

extension GemPriceAlertRow {
    public var prefixText: String {
        prefix.text
    }

    public var suffixText: String {
        suffix.text
    }
}

extension GemPriceAlertLabel {
    public var text: String {
        switch self {
        case .over: Localized.PriceAlerts.Direction.over
        case .under: Localized.PriceAlerts.Direction.under
        case .increasesBy: Localized.PriceAlerts.Direction.increasesBy
        case .decreasesBy: Localized.PriceAlerts.Direction.decreasesBy
        }
    }
}

extension GemTransactionTitle {
    public var title: String {
        switch self {
        case .received: Localized.Transaction.Title.received
        case .sent: Localized.Transaction.Title.sent
        case .transfer: Localized.Transfer.title
        case .smartContract: Localized.Transfer.SmartContract.title
        case .swap: Localized.Wallet.swap
        case .approve: Localized.Transfer.Approve.title
        case .stake: Localized.Transfer.Stake.title
        case .unstake: Localized.Transfer.Unstake.title
        case .redelegate: Localized.Transfer.Redelegate.title
        case .rewards: Localized.Transfer.Rewards.title
        case .withdraw: Localized.Transfer.Withdraw.title
        case .activateAsset: Localized.Transfer.ActivateAsset.title
        case .freeze: Localized.Transfer.Freeze.title
        case .unfreeze: Localized.Transfer.Unfreeze.title
        case .earn: Localized.Common.earn
        case let .perpetualOpen(direction):
            Self.perpetualTitle(direction, Localized.Perpetual.openDirection, Localized.Perpetual.position)
        case let .perpetualClose(direction):
            Self.perpetualTitle(direction, Localized.Perpetual.closeDirection, Localized.Perpetual.closePosition)
        case .perpetualModify: Localized.Perpetual.modify
        }
    }

    private static func perpetualTitle(
        _ direction: Gemstone.PerpetualDirection?,
        _ directionTitle: (String) -> String,
        _ fallback: String,
    ) -> String {
        guard let direction else { return fallback }
        return directionTitle(PerpetualDirectionViewModel(direction: direction.toPrimitives()).title)
    }
}

extension GemWalletSubtitle {
    public var text: String {
        switch self {
        case .multicoin: Localized.Wallet.multicoin
        case let .address(value): value
        }
    }
}

extension GemSimulationWarningKind {
    var warningTitle: String {
        switch self {
        case .unlimitedApproval: Localized.Simulation.Warning.UnlimitedTokenApproval.title
        case .nftCollectionApproval: Localized.Simulation.Warning.NftCollectionApproval.title
        case .externallyOwnedSpender: Localized.Common.warning
        case .suspiciousSpender, .validationError: Localized.Errors.errorOccurred
        }
    }

    var defaultMessage: String? {
        switch self {
        case .unlimitedApproval: Localized.Simulation.Warning.UnlimitedTokenApproval.description
        case .validationError: Localized.Errors.errorOccurred
        case .externallyOwnedSpender: Localized.Simulation.warningExternallyOwnedSpenderDescription
        case .suspiciousSpender: Localized.Common.suspiciousAddress
        case .nftCollectionApproval: nil
        }
    }
}

extension GemFiatTransactionBadge {
    public var text: String {
        switch self {
        case .pending: Localized.Transaction.Status.pending
        case .failed: Localized.Transaction.Status.failed
        }
    }
}

extension GemHeaderButtonKind {
    public var title: String {
        switch self {
        case .send: Localized.Wallet.send
        case .receive: Localized.Wallet.receive
        case .buy: Localized.Wallet.buy
        case .swap: Localized.Wallet.swap
        case .deposit: Localized.Wallet.deposit
        case .withdraw: Localized.Wallet.withdraw
        case .more: Localized.Wallet.more
        }
    }
}

extension GemAssetInfoKind {
    public var title: String {
        switch self {
        case .name: Localized.Asset.name
        case .symbol: Localized.Asset.symbol
        case .decimals: Localized.Asset.decimals
        case .kind: Localized.Common.type
        }
    }
}
