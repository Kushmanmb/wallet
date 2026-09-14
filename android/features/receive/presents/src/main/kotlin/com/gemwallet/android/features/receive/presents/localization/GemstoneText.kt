package com.gemwallet.android.features.receive.presents.localization

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ui.R
import uniffi.gemstone.GemMemoWarning

@Composable
internal fun GemMemoWarning.string(): String? = when (this) {
    GemMemoWarning.DESTINATION_TAG -> stringResource(R.string.wallet_receive_no_destination_tag_required)
    GemMemoWarning.MEMO -> stringResource(R.string.wallet_receive_no_memo_required)
    GemMemoWarning.NOT_SUPPORTED -> null
}
