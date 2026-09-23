package com.gemwallet.android.model

import com.wallet.core.primitives.RecentActivityType
import uniffi.gemstone.GemAssetFilter

data class RecentAssetsRequest(val types: List<RecentActivityType> = RecentActivityType.entries, val filters: Set<GemAssetFilter> = emptySet(), val limit: Int = 10)
