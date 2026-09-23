// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import func Gemstone.activityFilters
import struct Gemstone.GemActivityFilters
import enum Gemstone.GemTransactionFilter
import GemstonePrimitives
import Primitives
import PrimitivesComponents
import Store

public extension TransactionsRequestFilter {
    static var activityDefaults: [TransactionsRequestFilter] {
        activity(chains: [], filters: [])
    }

    static var pendingActivity: [TransactionsRequestFilter] {
        let activity = activityFilters(chains: [], filters: [])
        return requestFilters(activity) + [.states(activity.pendingStates.map { $0.toPrimitives().rawValue })]
    }

    static func activity(chains: [Chain], filters: [GemTransactionFilter]) -> [TransactionsRequestFilter] {
        requestFilters(activityFilters(chains: chains.map(\.rawValue), filters: filters))
    }

    private static func requestFilters(_ activity: GemActivityFilters) -> [TransactionsRequestFilter] {
        var request: [TransactionsRequestFilter] = [.assetRankGreaterThan(activity.assetRankGreaterThan.asInt)]
        if activity.chains.isNotEmpty {
            request.append(.chains(activity.chains))
        }
        request.append(.types(activity.transactionTypes.map { $0.toPrimitives().rawValue }))
        return request
    }
}
