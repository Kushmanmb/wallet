package com.gemwallet.android.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.ConnectionStatus

@StringRes
internal fun ConnectionStatus.stringRes(): Int? = when (this) {
    ConnectionStatus.Online -> null
    ConnectionStatus.NoInternet -> R.string.errors_no_internet_connection
    ConnectionStatus.NoService -> R.string.errors_no_service_connection
}
