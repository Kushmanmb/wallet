package com.gemwallet.android.ui.localization

import androidx.annotation.StringRes
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ext.toPrimitives
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.PerpetualDirection
import uniffi.gemstone.DelegationState
import uniffi.gemstone.GemDelegationStatus
import uniffi.gemstone.GemSimulationWarningKind
import uniffi.gemstone.GemSimulationWarningRow
import uniffi.gemstone.GemTransactionFilter
import uniffi.gemstone.GemTransactionTitle
import uniffi.gemstone.GemWalletSubtitle
import uniffi.gemstone.SimulationSeverity

@Composable
fun GemTransactionTitle.string(): String = when (this) {
    GemTransactionTitle.Received -> stringResource(R.string.transaction_title_received)
    GemTransactionTitle.Sent -> stringResource(R.string.transaction_title_sent)
    GemTransactionTitle.Transfer -> stringResource(R.string.transfer_title)
    GemTransactionTitle.SmartContract -> stringResource(R.string.transfer_smart_contract_title)
    GemTransactionTitle.Swap -> stringResource(R.string.wallet_swap)
    GemTransactionTitle.Approve -> stringResource(R.string.transfer_approve_title)
    GemTransactionTitle.Stake -> stringResource(R.string.transfer_stake_title)
    GemTransactionTitle.Unstake -> stringResource(R.string.transfer_unstake_title)
    GemTransactionTitle.Redelegate -> stringResource(R.string.transfer_redelegate_title)
    GemTransactionTitle.Rewards -> stringResource(R.string.transfer_rewards_title)
    GemTransactionTitle.Withdraw -> stringResource(R.string.transfer_withdraw_title)
    GemTransactionTitle.ActivateAsset -> stringResource(R.string.transfer_activate_asset_title)
    GemTransactionTitle.Freeze -> stringResource(R.string.transfer_freeze_title)
    GemTransactionTitle.Unfreeze -> stringResource(R.string.transfer_unfreeze_title)
    GemTransactionTitle.Earn -> stringResource(R.string.common_earn)
    is GemTransactionTitle.PerpetualOpen -> perpetualTitle(direction, R.string.perpetual_open_direction, R.string.perpetual_position)
    is GemTransactionTitle.PerpetualClose -> perpetualTitle(direction, R.string.perpetual_close_direction, R.string.perpetual_close_position)
    GemTransactionTitle.PerpetualModify -> stringResource(R.string.perpetual_modify)
}

@Composable
fun GemWalletSubtitle.string(): String = when (this) {
    GemWalletSubtitle.Multicoin -> stringResource(R.string.wallet_multicoin)
    is GemWalletSubtitle.Address -> value
}

@Composable
fun GemDelegationStatus.stateText(): String = stringResource(
    when (state) {
        DelegationState.ACTIVE -> R.string.stake_active
        DelegationState.PENDING -> R.string.stake_pending
        DelegationState.INACTIVE -> R.string.stake_inactive
        DelegationState.ACTIVATING -> R.string.stake_activating
        DelegationState.DEACTIVATING -> R.string.stake_deactivating
        DelegationState.AWAITING_WITHDRAWAL -> R.string.stake_awaiting_withdrawal
    }
)

@StringRes
fun GemTransactionFilter.getLabel() = when (this) {
    GemTransactionFilter.TRANSFERS -> R.string.transfer_title
    GemTransactionFilter.SWAPS -> R.string.wallet_swap
    GemTransactionFilter.STAKE -> R.string.wallet_stake
    GemTransactionFilter.SMART_CONTRACT -> R.string.transfer_smart_contract_title
    GemTransactionFilter.PERPETUALS -> R.string.perpetuals_title
    GemTransactionFilter.OTHERS -> R.string.transfer_other_title
}

@StringRes
fun GemSimulationWarningRow.titleRes(): Int = when (kind) {
    GemSimulationWarningKind.VALIDATION_ERROR -> if (severity != SimulationSeverity.CRITICAL) R.string.common_warning else R.string.errors_error_occurred
    GemSimulationWarningKind.NFT_COLLECTION_APPROVAL -> R.string.simulation_warning_nft_collection_approval_title
    GemSimulationWarningKind.UNLIMITED_APPROVAL -> R.string.simulation_warning_unlimited_token_approval_title
    GemSimulationWarningKind.EXTERNALLY_OWNED_SPENDER -> R.string.common_warning
    GemSimulationWarningKind.SUSPICIOUS_SPENDER -> R.string.errors_error_occurred
}

@StringRes
fun GemSimulationWarningRow.descriptionRes(): Int? = when (kind) {
    GemSimulationWarningKind.UNLIMITED_APPROVAL -> R.string.simulation_warning_unlimited_token_approval_description
    GemSimulationWarningKind.EXTERNALLY_OWNED_SPENDER -> R.string.simulation_warning_externally_owned_spender_description
    GemSimulationWarningKind.SUSPICIOUS_SPENDER -> R.string.common_suspicious_address
    GemSimulationWarningKind.VALIDATION_ERROR -> if (severity == SimulationSeverity.CRITICAL) R.string.errors_error_occurred else null
    GemSimulationWarningKind.NFT_COLLECTION_APPROVAL -> null
}

@Composable
fun GemSimulationWarningRow.descriptionText(): String? = when (kind) {
    GemSimulationWarningKind.VALIDATION_ERROR -> if (severity != SimulationSeverity.CRITICAL) message.orEmpty() else message ?: stringResource(R.string.errors_error_occurred)
    else -> message ?: descriptionRes()?.let { stringResource(it) }
}

@Composable
private fun perpetualTitle(direction: uniffi.gemstone.PerpetualDirection?, @StringRes directionTitle: Int, @StringRes fallback: Int): String {
    val side = when (direction?.toPrimitives()) {
        PerpetualDirection.Long -> stringResource(R.string.perpetual_long)
        PerpetualDirection.Short -> stringResource(R.string.perpetual_short)
        null -> return stringResource(fallback)
    }
    return stringResource(directionTitle, side)
}
