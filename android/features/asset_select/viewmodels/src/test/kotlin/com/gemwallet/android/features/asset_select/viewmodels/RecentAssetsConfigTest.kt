package com.gemwallet.android.features.asset_select.viewmodels

import com.gemwallet.android.model.RecentAssetsRequest
import com.wallet.core.primitives.RecentActivityType
import org.junit.Assert.assertEquals
import org.junit.Test
import uniffi.gemstone.GemAssetFilter

class RecentAssetsConfigTest {

    @Test
    fun `request defaults to all types with no filters`() {
        val request = RecentAssetsRequest()
        assertEquals(RecentActivityType.entries, request.types)
        assertEquals(emptySet<GemAssetFilter>(), request.filters)
    }

    @Test
    fun `request with filters preserves them`() {
        val request = RecentAssetsRequest(filters = setOf(GemAssetFilter.Buyable, GemAssetFilter.HasBalance))
        assertEquals(setOf(GemAssetFilter.Buyable, GemAssetFilter.HasBalance), request.filters)
    }
}
