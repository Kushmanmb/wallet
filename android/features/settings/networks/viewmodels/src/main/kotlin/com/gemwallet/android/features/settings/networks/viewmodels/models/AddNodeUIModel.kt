package com.gemwallet.android.features.settings.networks.viewmodels.models

import com.gemwallet.android.ui.models.ButtonState
import com.gemwallet.android.ui.models.buttonState
import com.wallet.core.primitives.Chain
import uniffi.gemstone.GemAddNodeFailure
import uniffi.gemstone.GemAddNodePhase
import uniffi.gemstone.GemAddNodeViewState
import uniffi.gemstone.GemNodeCheck

data class AddNodeUIModel(
    val chain: Chain? = null,
    val state: GemAddNodeViewState? = null,
) {
    val status: GemNodeCheck? = (state?.phase as? GemAddNodePhase.Ready)?.check

    val failure: GemAddNodeFailure? = (state?.phase as? GemAddNodePhase.Failed)?.failure

    private val checking: Boolean = state?.phase == GemAddNodePhase.Checking

    val canImport: Boolean = state?.canImport == true

    val buttonState: ButtonState
        get() = buttonState(enabled = canImport, loading = checking)
}
