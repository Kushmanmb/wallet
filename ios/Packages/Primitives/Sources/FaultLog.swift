// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import os

/// Records a fault in every build, including release, so a field report can show that an
/// always-on path dropped work. Never pass wallet data: the message is public in the log.
@inlinable
public func faultLog(_ message: @autoclosure () -> String) {
    os_log("%{public}@", log: .default, type: .fault, message())
}
