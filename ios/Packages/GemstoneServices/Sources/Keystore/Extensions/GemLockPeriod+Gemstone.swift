// Copyright (c). Gem Wallet. All rights reserved.

public import enum Gemstone.GemLockPeriod
import func Gemstone.lockPeriodFromMinutes
import func Gemstone.lockPeriods

extension GemLockPeriod: @retroactive Identifiable {
    public var id: Self {
        self
    }
}

public extension GemLockPeriod {
    static var offered: [GemLockPeriod] {
        lockPeriods()
    }

    static var `default`: GemLockPeriod {
        lockPeriodFromMinutes(minutes: nil)
    }
}
