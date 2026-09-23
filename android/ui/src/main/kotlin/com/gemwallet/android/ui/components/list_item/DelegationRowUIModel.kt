package com.gemwallet.android.ui.components.list_item

import com.gemwallet.android.ext.toGem
import com.gemwallet.android.model.AssetInfo
import com.wallet.core.primitives.Currency
import com.wallet.core.primitives.Delegation
import uniffi.gemstone.GemDelegationListRow
import uniffi.gemstone.delegationListRows

class DelegationRowUIModel(val delegation: Delegation, val row: GemDelegationListRow)

fun List<Delegation>.delegationRows(assetInfo: AssetInfo): List<DelegationRowUIModel> = zip(
    delegationListRows(
        map { it.toGem() },
        assetInfo.asset.toGem(),
        assetInfo.price?.price?.price,
        (assetInfo.price?.currency ?: Currency.USD).toGem(),
    ),
    ::DelegationRowUIModel,
)
