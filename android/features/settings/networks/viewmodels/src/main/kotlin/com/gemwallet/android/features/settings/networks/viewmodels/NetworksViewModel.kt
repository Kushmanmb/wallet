package com.gemwallet.android.features.settings.networks.viewmodels

import com.gemwallet.android.ext.requireChain
import androidx.compose.foundation.text.input.TextFieldState
import androidx.compose.runtime.snapshotFlow
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import uniffi.gemstone.GemErrorText
import uniffi.gemstone.GemChainSettingsServiceInterface
import uniffi.gemstone.GemChainSettingsSection
import uniffi.gemstone.GemExplorerRow
import uniffi.gemstone.GemNodeRow
import uniffi.gemstone.GemNodeSelection
import uniffi.gemstone.GemNodeStatusState
import com.gemwallet.android.features.settings.networks.viewmodels.models.NetworksUIState
import com.wallet.core.primitives.Chain
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.Job
import kotlinx.coroutines.launch
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.supervisorScope
import kotlinx.coroutines.withContext
import javax.inject.Inject
import com.gemwallet.android.ext.runCatchingCancellable
import com.gemwallet.android.ext.errorText

@OptIn(ExperimentalCoroutinesApi::class)
@HiltViewModel
class NetworksViewModel @Inject constructor(
    private val service: GemChainSettingsServiceInterface,
) : ViewModel() {

    private val sections = service.sections()
    private val state = MutableStateFlow(State())
    val uiState = state
        .map { it.toUIState() }
        .stateIn(viewModelScope, SharingStarted.Eagerly, state.value.toUIState())
    val chainFilter = TextFieldState()

    private var observeNodesJob: Job? = null
    private var refreshJob: Job? = null

    init {
        viewModelScope.launch {
            updateState { it.copy(availableChains = service.chains("").map { it.requireChain() }) }
            snapshotFlow { chainFilter.text }.collectLatest { query ->
                updateState { it.copy(availableChains = service.chains(query.toString()).map { it.requireChain() }) }
            }
        }
    }

    fun onSelectedChain(chain: Chain) {
        updateState {
            it.copy(
                chain = chain,
                selectChain = false,
                explorers = service.explorerRows(chain.string),
                availableAddNode = true,
                nodes = emptyList(),
                nodeStates = emptyMap(),
                refreshNonce = System.nanoTime(),
            )
        }
        observeNodes(chain)
    }

    fun refresh() {
        val chain = state.value.chain ?: return
        refreshNodeStatuses(chain, System.nanoTime())
    }

    fun onSelectNode(url: String) {
        val chain = state.value.chain ?: return
        viewModelScope.launch {
            runCatchingCancellable { service.selectNode(chain.string, url) }
                .onSuccess { loadNodes(chain) }
                .onFailure { error -> updateState { it.copy(error = error.errorText()) } }
        }
    }

    fun onSelectBlockExplorer(name: String) {
        val chain = state.value.chain ?: return
        runCatching { service.setExplorerName(chain.string, name) }
            .onSuccess { updateState { it.copy(explorers = service.explorerRows(chain.string)) } }
            .onFailure { error -> updateState { it.copy(error = error.errorText()) } }
    }

    fun onSelectChain() {
        updateState { it.copy(selectChain = true) }
    }

    fun onDeleteNode(url: String) {
        val chain = state.value.chain ?: return
        viewModelScope.launch {
            runCatchingCancellable { service.deleteNode(chain.string, url) }
                .onSuccess { loadNodes(chain) }
                .onFailure { error -> updateState { it.copy(error = error.errorText()) } }
        }
    }

    fun clearError() = updateState { it.copy(error = null) }

    private fun observeNodes(chain: Chain) {
        observeNodesJob?.cancel()
        observeNodesJob = viewModelScope.launch {
            loadNodes(chain)
            refreshNodeStatuses(chain, System.nanoTime())
        }
    }

    private suspend fun loadNodes(chain: Chain) {
        val nodes = service.nodes(chain.string)

        updateState {
            it.copy(
                nodes = nodes,
                nodeStates = visibleNodeStates(nodes, it.nodeStates),
            )
        }
    }

    private fun refreshNodeStatuses(chain: Chain, refreshNonce: Long) {
        refreshJob?.cancel()
        refreshJob = viewModelScope.launch {
            val nodes = state.value.nodes
            if (nodes.isEmpty()) {
                updateState { current ->
                    if (current.chain == chain && current.refreshNonce <= refreshNonce) {
                        current.copy(
                            refreshNonce = refreshNonce,
                        )
                    } else {
                        current
                    }
                }
                return@launch
            }

            val loadingStates = nodes.associate { it.url to GemNodeStatusState.Loading }
            updateState { current ->
                if (current.chain != chain) {
                    current
                } else {
                    current.copy(
                        refreshNonce = refreshNonce,
                        nodeStates = loadingStates,
                    )
                }
            }

            supervisorScope {
                nodes.forEach { node ->
                    launch {
                        val nodeState = withContext(Dispatchers.IO) {
                            service.nodeStatus(chain.string, node.url)
                        }
                        updateNodesIfCurrent(chain, refreshNonce) { current ->
                            if (current.nodes.none { it.url == node.url }) {
                                current
                            } else {
                                current.copy(
                                    nodeStates = visibleNodeStates(
                                        current.nodes,
                                        current.nodeStates + (node.url to nodeState),
                                    ),
                                )
                            }
                        }
                    }
                }
            }
        }
    }

    private fun updateNodesIfCurrent(chain: Chain, refreshNonce: Long, transform: (State) -> State) {
        updateState { current ->
            if (current.chain != chain || current.refreshNonce != refreshNonce) current
            else transform(current)
        }
    }

    private fun updateState(transform: (State) -> State) {
        state.update(transform)
    }

    private data class State(
        val chain: Chain? = null,
        val explorers: List<GemExplorerRow> = emptyList(),
        val nodeStates: Map<String, GemNodeStatusState> = emptyMap(),
        val nodes: List<GemNodeSelection> = emptyList(),
        val availableChains: List<Chain> = emptyList(),
        val selectChain: Boolean = true,
        val availableAddNode: Boolean = true,
        val refreshNonce: Long = 0,
        val error: GemErrorText? = null,
    )

    private fun State.toUIState(): NetworksUIState = NetworksUIState(
        chain = chain,
        chains = availableChains,
        selectChain = selectChain,
        sections = sections,
        blockExplorers = explorers,
        availableAddNode = availableAddNode,
        nodeRows = chain?.let { service.nodeRows(it.string, nodes, nodeStates) }.orEmpty(),
        error = error,
    )
}

internal fun visibleNodeStates(
    nodes: List<GemNodeSelection>,
    nodeStates: Map<String, GemNodeStatusState>,
): Map<String, GemNodeStatusState> {
    val nodeUrls = nodes.mapTo(hashSetOf()) { it.url }
    return nodeStates.filterKeys(nodeUrls::contains)
}
