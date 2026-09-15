package com.gemwallet.android.ui.localization

import android.content.Context
import androidx.annotation.StringRes
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ext.asset
import com.gemwallet.android.ext.requireChain
import com.gemwallet.android.ext.toPrimitives
import com.gemwallet.android.model.ValueFormatter
import com.gemwallet.android.ui.R
import com.wallet.core.primitives.Asset
import com.wallet.core.primitives.ChartPeriod
import com.wallet.core.primitives.FeePriority
import com.wallet.core.primitives.PerpetualDirection
import com.wallet.core.primitives.QRScanType
import com.wallet.core.primitives.Resource
import com.wallet.core.primitives.TransactionState
import uniffi.gemstone.DelegationState
import uniffi.gemstone.GemAddNodeFailure
import uniffi.gemstone.GemApprovalValue
import uniffi.gemstone.GemAssetMenuAction
import uniffi.gemstone.GemDelegationStatus
import uniffi.gemstone.GemFiatTransactionBadge
import uniffi.gemstone.GemLocalizedText
import uniffi.gemstone.GemSimulationWarningKind
import uniffi.gemstone.GemSimulationWarningRow
import uniffi.gemstone.GemTransactionFilter
import uniffi.gemstone.LinkType
import uniffi.gemstone.GemTransactionStateTone
import uniffi.gemstone.GemVerificationLevel
import uniffi.gemstone.WalletConnectionVerificationStatus
import uniffi.gemstone.verificationLevel
import uniffi.gemstone.GemTransactionTitle
import uniffi.gemstone.GemWalletSubtitle
import uniffi.gemstone.SimulationPayloadFieldKind
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
fun GemFiatTransactionBadge.string(): String = stringResource(
    when (this) {
        GemFiatTransactionBadge.PENDING -> R.string.transaction_status_pending
        GemFiatTransactionBadge.FAILED -> R.string.transaction_status_failed
    }
)

@Composable
fun GemWalletSubtitle.string(): String = when (this) {
    GemWalletSubtitle.Multicoin -> stringResource(R.string.wallet_multicoin)
    is GemWalletSubtitle.Address -> value
}

@Composable
fun GemAddNodeFailure.string(): String = stringResource(
    when (this) {
        GemAddNodeFailure.INVALID_URL -> R.string.errors_invalid_url
        GemAddNodeFailure.INVALID_NETWORK_ID -> R.string.errors_invalid_network_id
        GemAddNodeFailure.UNAVAILABLE -> R.string.errors_error_occurred
    }
)

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
    GemSimulationWarningKind.UNLIMITED_APPROVAL,
    GemSimulationWarningKind.NFT_COLLECTION_APPROVAL,
    GemSimulationWarningKind.EXTERNALLY_OWNED_SPENDER,
    GemSimulationWarningKind.SUSPICIOUS_SPENDER -> message ?: descriptionRes()?.let { stringResource(it) }
}

@Composable
private fun perpetualTitle(direction: uniffi.gemstone.PerpetualDirection?, @StringRes directionTitle: Int, @StringRes fallback: Int): String {
    val side = when (val side = direction?.toPrimitives()) {
        null -> return stringResource(fallback)
        else -> stringResource(side.stringRes())
    }
    return stringResource(directionTitle, side)
}

fun GemLocalizedText.string(context: Context): String = when (this) {
    is GemLocalizedText.WalletDefaultName -> context.getString(R.string.wallet_default_name, index)
    is GemLocalizedText.WalletDefaultNameChain ->
        context.getString(R.string.wallet_default_name_chain, chain.requireChain().asset().name, index)
}

@StringRes
fun ChartPeriod.stringRes(): Int = when (this) {
    ChartPeriod.Hour -> R.string.charts_hour
    ChartPeriod.Day -> R.string.charts_day
    ChartPeriod.Week -> R.string.charts_week
    ChartPeriod.Month -> R.string.charts_month
    ChartPeriod.Year -> R.string.charts_year
    ChartPeriod.All -> R.string.charts_all
}

