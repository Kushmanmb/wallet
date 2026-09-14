package com.gemwallet.android.features.confirm.presents.localization

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.domains.asset.title
import com.gemwallet.android.ext.boldMarkdown
import com.gemwallet.android.ext.networkName
import com.gemwallet.android.ext.toGemNetworkError
import com.gemwallet.android.ext.toPrimitives
import com.gemwallet.android.model.GemNetworkError
import com.gemwallet.android.model.ValueFormatter
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.localizedDescription
import com.gemwallet.android.ui.components.perpetual.title
import uniffi.gemstone.GemConfirmButtonKind
import uniffi.gemstone.GemConfirmException
import uniffi.gemstone.GemConfirmScreen
import uniffi.gemstone.GemConfirmTitle
import uniffi.gemstone.GemSignerError

@Composable
internal fun GemConfirmTitle.string(): String = when (this) {
    GemConfirmTitle.Send -> stringResource(R.string.transfer_send_title)
    GemConfirmTitle.Deposit -> stringResource(R.string.wallet_deposit)
    GemConfirmTitle.Withdraw -> stringResource(R.string.transfer_withdraw_title)
    GemConfirmTitle.Swap -> stringResource(R.string.wallet_swap)
    GemConfirmTitle.Approve -> stringResource(R.string.transfer_approve_title)
    GemConfirmTitle.Request -> stringResource(R.string.transfer_review_request)
    GemConfirmTitle.Stake -> stringResource(R.string.transfer_stake_title)
    GemConfirmTitle.Unstake -> stringResource(R.string.transfer_unstake_title)
    GemConfirmTitle.Redelegate -> stringResource(R.string.transfer_redelegate_title)
    GemConfirmTitle.ClaimRewards -> stringResource(R.string.transfer_claim_rewards_title)
    GemConfirmTitle.Freeze -> stringResource(R.string.transfer_freeze_title)
    GemConfirmTitle.Unfreeze -> stringResource(R.string.transfer_unfreeze_title)
    GemConfirmTitle.ActivateAsset -> stringResource(R.string.transfer_activate_asset_title)
    is GemConfirmTitle.PerpetualOpen -> direction.toPrimitives().title()
    is GemConfirmTitle.PerpetualIncrease -> stringResource(R.string.perpetual_increase_direction, direction.toPrimitives().title())
    is GemConfirmTitle.PerpetualReduce -> stringResource(R.string.perpetual_reduce_direction, direction.toPrimitives().title())
    GemConfirmTitle.PerpetualClose -> stringResource(R.string.perpetual_close_position)
    GemConfirmTitle.PerpetualModify -> stringResource(R.string.perpetual_modify_position)
}

@Composable
internal fun GemConfirmScreen.buttonLabel(kind: GemConfirmButtonKind): String = when {
    failure?.error is GemConfirmException.AccountMissing -> stringResource(R.string.errors_wallet_account_missing)
    kind == GemConfirmButtonKind.RETRY -> stringResource(R.string.common_try_again)
    else -> stringResource(R.string.transfer_confirm)
}

@Composable
internal fun Throwable.toPreloadLabel(): String = toConfirmLabel()
    ?: toGemNetworkError()?.localizedDescription()
    ?: "${stringResource(R.string.confirm_fee_error)}: ${stringResource(R.string.errors_unable_estimate_network_fee)}"

@Composable
internal fun Throwable.toBroadcastLabel(): String = toConfirmLabel()
    ?: toGemNetworkError()?.localizedDescription()
    ?: "${stringResource(R.string.errors_transfer_error)}: ${message ?: toString()}"

@Composable
private fun Throwable.toConfirmLabel(): String? = (this as? GemConfirmException)?.string()

@Composable
private fun GemConfirmException.string(): String = when (this) {
    is GemConfirmException.ScanMalicious -> stringResource(R.string.errors_scan_transaction_malicious_description)
    is GemConfirmException.ScanMemoRequired -> stringResource(R.string.errors_scan_transaction_memo_required, symbol)
    is GemConfirmException.FeeRatesMissing -> stringResource(R.string.errors_unable_estimate_network_fee)
    is GemConfirmException.Offline -> GemNetworkError.Offline.localizedDescription()
    is GemConfirmException.Cancelled -> stringResource(R.string.errors_cancelled)
    is GemConfirmException.AccountMissing -> stringResource(R.string.errors_wallet_account_missing)
    is GemConfirmException.SenderMismatch -> stringResource(R.string.errors_unknown)
    is GemConfirmException.BalanceMissing -> toString()
    is GemConfirmException.InsufficientBalance -> {
        val formatter = ValueFormatter(style = ValueFormatter.Style.Full)
        val asset = asset.toPrimitives()
        stringResource(
            R.string.info_balance_required_description,
            formatter.string(requirement.required, asset).boldMarkdown(),
            formatter.string(requirement.available, asset),
            formatter.string(requirement.shortfall, asset),
        )
    }
    is GemConfirmException.InsufficientNetworkFee -> {
        val formatter = ValueFormatter(style = ValueFormatter.Style.Full)
        val asset = asset.toPrimitives()
        requirement?.let {
            stringResource(
                R.string.info_insufficient_network_fee_balance_description,
                formatter.string(it.required, asset).boldMarkdown(),
                asset.id.chain.networkName().boldMarkdown(),
                formatter.string(it.available, asset),
                formatter.string(it.shortfall, asset),
            )
        } ?: stringResource(R.string.transfer_insufficient_network_fee_balance, asset.title.boldMarkdown())
    }
    is GemConfirmException.MinimumAccountBalanceTooLow -> stringResource(
        R.string.transfer_minimum_account_balance,
        ValueFormatter(style = ValueFormatter.Style.Full).string(requirement.required, asset.toPrimitives()).boldMarkdown(),
    )
    is GemConfirmException.BelowSwapMinimum -> {
        val formatter = ValueFormatter(style = ValueFormatter.Style.Full)
        val asset = asset.toPrimitives()
        stringResource(
            R.string.info_swap_minimum_amount_description,
            providerName.boldMarkdown(),
            formatter.string(requirement.required, asset).boldMarkdown(),
            formatter.string(requirement.available, asset),
            formatter.string(requirement.shortfall, asset),
        )
    }
    is GemConfirmException.Sign -> when (error) {
        GemSignerError.DustThreshold -> stringResource(R.string.errors_dust_threshold_short)
        GemSignerError.InsufficientFunds -> stringResource(R.string.info_insufficient_balance_title)
        else -> msg
    }
    is GemConfirmException.Network -> msg
    is GemConfirmException.Load -> msg
    is GemConfirmException.Broadcast -> msg
    is GemConfirmException.Record -> msg
    is GemConfirmException.ApprovalInvalid -> msg
}
