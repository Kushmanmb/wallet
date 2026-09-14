package com.gemwallet.android

import android.net.Uri
import uniffi.gemstone.DocsUrl
import uniffi.gemstone.PublicUrl

object AppUrl {
    private const val UTM_SOURCE = "gemwallet_android"

    fun docs(item: DocsUrl): String = item.url().withUTM()

    fun page(item: PublicUrl): String = item.url().withUTM()

    fun staking(chain: String): String = docs(DocsUrl.Staking(chain))

    val accountMinimalBalance: String by lazy { docs(DocsUrl.AccountMinimalBalance) }
    val addCustomToken: String by lazy { docs(DocsUrl.AddCustomToken) }
    val dust: String by lazy { docs(DocsUrl.Dust) }
    val howToSecureSecretPhrase: String by lazy { docs(DocsUrl.HowToSecureSecretPhrase) }
    val migrateWallet: String by lazy { docs(DocsUrl.MigrateWallet) }
    val networkFees: String by lazy { docs(DocsUrl.NetworkFees) }
    val noQuotes: String by lazy { docs(DocsUrl.NoQuotes) }
    val perpetualsAutoclose: String by lazy { docs(DocsUrl.PerpetualsAutoclose) }
    val perpetualsFundingPayments: String by lazy { docs(DocsUrl.PerpetualsFundingPayments) }
    val perpetualsFundingRate: String by lazy { docs(DocsUrl.PerpetualsFundingRate) }
    val perpetualsLiquidationPrice: String by lazy { docs(DocsUrl.PerpetualsLiquidationPrice) }
    val perpetualsOpenInterest: String by lazy { docs(DocsUrl.PerpetualsOpenInterest) }
    val priceImpact: String by lazy { docs(DocsUrl.PriceImpact) }
    val rootedDevice: String by lazy { docs(DocsUrl.RootedDevice) }
    val slippage: String by lazy { docs(DocsUrl.Slippage) }
    val stakingApr: String by lazy { docs(DocsUrl.StakingApr) }
    val stakingLockTime: String by lazy { docs(DocsUrl.StakingLockTime) }
    val tokenVerification: String by lazy { docs(DocsUrl.TokenVerification) }
    val transactionStatus: String by lazy { docs(DocsUrl.TransactionStatus) }
    val walletConnect: String by lazy { docs(DocsUrl.WalletConnect) }
    val whatIsSecretPhrase: String by lazy { docs(DocsUrl.WhatIsSecretPhrase) }
    val whatIsWatchWallet: String by lazy { docs(DocsUrl.WhatIsWatchWallet) }

    private fun String.withUTM(): String = Uri.parse(this)
        .buildUpon()
        .appendQueryParameter("utm_source", UTM_SOURCE)
        .build()
        .toString()
}
