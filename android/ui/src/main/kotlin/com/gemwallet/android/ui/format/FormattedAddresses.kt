package com.gemwallet.android.ui.format

import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import com.gemwallet.android.ext.toGem
import com.gemwallet.android.ui.LocalAddressService
import com.wallet.core.primitives.ChainAddress
import uniffi.gemstone.GemAddressFormatStyle

@Composable
fun rememberFormattedAddresses(
    addresses: List<ChainAddress>,
    style: GemAddressFormatStyle = GemAddressFormatStyle.Short,
): Map<String, String> {
    val addressService = LocalAddressService.current
    return remember(addressService, addresses, style) {
        addresses.map { it.address }.zip(addressService.formatAll(addresses.map { it.toGem() }, style)).toMap()
    }
}
