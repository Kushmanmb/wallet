// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemStakeAmountInput
import Localization

extension GemStakeAmountInput {
    var title: String {
        switch self {
        case .stake: Localized.Transfer.Stake.title
        case .unstake: Localized.Transfer.Unstake.title
        case .redelegate: Localized.Transfer.Redelegate.title
        case .withdraw: Localized.Transfer.Withdraw.title
        case .rewards: Localized.Transfer.ClaimRewards.title
        case .freeze: Localized.Transfer.Freeze.title
        case .unfreeze: Localized.Transfer.Unfreeze.title
        }
    }
}
