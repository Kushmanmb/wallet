// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemSwapProgressStep
import Localization

extension GemSwapProgressStep {
    var tagTitle: String? {
        switch self {
        case .completed: Localized.Transaction.Status.completed
        case .pending: Localized.Transaction.Status.inprogress
        case .waiting: nil
        case .failed: Localized.Transaction.Status.failed
        case .reverted: Localized.Transaction.Status.reverted
        case .refunded: Localized.Transaction.Status.refunded
        }
    }
}
