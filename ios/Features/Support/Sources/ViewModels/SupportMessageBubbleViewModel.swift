// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import enum Gemstone.GemSupportMessageOutcome
import struct Gemstone.GemSupportMessageRow
import struct Gemstone.SupportMessageLink
import GemstonePrimitives
import Primitives
import Style
import SwiftUI

struct SupportMessageBubbleViewModel: Identifiable {
    private let row: GemSupportMessageRow
    private let message: SupportMessage
    private let retryAction: (SupportMessage) -> Void
    private let imageAction: (SupportMessageImage) -> Void

    init(
        row: GemSupportMessageRow,
        retryAction: @escaping (SupportMessage) -> Void,
        imageAction: @escaping (SupportMessageImage) -> Void,
    ) {
        self.row = row
        message = row.message.toPrimitives()
        self.retryAction = retryAction
        self.imageAction = imageAction
    }

    var id: String { message.id }
    var content: String { message.content.trim() }
    var displayText: String { row.content.text }
    var links: [SupportMessageLink] { row.content.links }
    var hasContent: Bool { hasDisplayText || hasLinks }
    var hasDisplayText: Bool { displayText.isNotEmpty }
    var hasLinks: Bool { links.isNotEmpty }
    var hasImages: Bool { message.images.isNotEmpty }
    var images: [SupportMessageImage] { message.images }
    var isSending: Bool { outcome == .sending }
    var isFailed: Bool {
        if case .failed = outcome {
            true
        } else {
            false
        }
    }

    var palette: Palette {
        switch message.sender {
        case .user: Palette(text: Colors.whiteSolid, background: Colors.blue, secondary: Colors.whiteSolid, link: Colors.whiteSolid)
        case .agent: Palette(text: Colors.black, background: Colors.white, secondary: Colors.secondaryText, link: Colors.blue)
        }
    }

    var alignment: Alignment {
        switch message.sender {
        case .user: .trailing
        case .agent: .leading
        }
    }

    var time: String { message.createdAt.formatted(date: .omitted, time: .shortened) }

    var outcome: GemSupportMessageOutcome {
        row.outcome
    }

    func retry() {
        retryAction(message)
    }

    func imageURL(for image: SupportMessageImage) -> URL? {
        image.url.asURL
    }

    func onImageTap(_ image: SupportMessageImage) {
        imageAction(image)
    }
}

// MARK: - Types

extension SupportMessageBubbleViewModel {
    struct Palette {
        let text: Color
        let background: Color
        let secondary: Color
        let link: Color
    }
}
