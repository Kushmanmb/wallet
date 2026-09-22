// Copyright (c). Gem Wallet. All rights reserved.

import BigInt
import Components
import struct Gemstone.GemSimulationValue
import GemstonePrimitives
import GemstonePrimitivesTestKit
@testable import Primitives
import PrimitivesComponents
import PrimitivesComponentsTestKit
import PrimitivesTestKit
import Testing
@testable import Transfer
@testable import TransferTestKit

struct ConfirmHeaderViewModelTests {
    @Test
    func amountShowsClearHeader() {
        let headerType = TransactionHeaderType.amount(
            .numeric(.mock(asset: .mockEthereumUSDT(), price: nil, value: 1)),
        )
        #expect(headerType.showsClearHeader == true)
    }

    @Test
    func swapHidesClearHeader() {
        let headerType = TransactionHeaderType.swap(
            from: SwapAmountField(
                assetId: .mockEthereum(),
                assetImage: AssetImage(),
                amount: "1 ETH",
                fiatAmount: "$1",
            ),
            to: SwapAmountField(
                assetId: Asset.mockEthereumUSDT().id,
                assetImage: AssetImage(),
                amount: "2 USDC",
                fiatAmount: "$2",
            ),
        )
        #expect(headerType.showsClearHeader == false)
    }

    @Test
    func nftShowsClearHeader() {
        #expect(TransactionHeaderType.nft(name: nil, image: AssetImage()).showsClearHeader == true)
    }

    @Test
    func assetShowsClearHeader() {
        #expect(TransactionHeaderType.asset(image: AssetImage()).showsClearHeader == true)
    }

    @Test
    func simulationHeaderDataResolvesAssetValue() {
        let model = ConfirmHeaderViewModel(
            request: .mock(),
            state: .mock(simulation: .mock(headerData: GemSimulationValue(asset: Asset.mockEthereumUSDT().toGem(), value: .exact(value: BigUInt(1_000_000))))),
            currency: .usd,
        )

        guard case let .header(item) = model.itemModel else { return }
        guard case let .assetValue(header) = item.headerType,
              let data = header as? AssetValueHeaderViewModel
        else {
            Issue.record("Expected assetValue header")
            return
        }
        #expect(data.data.asset == Asset.mockEthereumUSDT().toGem())
        #expect(data.data.value == .exact(value: BigUInt(1_000_000)))
        #expect(item.showClearHeader == true)
    }

    @Test
    func tokenApproveResolvesAssetHeader() {
        let model = ConfirmHeaderViewModel(
            request: .mock(data: .mock(type: .tokenApprove(.mock(), .mock(isUnlimited: true)))),
            state: .mock(),
            currency: .usd,
        )

        guard case let .header(item) = model.itemModel else { return }
        guard case let .assetValue(header) = item.headerType,
              let valueHeader = header as? AssetValueHeaderViewModel
        else {
            Issue.record("Expected asset value header")
            return
        }
        #expect(valueHeader.data.value == .unlimited)
        #expect(item.showClearHeader == true)
    }

    @Test
    func genericApprovalKeepsTheAmountHeader() {
        let request = ConfirmTransferRequest.mock(
            data: .mock(type: .generic(asset: .mockEthereum(), metadata: .mock(), extra: .mock())),
            simulation: .mock(header: .init(assetId: Asset.mockEthereumUSDT().id.identifier, value: nil, isUnlimited: true)),
        )
        let waiting = ConfirmHeaderViewModel(request: request, state: .mock(), currency: .usd)
        let loaded = ConfirmHeaderViewModel(
            request: request,
            state: .mock(simulation: .mock(headerData: GemSimulationValue(asset: Asset.mockEthereumUSDT().toGem(), value: .unlimited))),
            currency: .usd,
        )

        guard case let .header(waitingItem) = waiting.itemModel,
              case .assetValue = waitingItem.headerType,
              case let .header(loadedItem) = loaded.itemModel,
              case let .assetValue(model) = loadedItem.headerType,
              let header = model as? AssetValueHeaderViewModel
        else {
            Issue.record("Expected the amount header before and after the value loads")
            return
        }
        #expect(header.data.value == .unlimited)
    }
}
