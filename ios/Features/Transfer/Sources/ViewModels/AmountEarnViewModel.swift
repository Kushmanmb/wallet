// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Foundation
import enum Gemstone.EarnType
import protocol Gemstone.GemAmountServiceProtocol
import enum Gemstone.GemAmountType
import struct Gemstone.GemTransferData
import struct Gemstone.GemValidatorRow
import func Gemstone.validatorRow
import GemstonePrimitives
import Localization
import Primitives

public final class AmountEarnViewModel: AmountDataProvidable {
    let asset: Asset
    let action: Gemstone.EarnType
    private let service: any GemAmountServiceProtocol

    init(asset: Asset, action: Gemstone.EarnType, service: any GemAmountServiceProtocol) {
        self.asset = asset
        self.action = action
        self.service = service
    }

    var provider: DelegationValidator {
        switch action {
        case let .deposit(provider): provider.toPrimitives()
        case let .withdraw(delegation): delegation.validator.toPrimitives()
        }
    }

    var providerRow: GemValidatorRow {
        validatorRow(validator: provider.toGem())
    }

    var providerTitle: String {
        Localized.Common.provider
    }

    var title: String {
        gemAmountType.title().title
    }

    var gemAmountType: GemAmountType {
        service.earnAmountType(earnType: action)
    }

    func makeTransferData(value: BigInt, useMaxAmount: Bool) async throws -> GemTransferData {
        try await service.earnTransferData(asset: asset.toGem(), earnType: action, value: value, useMaxAmount: useMaxAmount)
    }
}
