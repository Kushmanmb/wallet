package com.gemwallet.android.ui.models

import com.wallet.core.primitives.NFTAssetId
import uniffi.gemstone.GemNftItem
import uniffi.gemstone.GemNftRow
import uniffi.gemstone.nftRows

sealed interface NftItemTarget {
    data class Collection(val id: String) : NftItemTarget
    data class Asset(val id: NFTAssetId) : NftItemTarget
}

data class NftItemUIModel(val row: GemNftRow, val target: NftItemTarget) {
    val imageUrl: String get() = row.imageUrl
    val name: String get() = row.title
    val isVerified: Boolean get() = row.isVerified
    val countText: String? get() = row.countText
}

fun List<GemNftItem>.toUIModels(): List<NftItemUIModel> = zip(nftRows(this)) { item, row ->
    when (item) {
        is GemNftItem.Collection -> NftItemUIModel(row, NftItemTarget.Collection(item.data.collection.id))
        is GemNftItem.Asset -> NftItemUIModel(row, NftItemTarget.Asset(NFTAssetId(item.data.asset.id)))
    }
}
