package com.gemwallet.android.features.swap.views.components

import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.features.swap.viewmodels.models.SwapFailure
import com.gemwallet.android.features.swap.viewmodels.models.SwapUiState
import com.gemwallet.android.model.AssetInfo
import com.gemwallet.android.model.ValueFormatter
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.InfoBottomSheet
import com.gemwallet.android.ui.components.InfoSheetEntity
import com.gemwallet.android.ui.components.list_item.WarningItem
import com.gemwallet.android.ui.models.ListPosition
import com.wallet.core.primitives.Asset
import java.math.BigInteger

@Composable
internal fun SwapError(state: SwapUiState, pay: AssetInfo?) {
    var isShowInfoSheet by remember { mutableStateOf(false) }
    val error = state.error ?: return

    val errorText = when (error) {
        SwapFailure.UnsupportedAsset -> stringResource(R.string.errors_swap_not_supported_asset)
        SwapFailure.NoQuote -> stringResource(R.string.errors_swap_no_quote_available)
        is SwapFailure.AmountTooSmall ->
            "${stringResource(R.string.errors_swap_amount_too_small)} ${minimumAmount(error.minAmount, pay?.asset)}"
        is SwapFailure.Unknown -> "${stringResource(R.string.errors_unknown_try_again)}: ${error.message}"
    }

    val infoSheetEntity = when (error) {
        SwapFailure.NoQuote -> InfoSheetEntity.NoQuoteInfo
        SwapFailure.UnsupportedAsset,
        is SwapFailure.AmountTooSmall,
        is SwapFailure.Unknown -> null
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

private fun minimumAmount(minAmount: BigInteger?, asset: Asset?): String {
    if (minAmount == null || asset == null) return ""
    return ValueFormatter(style = ValueFormatter.Style.Auto).string(minAmount, asset)
}
