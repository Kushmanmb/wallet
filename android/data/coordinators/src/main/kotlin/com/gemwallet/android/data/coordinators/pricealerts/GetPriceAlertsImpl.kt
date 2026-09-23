package com.gemwallet.android.data.coordinators.pricealerts

import com.gemwallet.android.application.assets.cases.GetWalletAssets
import com.gemwallet.android.application.pricealerts.cases.GetPriceAlerts
import com.gemwallet.android.data.services.gemstone.stores.GemstonePriceAlertStore
import com.gemwallet.android.ext.toIdentifier
import com.wallet.core.primitives.AssetId
import com.wallet.core.primitives.Price
import com.wallet.core.primitives.PriceAlert
import com.wallet.core.primitives.PriceAlertData
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.flow.mapLatest

@OptIn(ExperimentalCoroutinesApi::class)
class GetPriceAlertsImpl(private val priceAlertStore: GemstonePriceAlertStore, private val getWalletAssets: GetWalletAssets) : GetPriceAlerts {
    override fun assetPriceAlerts(assetId: AssetId): Flow<List<PriceAlert>> = priceAlertStore.observePriceAlerts(assetId).mapLatest { alerts -> alerts.map { it.priceAlert } }

    override fun invoke(assetId: AssetId?): Flow<List<PriceAlertData>> = priceAlertStore.observePriceAlerts(assetId)
        .flatMapLatest { items ->
            val alerts = items.groupBy { it.priceAlert.assetId.toIdentifier() }
            getWalletAssets.byIdentifiers(alerts.keys.toList()).mapLatest { assetInfos ->
                assetInfos.flatMap { assetInfo ->
                    alerts[assetInfo.id().toIdentifier()]?.map { item ->
                        PriceAlertData(
                            asset = assetInfo.asset,
                            price = assetInfo.price?.price?.let {
                                Price(
                                    price = it.price,
                                    priceChangePercentage24h = it.priceChangePercentage24h,
                                    updatedAt = it.updatedAt,
                                )
                            },
                            priceAlert = item.priceAlert,
                            rankScore = assetInfo.metadata.rankScore,
                        )
                    }.orEmpty()
                }
            }
        }
}
