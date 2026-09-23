package com.gemwallet.android.features.asset.viewmodels.chart.viewmodels

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.gemwallet.android.application.IoDispatcher
import com.gemwallet.android.application.assets.cases.GetAssetLinks
import com.gemwallet.android.application.assets.cases.GetAssetMarket
import com.gemwallet.android.application.assets.cases.GetAssetTokenInfo
import com.gemwallet.android.application.assets.cases.GetWalletAssets
import com.gemwallet.android.application.pricealerts.cases.GetPriceAlerts
import com.gemwallet.android.application.session.cases.GetCurrentCurrency
import com.gemwallet.android.ext.toGem
import com.gemwallet.android.model.AssetInfo
import com.gemwallet.android.ui.models.navigation.requireAssetId
import com.wallet.core.primitives.AssetId
import com.wallet.core.primitives.AssetLink
import com.wallet.core.primitives.PriceAlert
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.flowOn
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.mapLatest
import kotlinx.coroutines.flow.stateIn
import uniffi.gemstone.AssetMarket
import uniffi.gemstone.GemChartServiceInterface
import uniffi.gemstone.GemListSection
import javax.inject.Inject

@OptIn(ExperimentalCoroutinesApi::class)
@HiltViewModel
class AssetChartViewModel internal constructor(
    getAssetTokenInfo: GetAssetTokenInfo,
    getAssetLinks: GetAssetLinks,
    getAssetMarket: GetAssetMarket,
    getWalletAssets: GetWalletAssets,
    private val chartService: GemChartServiceInterface,
    getPriceAlerts: GetPriceAlerts,
    getCurrentCurrency: GetCurrentCurrency,
    private val ioDispatcher: CoroutineDispatcher,
    val assetId: AssetId,
) : ViewModel() {

    private val storedAssetInfo: AssetInfo? = getWalletAssets().value.firstOrNull { it.asset.id == assetId }

    private val assetInfo = getAssetTokenInfo(assetId)
        .stateIn(viewModelScope, SharingStarted.Eagerly, storedAssetInfo)

    private val links = getAssetLinks(assetId)
    private val market = getAssetMarket(assetId)
    private val priceAlerts = getPriceAlerts(assetId).map { alerts -> alerts.map { it.priceAlert } }

    val title = assetInfo
        .map { it?.asset?.name.orEmpty() }
        .distinctUntilChanged()
        .stateIn(viewModelScope, SharingStarted.Eagerly, storedAssetInfo?.asset?.name.orEmpty())

    private val marketInCurrency = combine(market, getCurrentCurrency.getCurrency()) { market, _ -> market }
        .mapLatest { market -> market?.let { chartService.marketInCurrency(it.toGem()) } }

    val sections = combine(assetInfo, links, marketInCurrency, priceAlerts) { info, assetLinks, assetMarket, alerts ->
        sections(info, assetLinks, assetMarket, alerts)
    }
        .flowOn(ioDispatcher)
        .stateIn(viewModelScope, SharingStarted.Eagerly, sections(storedAssetInfo, emptyList(), null, emptyList()))

    private fun sections(assetInfo: AssetInfo?, links: List<AssetLink>, market: AssetMarket?, priceAlerts: List<PriceAlert>): List<GemListSection> = assetInfo?.let {
        chartService.sections(
            asset = it.asset.toGem(),
            price = it.price?.price?.price,
            market = market,
            priceAlerts = priceAlerts.map { alert -> alert.toGem() },
            links = links.map { link -> link.toGem() },
        )
    }.orEmpty()

    @Inject
    constructor(
        getAssetTokenInfo: GetAssetTokenInfo,
        getAssetLinks: GetAssetLinks,
        getAssetMarket: GetAssetMarket,
        getWalletAssets: GetWalletAssets,
        chartService: GemChartServiceInterface,
        getPriceAlerts: GetPriceAlerts,
        getCurrentCurrency: GetCurrentCurrency,
        @IoDispatcher ioDispatcher: CoroutineDispatcher,
        savedStateHandle: SavedStateHandle,
    ) : this(
        getAssetTokenInfo = getAssetTokenInfo,
        getAssetLinks = getAssetLinks,
        getAssetMarket = getAssetMarket,
        getWalletAssets = getWalletAssets,
        chartService = chartService,
        getPriceAlerts = getPriceAlerts,
        getCurrentCurrency = getCurrentCurrency,
        ioDispatcher = ioDispatcher,
        assetId = savedStateHandle.requireAssetId(),
    )
}
