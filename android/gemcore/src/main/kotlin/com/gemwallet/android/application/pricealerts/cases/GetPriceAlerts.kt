package com.gemwallet.android.application.pricealerts.cases

import com.wallet.core.primitives.AssetId
import com.wallet.core.primitives.PriceAlert
import com.wallet.core.primitives.PriceAlertData
import kotlinx.coroutines.flow.Flow

interface GetPriceAlerts {
    operator fun invoke(assetId: AssetId? = null): Flow<List<PriceAlertData>>

    fun assetPriceAlerts(assetId: AssetId): Flow<List<PriceAlert>>
}
