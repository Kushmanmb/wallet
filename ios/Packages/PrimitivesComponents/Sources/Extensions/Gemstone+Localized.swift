// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.PerpetualDirection
import enum Gemstone.FeeOption
import enum Gemstone.GemFiatTransactionBadge
import enum Gemstone.GemSimulationWarningKind
import enum Gemstone.GemTransactionTitle
import enum Gemstone.GemWalletSubtitle
import GemstonePrimitives
import Localization
import Primitives


extension FeeOption {
    public var title: String {
        switch self {
        case .tokenAccountCreation: Localized.Banner.AccountActivation.title
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
