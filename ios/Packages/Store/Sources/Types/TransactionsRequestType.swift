// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import Primitives

public enum TransactionsRequestType: Equatable {
    case all
    case asset(assetId: AssetId)
    case transaction(id: String)
}

extension TransactionsRequestType: Identifiable {
    public var id: String {
        switch self {
        case .all: "all"
        case let .transaction(id): id
        case let .asset(asset): asset.identifier
        }
    }
}

extension TransactionsRequestType: Sendable {}
