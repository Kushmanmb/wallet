package com.gemwallet.android.features.import_wallet.localization

import androidx.annotation.StringRes
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ui.R
import uniffi.gemstone.GemWalletImportException
import uniffi.gemstone.GemWalletImportKind

@StringRes
internal fun GemWalletImportKind.tabStringRes(): Int = when (this) {
    GemWalletImportKind.ADDRESS -> R.string.common_address
    GemWalletImportKind.PHRASE -> R.string.common_phrase
    GemWalletImportKind.PRIVATE_KEY -> R.string.common_private_key
}

@StringRes
internal fun GemWalletImportKind.fieldStringRes(): Int = when (this) {
    GemWalletImportKind.ADDRESS -> R.string.wallet_import_address_field
    GemWalletImportKind.PHRASE -> R.string.common_secret_phrase
    GemWalletImportKind.PRIVATE_KEY -> R.string.common_private_key
}

@Composable
internal fun GemWalletImportException.string(): String = when (this) {
    is GemWalletImportException.InvalidSecretPhraseWords -> stringResource(R.string.errors_import_invalid_secret_phrase_word, words.joinToString())
    is GemWalletImportException.InvalidSecretPhrase -> stringResource(R.string.errors_import_invalid_secret_phrase)
    is GemWalletImportException.InvalidAddress -> stringResource(R.string.errors_invalid_address_name)
    is GemWalletImportException.InvalidPrivateKey -> stringResource(R.string.errors_import_invalid_private_key)
    is GemWalletImportException.MissingChain -> stringResource(R.string.errors_unknown)
}
