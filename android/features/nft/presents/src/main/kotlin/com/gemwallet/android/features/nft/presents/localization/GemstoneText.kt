package com.gemwallet.android.features.nft.presents.localization

import androidx.annotation.StringRes
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.ReportReason
import uniffi.gemstone.GemNftList

@StringRes
internal fun GemNftList.stringRes(): Int = when (this) {
    GemNftList.COLLECTIONS,
    GemNftList.COLLECTION -> R.string.nft_collections
    GemNftList.UNVERIFIED -> R.string.asset_verification_unverified
}

@StringRes
internal fun ReportReason.stringRes(): Int = when (this) {
    ReportReason.Spam -> R.string.nft_report_reason_spam
    ReportReason.Malicious -> R.string.nft_report_reason_malicious
    ReportReason.Inappropriate -> R.string.nft_report_reason_inappropriate
    ReportReason.Copyright -> R.string.nft_report_reason_copyright
    ReportReason.Other -> R.string.nft_report_reason_other
}
