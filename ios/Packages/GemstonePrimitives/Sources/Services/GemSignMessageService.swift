// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.GemSignMessagePreview
import protocol Gemstone.GemSignMessageServiceProtocol
import Primitives

public extension GemSignMessageServiceProtocol {
    func withAddressNames(chain: Chain, preview: GemSignMessagePreview) async -> GemSignMessagePreview {
        await withAddressNames(chain: chain.rawValue, preview: preview)
    }
}