@StringRes
fun LinkType.stringRes(): Int = when (this) {
    LinkType.X -> R.string.social_x
    LinkType.DISCORD -> R.string.social_discord
    LinkType.REDDIT -> R.string.social_reddit
    LinkType.TELEGRAM -> R.string.social_telegram
    LinkType.GIT_HUB -> R.string.social_github
    LinkType.YOU_TUBE -> R.string.social_youtube
    LinkType.FACEBOOK -> R.string.social_facebook
    LinkType.WEBSITE -> R.string.social_website
    LinkType.COINGECKO -> R.string.social_coingecko
    LinkType.OPEN_SEA -> R.string.social_opensea
    LinkType.INSTAGRAM -> R.string.social_instagram
    LinkType.MAGIC_EDEN -> R.string.social_magiceden
    LinkType.COIN_MARKET_CAP -> R.string.social_coinmarketcap
    LinkType.TIK_TOK -> R.string.social_tiktok
}

@StringRes
fun Resource.stringRes(): Int = when (this) {
    Resource.Bandwidth -> R.string.stake_resource_bandwidth
    Resource.Energy -> R.string.stake_resource_energy
}

@StringRes
fun QRScanType.stringRes(): Int = when (this) {
    QRScanType.Universal -> R.string.wallet_scan_hint
    QRScanType.WalletConnect -> R.string.wallet_connect_title
    QRScanType.Address -> R.string.wallet_scan_hint_address
    QRScanType.Memo -> R.string.transfer_memo
    QRScanType.Url -> R.string.common_url
    QRScanType.TokenContract -> R.string.wallet_import_contract_address_field
    QRScanType.SecretPhrase -> R.string.common_secret_phrase
    QRScanType.PrivateKey -> R.string.common_private_key
}

@StringRes
fun SimulationPayloadFieldKind.stringRes(): Int? = when (this) {
    SimulationPayloadFieldKind.CONTRACT -> R.string.asset_contract
    SimulationPayloadFieldKind.METHOD -> R.string.common_method
    SimulationPayloadFieldKind.TOKEN -> R.string.common_token
    SimulationPayloadFieldKind.SPENDER -> R.string.transfer_to
    SimulationPayloadFieldKind.VALUE -> R.string.perpetual_value
    SimulationPayloadFieldKind.EXPIRATION -> R.string.common_expiration
    SimulationPayloadFieldKind.CUSTOM -> null
}

@StringRes
fun TransactionState.statusLabelRes(): Int = when (this) {
    TransactionState.Pending,
    TransactionState.InTransit -> R.string.transaction_status_pending
    TransactionState.Confirmed -> R.string.transaction_status_confirmed
    TransactionState.Failed -> R.string.transaction_status_failed
    TransactionState.Reverted -> R.string.transaction_status_reverted
    TransactionState.Refunded -> R.string.transaction_status_refunded
}

@StringRes
fun GemTransactionStateTone.infoDescriptionRes(): Int = when (this) {
    GemTransactionStateTone.PENDING -> R.string.info_transaction_pending_description
    GemTransactionStateTone.SUCCESS -> R.string.info_transaction_success_description
    GemTransactionStateTone.ERROR,
    GemTransactionStateTone.REFUNDED -> R.string.info_transaction_error_description
}

@StringRes
fun PerpetualDirection.stringRes(): Int = when (this) {
    PerpetualDirection.Long -> R.string.perpetual_long
    PerpetualDirection.Short -> R.string.perpetual_short
}

@StringRes
fun FeePriority.stringRes(): Int = when (this) {
    FeePriority.Normal -> R.string.fee_rates_normal
    FeePriority.Fast -> R.string.fee_rates_fast
}

@Composable
fun GemApprovalValue.string(symbol: String, formatter: ValueFormatter, asset: Asset): String = when (this) {
    is GemApprovalValue.Exact -> formatter.string(value, asset)
    GemApprovalValue.Unlimited -> stringResource(R.string.simulation_header_unlimited_asset, symbol)
}

@StringRes
fun GemVerificationLevel.stringRes(): Int = when (this) {
    GemVerificationLevel.VERIFIED -> R.string.asset_verification_verified
    GemVerificationLevel.UNVERIFIED -> R.string.asset_verification_unverified
    GemVerificationLevel.SUSPICIOUS -> R.string.asset_verification_suspicious
}

@StringRes
fun WalletConnectionVerificationStatus.titleRes(): Int = verificationLevel(this).stringRes()

@StringRes
fun GemAssetMenuAction.stringRes(): Int = when (this) {
    is GemAssetMenuAction.Pin -> if (isPinned) R.string.common_unpin else R.string.common_pin
    GemAssetMenuAction.Hide -> R.string.common_hide
    GemAssetMenuAction.AddToWallet -> R.string.asset_add_to_wallet
    is GemAssetMenuAction.CopyAddress -> R.string.wallet_copy_address
}
