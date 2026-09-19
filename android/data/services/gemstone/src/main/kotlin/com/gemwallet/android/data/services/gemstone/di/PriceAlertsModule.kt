package com.gemwallet.android.data.services.gemstone.di

import com.gemwallet.android.data.service.store.database.PriceAlertsDao
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import com.gemwallet.android.data.services.gemstone.stores.GemstonePriceAlertStore
import uniffi.gemstone.GemDeviceApiClient
import uniffi.gemstone.GemPreferencesService
import uniffi.gemstone.GemDeviceService
import uniffi.gemstone.GemPriceAlertService
import uniffi.gemstone.GemPriceAlertServiceInterface
import uniffi.gemstone.GemPriceAlertStore
import javax.inject.Singleton
import uniffi.gemstone.GemNotificationPermissions
import uniffi.gemstone.PriceAlertFormatter

@InstallIn(SingletonComponent::class)
@Module
object PriceAlertsModule {

    @Singleton
    @Provides
    fun provideGemstonePriceAlertStore(priceAlertsDao: PriceAlertsDao, priceAlertFormatter: PriceAlertFormatter): GemstonePriceAlertStore =
        GemstonePriceAlertStore(priceAlertsDao, priceAlertFormatter)

    @Provides
    @Singleton
    fun provideGemPriceAlertStore(store: GemstonePriceAlertStore): GemPriceAlertStore = store

    @Singleton
    @Provides
    fun provideGemPriceAlertService(
        apiClient: GemDeviceApiClient,
        preferencesService: GemPreferencesService,
        store: GemPriceAlertStore,
        deviceService: GemDeviceService,
        notificationPermissions: GemNotificationPermissions,
    ): GemPriceAlertService = GemPriceAlertService(
        api = apiClient,
        preferences = preferencesService,
        store = store,
        device = deviceService,
        permissions = notificationPermissions,
    )

    @Provides
    fun provideGemPriceAlertServiceInterface(service: GemPriceAlertService): GemPriceAlertServiceInterface = service
}
