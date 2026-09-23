package com.gemwallet.android.testkit

import com.gemwallet.android.model.AssetBalance
import com.gemwallet.android.model.Balance
import com.gemwallet.android.model.Crypto
import com.wallet.core.primitives.Asset
import com.wallet.core.primitives.BalanceMetadata
import java.math.BigInteger

fun mockAssetBalance(
    asset: Asset = mockAsset(),
    available: BigInteger = BigInteger.ZERO,
    frozen: BigInteger = BigInteger.ZERO,
    locked: BigInteger = BigInteger.ZERO,
    staked: BigInteger = BigInteger.ZERO,
    pending: BigInteger = BigInteger.ZERO,
    rewards: BigInteger = BigInteger.ZERO,
    reserved: BigInteger = BigInteger.ZERO,
    withdrawable: BigInteger = BigInteger.ZERO,
    pendingUnconfirmed: BigInteger = BigInteger.ZERO,
    earn: BigInteger = BigInteger.ZERO,
    metadata: BalanceMetadata? = null,
    isActive: Boolean = true,
): AssetBalance {
    val balance = Balance(
        available = available,
        frozen = frozen,
        locked = locked,
        staked = staked,
        pending = pending,
        rewards = rewards,
        reserved = reserved,
        withdrawable = withdrawable,
        pendingUnconfirmed = pendingUnconfirmed,
        earn = earn,
    )
    val balanceAmount = balance.createAmount(asset.decimals)
    return AssetBalance(
        asset = asset,
        balance = balance,
        balanceAmount = balanceAmount,
        totalAmount = balanceAmount.available + balanceAmount.frozen + balanceAmount.locked + balanceAmount.staked + balanceAmount.pending + balanceAmount.rewards + balanceAmount.earn,
        metadata = metadata,
        isActive = isActive,
    )
}

private fun Balance<BigInteger>.createAmount(decimals: Int) = Balance(
    available = Crypto(available).value(decimals).stripTrailingZeros().toDouble(),
    frozen = Crypto(frozen).value(decimals).stripTrailingZeros().toDouble(),
    locked = Crypto(locked).value(decimals).stripTrailingZeros().toDouble(),
    staked = Crypto(staked).value(decimals).stripTrailingZeros().toDouble(),
    pending = Crypto(pending).value(decimals).stripTrailingZeros().toDouble(),
    rewards = Crypto(rewards).value(decimals).stripTrailingZeros().toDouble(),
    reserved = Crypto(reserved).value(decimals).stripTrailingZeros().toDouble(),
    withdrawable = Crypto(withdrawable).value(decimals).stripTrailingZeros().toDouble(),
    pendingUnconfirmed = Crypto(pendingUnconfirmed).value(decimals).stripTrailingZeros().toDouble(),
    earn = Crypto(earn).value(decimals).stripTrailingZeros().toDouble(),
)
