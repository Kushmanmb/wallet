package com.gemwallet.android.model

import com.gemwallet.android.ext.requireChain
import com.wallet.core.primitives.Chain
import uniffi.gemstone.GemAssetFilter

fun Collection<GemAssetFilter>.chains(): List<Chain> = filterIsInstance<GemAssetFilter.Chains>().flatMap { filter -> filter.chains.map { it.requireChain() } }

fun Collection<GemAssetFilter>.chainsOrAssetIds(): GemAssetFilter.ChainsOrAssetIds? = filterIsInstance<GemAssetFilter.ChainsOrAssetIds>()
    .takeIf { it.isNotEmpty() }
    ?.let { filters -> GemAssetFilter.ChainsOrAssetIds(filters.flatMap { it.chains }, filters.flatMap { it.assetIds }) }
