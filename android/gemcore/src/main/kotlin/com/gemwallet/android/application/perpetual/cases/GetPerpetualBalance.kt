package com.gemwallet.android.application.perpetual.cases

import com.wallet.core.primitives.PerpetualBalance
import kotlinx.coroutines.flow.Flow
import uniffi.gemstone.GemPerpetualCollateral
import com.gemwallet.android.domains.perpetual.values.PerpetualBalance as PerpetualBalanceDisplay

interface GetPerpetualBalance {
    fun getBalance(): Flow<PerpetualBalance?>

    fun getCollateral(): Flow<GemPerpetualCollateral?>

    fun getDisplayBalance(): Flow<PerpetualBalanceDisplay>
}
