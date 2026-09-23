package com.gemwallet.android.data.coordinators.transaction

import com.gemwallet.android.application.session.cases.GetCurrentWalletId
import com.gemwallet.android.application.transactions.cases.GetPendingTransactionsCount
import com.gemwallet.android.application.transactions.cases.TransactionsRequestFilter
import com.gemwallet.android.data.services.gemstone.stores.GemstoneTransactionStore
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flatMapLatest

@OptIn(ExperimentalCoroutinesApi::class)
class GetPendingTransactionsCountImpl(private val getCurrentWalletId: GetCurrentWalletId, private val transactionStore: GemstoneTransactionStore) : GetPendingTransactionsCount {

    override fun getPendingTransactionsCount(): Flow<Int?> = getCurrentWalletId()
        .flatMapLatest { walletId ->
            transactionStore.observeTransactionsCount(
                walletId,
                TransactionsRequestFilter.pendingActivity(),
            )
        }
}
