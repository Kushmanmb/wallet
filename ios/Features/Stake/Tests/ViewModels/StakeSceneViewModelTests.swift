// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import GemstonePrimitives
import struct Gemstone.GemDurationPart
import enum Gemstone.GemListRow
import struct Gemstone.GemTransferData
import Localization
import Primitives
import PrimitivesTestKit
@testable import Stake
import GemstonePrimitivesTestKit
import GemstoneServices
import GemstoneServicesTestKit
import StakeTestKit
@testable import Store
import Testing

@MainActor
struct StakeSceneViewModelTests {
    @Test
    func theInfoSectionShowsTheRowsCoreReturns() {
        let rows: [GemListRow] = [
            .amount(title: .stakeApr, amount: .mock(value: 12.5, tone: .positive), info: .stakeApr),
            .duration(title: .lockTime, parts: [GemDurationPart(value: 14, unit: .day)], info: .stakeLockTime),
        ]
        let model = StakeSceneViewModel.mock(chain: .tron, stakeService: GemStakeServiceMock(infoRows: rows))

        #expect(model.infoRows == rows)
    }

    @Test
    func theInfoSheetMatchesTheRowThatOpenedIt() {
        let model = StakeSceneViewModel.mock(chain: .tron)

        model.onInfo(.stakeApr)
        #expect(model.isPresentingInfoSheet?.id == "stakeApr")

        model.onInfo(.stakeLockTime)
        #expect(model.isPresentingInfoSheet?.id == "stakeLockTime")
    }

    @Test
    func stakeStillRequiresValidators() {
        let tron = StakeSceneViewModel.mock(chain: .tron)
        tron.assetQuery.value = .mock(asset: Chain.tron.asset, balance: .mock(frozen: 1))

        let stake = tron.actions.first { $0.action == .stake }
        #expect(stake?.isEnabled == false)
        #expect(stake?.requiresFrozenBalance == false)
    }

    @Test
    func claimRewardsRoutesToConfirmInput() {
        let transfer = GemTransferData.mock()
        let model = StakeSceneViewModel.mock(chain: .tron, stakeService: GemStakeServiceMock(claimRewardsDestination: .transfer(transfer: transfer)))

        #expect(model.destination(for: .claimRewards) as? ConfirmTransferInput == ConfirmTransferInput(data: transfer))
    }

    @Test
    func claimRewardsAcrossValidatorsRoutesToAmount() {
        let model = StakeSceneViewModel.mock(chain: .tron)

        #expect(model.destination(for: .claimRewards) is AmountInput)
    }

}
