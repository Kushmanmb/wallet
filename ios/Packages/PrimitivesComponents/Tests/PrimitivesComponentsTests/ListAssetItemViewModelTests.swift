// Copyright (c). Gem Wallet. All rights reserved.

import enum Gemstone.GemSelectAssetType
import Primitives
@testable import PrimitivesComponents
import PrimitivesComponentsTestKit
import PrimitivesTestKit
import SwiftUI
import Testing

struct ListAssetItemViewModelTests {
    @Test
    func receiveCollectionUsesNetworkName() {
        let ton = Asset.mock(
            id: AssetId(chain: .ton, tokenId: nil),
            name: "Gram",
            symbol: "GRAM",
            decimals: 9,
            type: .native,
        )
        let assetDataModel = AssetDataViewModel.mock(assetData: .mock(asset: ton))

        let collectionModel = ListAssetItemViewModel(
            showBalancePrivacy: .constant(false),
            assetDataModel: assetDataModel,
            rowStyle: GemSelectAssetType.receiveCollection.flow().rowStyle,
        )
        let assetModel = ListAssetItemViewModel(
            showBalancePrivacy: .constant(false),
            assetDataModel: assetDataModel,
            rowStyle: GemSelectAssetType.receive.flow().rowStyle,
        )

        #expect(collectionModel.name == "TON")
        #expect(assetModel.name == "Gram")
    }

    @Test
    func theSymbolShowsOnlyWhenItAddsToTheTitleTheRowShows() {
        let usdc = AssetDataViewModel.mock(
            assetData: .mock(asset: .mock(id: AssetId(chain: .ethereum, tokenId: "0xusdc"), name: "USDC", symbol: "USDC", decimals: 6, type: .erc20)),
        )
        let ethereum = AssetDataViewModel.mock(assetData: .mock(asset: .mockEthereum()))
        let model = { (data: AssetDataViewModel, type: GemSelectAssetType) in
            ListAssetItemViewModel(showBalancePrivacy: .constant(false), assetDataModel: data, rowStyle: type.flow().rowStyle)
        }

        #expect(model(usdc, .receive).symbol == nil, "a name that already is the symbol is not repeated")
        #expect(model(ethereum, .receive).symbol == "ETH")
        #expect(model(ethereum, .send).symbol == nil, "the send list shows no symbol at all")
    }

    @Test
    func theNetworkSubtitleNamesTheChainForTokensOnly() {
        let subtitle = { (asset: Asset) -> String? in
            let model = ListAssetItemViewModel(
                showBalancePrivacy: .constant(false),
                assetDataModel: AssetDataViewModel.mock(assetData: .mock(asset: asset)),
                rowStyle: GemSelectAssetType.receive.flow().rowStyle,
            )
            guard case let .type(value) = model.subtitleView else { return nil }
            return value.text
        }

        #expect(subtitle(.mockEthereumUSDT()) == "Ethereum")
        #expect(subtitle(.mockEthereum()) == nil, "a coin's row already names its network")
    }
}
