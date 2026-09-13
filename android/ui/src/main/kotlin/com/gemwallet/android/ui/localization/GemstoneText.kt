package com.gemwallet.android.ui.localization

import androidx.annotation.StringRes
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ext.toPrimitives
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.PerpetualDirection
import uniffi.gemstone.GemTransactionTitle
import uniffi.gemstone.GemWalletSubtitle

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
private fun perpetualTitle(direction: uniffi.gemstone.PerpetualDirection?, @StringRes directionTitle: Int, @StringRes fallback: Int): String {
    val side = when (direction?.toPrimitives()) {
        PerpetualDirection.Long -> stringResource(R.string.perpetual_long)
        PerpetualDirection.Short -> stringResource(R.string.perpetual_short)
        null -> return stringResource(fallback)
    }
    return stringResource(directionTitle, side)
}
