// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import class Gemstone.GemApplicationMetadataService
import struct Gemstone.GemConnectionRow
import GemstonePrimitives
import Primitives
import PrimitivesComponents

public struct WalletConnectionViewModel: Sendable {
    let connection: WalletConnection
    private let row: GemConnectionRow

    init(connection: WalletConnection) {
        self.connection = connection
        row = GemApplicationMetadataService.shared.connectionRow(metadata: connection.session.metadata.toGem())
    }

    var nameText: String {
        row.title
    }

    var imageUrl: URL? {
        row.iconUrl.flatMap(URL.init(string:))
    }

    var hostText: String? {
        row.host
    }

    var url: URL? {
        URL(string: connection.session.metadata.url)
    }
}
