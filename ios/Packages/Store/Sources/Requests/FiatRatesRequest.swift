// Copyright (c). Gem Wallet. All rights reserved.

import GRDB
import Primitives

public struct FiatRatesRequest: DatabaseQueryable, Equatable {
    public init() {}

    public func fetch(_ db: Database) throws -> [Currency: Double] {
        try FiatRateRecord
            .fetchAll(db)
            .reduce(into: [:]) { $0[$1.symbol] = $1.rate }
    }
}
