package com.gemwallet.android.features.bridge.viewmodels.model

import com.wallet.core.primitives.WalletConnection
import uniffi.gemstone.GemConnectionRow

data class ConnectionRowModel(
    val connection: WalletConnection,
    val row: GemConnectionRow,
)
