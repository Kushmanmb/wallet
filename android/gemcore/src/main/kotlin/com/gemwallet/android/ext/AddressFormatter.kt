package com.gemwallet.android.ext

import com.wallet.core.primitives.Chain
import uniffi.gemstone.GemAddressFormatStyle
import uniffi.gemstone.GemAddressService

class AddressFormatter(
    private val addressService: GemAddressService,
    private val address: String,
    private val chain: Chain? = null,
    private val style: GemAddressFormatStyle = GemAddressFormatStyle.Short,
) {
    fun value(): String = addressService.format(address, chain?.string, style)
}
