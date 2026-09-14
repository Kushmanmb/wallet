package com.gemwallet.android.features.swap.views.components

import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ext.boldMarkdown
import com.gemwallet.android.ext.toPrimitives
import com.gemwallet.android.features.swap.viewmodels.models.SwapUiState
import com.gemwallet.android.model.ValueFormatter
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.InfoBottomSheet
import com.gemwallet.android.ui.components.InfoSheetEntity
import com.gemwallet.android.ui.components.list_item.WarningItem
import com.gemwallet.android.ui.models.ListPosition
import com.wallet.core.primitives.Asset
import java.math.BigInteger
import uniffi.gemstone.GemSwapErrorDisplay
import uniffi.gemstone.GemValueStyle

@Composable
internal fun SwapError(state: SwapUiState) {
    var isShowInfoSheet by remember { mutableStateOf(false) }
    val error = state.error ?: return

    val errorText = when (error) {
        is GemSwapErrorDisplay.NotSupportedAsset -> stringResource(R.string.errors_swap_not_supported_asset)
        is GemSwapErrorDisplay.NoQuote -> stringResource(R.string.errors_swap_no_quote_available)
        is GemSwapErrorDisplay.MinimumAmount ->
            stringResource(R.string.errors_swap_minimum_amount, minimumAmount(error.minAmount, error.asset.toPrimitives()).boldMarkdown())
        is GemSwapErrorDisplay.AmountTooSmall -> stringResource(R.string.errors_swap_amount_too_small)
    }

    val infoSheetEntity = when (error) {
        is GemSwapErrorDisplay.NoQuote -> InfoSheetEntity.NoQuoteInfo
        is GemSwapErrorDisplay.NotSupportedAsset,
        is GemSwapErrorDisplay.MinimumAmount,
        is GemSwapErrorDisplay.AmountTooSmall -> null
    }

    WarningItem(
        title = stringResource(R.string.errors_error_occurred),
        message = errorText,
        color = MaterialTheme.colorScheme.error,
        position = ListPosition.Single,
        onClick = infoSheetEntity?.let { { isShowInfoSheet = true } },
    )

    if (isShowInfoSheet && infoSheetEntity != null) {
        InfoBottomSheet(item = infoSheetEntity) { isShowInfoSheet = false }
    }
}

private fun minimumAmount(minAmount: BigInteger, asset: Asset): String =
    ValueFormatter(style = GemValueStyle.AUTO).string(minAmount, asset)
