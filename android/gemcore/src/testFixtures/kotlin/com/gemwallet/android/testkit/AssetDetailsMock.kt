package com.gemwallet.android.testkit

import com.gemwallet.android.ext.toIdentifier
import com.wallet.core.primitives.Asset
import uniffi.gemstone.GemAssetBalanceRow
import uniffi.gemstone.GemAssetDetails
import uniffi.gemstone.GemAssetDetailsState
import uniffi.gemstone.GemSwapPairSuggestion

fun mockGemAssetDetails(
    asset: Asset = mockAsset(),
    state: GemAssetDetailsState = mockGemAssetDetailsState(),
    balanceRows: List<GemAssetBalanceRow> = emptyList(),
) = GemAssetDetails(
    state = state,
    balanceRows = balanceRows,
    title = asset.name,
    explorerName = "Explorer",
    addressLink = null,
    tokenLink = null,
    verificationStatus = null,
    networkDestination = null,
    shareUrl = "",
    swapPair = GemSwapPairSuggestion(payAssetId = asset.id.toIdentifier(), receiveAssetId = null),
)
