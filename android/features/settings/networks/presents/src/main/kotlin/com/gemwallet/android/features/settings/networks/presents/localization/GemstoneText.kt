package com.gemwallet.android.features.settings.networks.presents.localization

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ui.R
import uniffi.gemstone.GemNodeRowTitle
import uniffi.gemstone.GemServiceEndpointType

@Composable
internal fun GemNodeRowTitle.string(): String = when (this) {
    is GemNodeRowTitle.Host -> host
    is GemNodeRowTitle.GemNode -> "${stringResource(R.string.nodes_gem_wallet_node)} $flag"
}

@Composable
internal fun GemServiceEndpointType.string(): String = when (this) {
    GemServiceEndpointType.API -> "API"
    GemServiceEndpointType.GEM_NODE -> stringResource(R.string.nodes_gem_wallet_node)
}
