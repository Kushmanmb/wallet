package com.gemwallet.android.features.swap.viewmodels.models

import uniffi.gemstone.SwapperException
import java.math.BigInteger

sealed interface SwapFailure {
    data object UnsupportedAsset : SwapFailure
    data object NoQuote : SwapFailure
    data class AmountTooSmall(val minAmount: BigInteger?) : SwapFailure
    data class Unknown(val message: String) : SwapFailure
}

internal fun Throwable.toSwapFailure(): SwapFailure = when (this) {
    is SwapperException.NotSupportedChain,
    is SwapperException.NotSupportedAsset -> SwapFailure.UnsupportedAsset
    is SwapperException.NoQuoteAvailable,
    is SwapperException.NoAvailableProvider,
    is SwapperException.InvalidRoute,
    is SwapperException.ComputeQuoteException,
    is SwapperException.TransactionException -> SwapFailure.NoQuote
    is SwapperException.InputAmountException -> SwapFailure.AmountTooSmall(minAmount?.toBigIntegerOrNull())
    else -> SwapFailure.Unknown(message.orEmpty())
}
