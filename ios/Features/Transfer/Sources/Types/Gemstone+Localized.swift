// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemAmountTitle
import GemstonePrimitives
import Localization
import Primitives
import PrimitivesComponents

extension GemAmountTitle {
    var title: String {
        switch self {
        case .send: Localized.Transfer.Send.title
        case .deposit: Localized.Wallet.deposit
        case .withdraw: Localized.Wallet.withdraw
        case .stake: Localized.Transfer.Stake.title
        case .unstake: Localized.Transfer.Unstake.title
        case .redelegate: Localized.Transfer.Redelegate.title
        case .rewards: Localized.Transfer.ClaimRewards.title
        case .freeze: Localized.Transfer.Freeze.title
        case .unfreeze: Localized.Transfer.Unfreeze.title
        case let .perpetualOpen(direction): direction.toPrimitives().title
        case let .perpetualIncrease(direction): Localized.Perpetual.increaseDirection(direction.toPrimitives().title)
        case let .perpetualReduce(direction): Localized.Perpetual.reduceDirection(direction.toPrimitives().title)
        }
    }
}
