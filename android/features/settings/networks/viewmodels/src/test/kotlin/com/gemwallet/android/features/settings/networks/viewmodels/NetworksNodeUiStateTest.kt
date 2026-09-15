package com.gemwallet.android.features.settings.networks.viewmodels

import org.junit.Assert.assertEquals
import org.junit.Test
import uniffi.gemstone.GemNodeSelection
import uniffi.gemstone.GemNodeStatusState
import uniffi.gemstone.Latency
import uniffi.gemstone.LatencyType

class NetworksNodeUiStateTest {

    @Test
    fun `visibleNodeStates removes deleted node entries`() {
        val gemNode = selection("https://gemnodes.com/bitcoin")
        val remainingNode = selection("https://custom.example.com/bitcoin")
        val deletedNode = selection("https://deleted.example.com/bitcoin")
        val nodeStates = mapOf(
            gemNode.url to GemNodeStatusState.Loading,
            remainingNode.url to GemNodeStatusState.Error,
            deletedNode.url to GemNodeStatusState.Result(
                latestBlockNumber = 1UL,
                latency = Latency(LatencyType.FAST, 20.0),
            ),
        )

        val visibleStates = visibleNodeStates(
            nodes = listOf(gemNode, remainingNode),
            nodeStates = nodeStates,
        )

        assertEquals(setOf(gemNode.url, remainingNode.url), visibleStates.keys)
    }

    private fun selection(url: String, isSelected: Boolean = false) = GemNodeSelection(
        url = url,
        host = url.removePrefix("https://").substringBefore("/"),
        isSelected = isSelected,
        gemNodeFlag = null,
    )
}
