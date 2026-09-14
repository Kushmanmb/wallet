package com.gemwallet.android.data.coordinators.tokens

import android.util.Log
import com.gemwallet.android.application.tokens.cases.SearchTokens
import com.gemwallet.android.ext.runCatchingCancellable
import com.gemwallet.android.ext.toIdentifier
import com.wallet.core.primitives.AssetId
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.withContext
import uniffi.gemstone.GemAssetsService

class SearchTokensImpl(
    private val assetsService: GemAssetsService,
    private val ioDispatcher: CoroutineDispatcher,
) : SearchTokens {

    override suspend fun search(assetIds: List<AssetId>): Boolean = withContext(ioDispatcher) {
        runCatchingCancellable { assetsService.syncAssets(assetIds.map { it.toIdentifier() }) }
            .onFailure { Log.e(TAG, "assets sync failed", it) }
            .isSuccess
    }

    private companion object {
        const val TAG = "SearchTokens"
    }
}
