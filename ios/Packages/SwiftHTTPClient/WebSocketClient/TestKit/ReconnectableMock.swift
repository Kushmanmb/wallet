// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import WebSocketClient

public struct ReconnectableMock: Reconnectable {
    private let delayMilliseconds: UInt64

    public init(delayMilliseconds: UInt64 = 0) {
        self.delayMilliseconds = delayMilliseconds
    }

    public func reconnectDelayMilliseconds(attempt _: UInt32) -> UInt64 {
        delayMilliseconds
    }

    public func pingIntervalMilliseconds() -> UInt64 {
        0
    }
}
