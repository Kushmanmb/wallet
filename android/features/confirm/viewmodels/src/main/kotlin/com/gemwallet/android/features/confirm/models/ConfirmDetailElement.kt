package com.gemwallet.android.features.confirm.models

import com.gemwallet.android.ui.models.perpetual.PerpetualConfirmDetailsUIModel
import com.gemwallet.android.ui.models.swap.SwapDetailsUIModel
import uniffi.gemstone.GemListRow

sealed interface ConfirmDetailElement {
    data class SwapDetails(
        val model: SwapDetailsUIModel,
    ) : ConfirmDetailElement

    data class PerpetualDetails(
        val model: PerpetualConfirmDetailsUIModel,
    ) : ConfirmDetailElement

    data class PerpetualModifyAutoclose(val row: GemListRow) : ConfirmDetailElement
}
