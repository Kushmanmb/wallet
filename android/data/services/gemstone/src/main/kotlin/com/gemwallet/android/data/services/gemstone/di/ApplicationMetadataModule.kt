package com.gemwallet.android.data.services.gemstone.di

import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import uniffi.gemstone.GemApplicationMetadataService
import uniffi.gemstone.GemApplicationMetadataServiceInterface
import javax.inject.Singleton

@InstallIn(SingletonComponent::class)
@Module
object ApplicationMetadataModule {

    @Provides
    @Singleton
    fun provideGemApplicationMetadataService(): GemApplicationMetadataService = GemApplicationMetadataService()

    @Provides
    fun provideGemApplicationMetadataServiceInterface(
        service: GemApplicationMetadataService,
    ): GemApplicationMetadataServiceInterface = service
}
