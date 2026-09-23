// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import func Gemstone.applicationHost
import func Gemstone.applicationIconUrl
import func Gemstone.applicationShortName
import Primitives

public extension Primitives.ApplicationMetadata {
    var iconURL: URL? {
        applicationIconUrl(metadata: toGem()).flatMap(URL.init(string:))
    }

    var shortName: String {
        applicationShortName(metadata: toGem())
    }

    var host: String {
        applicationHost(metadata: toGem())
    }
}
