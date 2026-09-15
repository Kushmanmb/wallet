// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import enum Gemstone.GemWalletImportKind
import Localization
import Primitives

extension GemWalletImportKind {
    var title: String {
        switch self {
        case .phrase: Localized.Common.phrase
        case .privateKey: Localized.Common.privateKey
        case .address: Localized.Common.address
        }
    }

    var description: String {
        switch self {
        case .phrase: Localized.Common.secretPhrase
        case .privateKey: Localized.Common.privateKey
        case .address: Localized.Common.address
        }
    }
}

extension WalletSource {
    var title: String {
        switch self {
        case .create: Localized.Wallet.New.title
        case .import: Localized.Wallet.Import.title
        }
    }
}
