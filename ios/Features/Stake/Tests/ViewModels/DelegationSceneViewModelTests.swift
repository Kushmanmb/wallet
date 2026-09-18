// Copyright (c). Gem Wallet. All rights reserved.

import GemstonePrimitivesTestKit
import Foundation
import Localization
import Primitives
import PrimitivesTestKit
@testable import Stake
import StakeTestKit
import Testing

struct DelegationSceneViewModelTests {
    @Test
    func rewardsShownWhenCoreReportsThem() {
        let claimable = DelegationSceneViewModel.mock(stakeService: GemStakeServiceMock(claimable: true))
        let notClaimable = DelegationSceneViewModel.mock(stakeService: GemStakeServiceMock(claimable: false))

        #expect(claimable.canClaimRewards == true)
        #expect(notClaimable.canClaimRewards == false)
    }

    @Test
    func rewardsRowOnlyWhenCoreShowsRewards() {
        let shown = DelegationSceneViewModel.mock(rewards: 500_000, stakeService: GemStakeServiceMock(rewardsShown: true))
        let hidden = DelegationSceneViewModel.mock(rewards: 500_000, stakeService: GemStakeServiceMock(rewardsShown: false))

        #expect(shown.rewardsItem?.title == Localized.Stake.rewards)
        #expect(shown.rewardsItem?.subtitle == "0.5 ATOM")
        #expect(hidden.rewardsItem == nil)
    }
}
