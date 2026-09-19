package com.gemwallet.android.features.confirm.models

import com.gemwallet.android.ui.models.swap.SwapDetailsUIModel
import uniffi.gemstone.GemListRow
import uniffi.gemstone.GemPerpetualDetails

sealed interface ConfirmDetailElement {
    data class SwapDetails(
        val model: SwapDetailsUIModel,
    ) : ConfirmDetailElement

    data class PerpetualDetails(
        val details: GemPerpetualDetails,
    ) : ConfirmDetailElement

    data class PerpetualModifyAutoclose(val row: GemListRow) : ConfirmDetailElement
}
