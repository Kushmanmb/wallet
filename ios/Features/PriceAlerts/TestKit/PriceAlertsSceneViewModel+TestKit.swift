// Copyright (c). Gem Wallet. All rights reserved.

import GemstonePrimitivesTestKit
import protocol Gemstone.GemPriceAlertServiceProtocol
import PriceAlerts

public extension PriceAlertsSceneViewModel {
    @MainActor
    static func mock(service: any GemPriceAlertServiceProtocol = GemPriceAlertServiceMock()) -> PriceAlertsSceneViewModel {
        PriceAlertsSceneViewModel(service: service)
    }
}
