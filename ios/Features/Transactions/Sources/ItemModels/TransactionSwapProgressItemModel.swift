// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemSwapProgressStep
import Localization
import Style
import SwiftUI

public struct TransactionSwapProgressItemModel: Equatable {
    public struct Step: Equatable {
        public let title: String
        public let subtitle: String
        public let status: GemSwapProgressStep

        public init(
            title: String,
            subtitle: String,
            status: GemSwapProgressStep,
        ) {
            self.title = title
            self.subtitle = subtitle
            self.status = status
        }
    }

    public let transfer: Step
    public let swap: Step
    public let estimatedTime: String?

    public init(
        transfer: Step,
        swap: Step,
        estimatedTime: String?,
    ) {
        self.transfer = transfer
        self.swap = swap
        self.estimatedTime = estimatedTime
    }
}

extension GemSwapProgressStep {
    var color: Color {
        switch self {
        case .completed: Colors.green
        case .pending: Colors.blue
        case .waiting: Colors.gray
        case .failed: Colors.red
        case .reverted: Colors.red
        case .refunded: Colors.orange
        }
    }

    var background: Color {
        color.opacity(.light)
    }

    var lineColor: Color {
        switch self {
        case .completed: Colors.green
        case .pending, .waiting, .failed, .reverted, .refunded: Colors.gray.opacity(.medium)
        }
    }

    var markerBackground: Color {
        switch self {
        case .completed, .failed, .reverted, .refunded: background
        case .pending, .waiting: .clear
        }
    }
}
