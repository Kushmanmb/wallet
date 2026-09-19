package com.gemwallet.android.features.asset.presents.chart

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListScope
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.pulltorefresh.PullToRefreshDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.hilt.lifecycle.viewmodel.compose.hiltViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.gemwallet.android.features.asset.viewmodels.chart.models.ChartSectionUIModel
import com.gemwallet.android.features.asset.viewmodels.chart.viewmodels.AssetChartViewModel
import com.gemwallet.android.features.asset.viewmodels.chart.viewmodels.ChartViewModel
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.list_item.GemListRowView
import com.gemwallet.android.ui.components.list_item.ListItem
import com.gemwallet.android.ui.components.list_item.ListItemModel
import com.gemwallet.android.ui.components.list_item.SubheaderItem
import com.gemwallet.android.ui.components.list_item.property.DataBadgeChevron
import com.gemwallet.android.ui.components.list_item.property.itemsPositioned
import com.gemwallet.android.ui.components.screen.PullToRefreshBox
import com.gemwallet.android.ui.components.screen.Scene
import com.gemwallet.android.ui.components.screen.rememberSnackbarState
import com.gemwallet.android.ui.models.ListPosition
import com.wallet.core.primitives.AssetId

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AssetChartScene(
    onCancel: () -> Unit,
    onPriceAlerts: (AssetId) -> Unit,
    onAddPriceAlertTarget: (AssetId) -> Unit,
    toastMessage: String? = null,
    onToastShown: () -> Unit = {},
    viewModel: AssetChartViewModel = hiltViewModel(),
    chartViewModel: ChartViewModel = hiltViewModel(),
) {
    val marketModel by viewModel.marketUIModel.collectAsStateWithLifecycle()
    val title by viewModel.title.collectAsStateWithLifecycle()
    val isChartRefreshing by chartViewModel.isRefreshing.collectAsStateWithLifecycle()
    val snackbar = rememberSnackbarState(
        message = toastMessage,
        iconRes = R.drawable.ic_notifications,
        onShown = onToastShown,
    )

    Scene(
        title = title,
        onClose = onCancel,
        snackbar = snackbar,
    ) {
        PullToRefreshBox(
            isRefreshing = isChartRefreshing,
            onRefresh = {
                chartViewModel.refresh()
            },
            containerColor = PullToRefreshDefaults.indicatorContainerColor,
        ) {
            LazyColumn(modifier = Modifier.fillMaxSize()) {
                item { Chart(chartViewModel) }
                marketModel?.let { model ->
                    model.sections.forEach { section ->
                        when (section) {
                            is ChartSectionUIModel.PriceAlerts -> item { LinkRow(section.model) { onPriceAlerts(viewModel.assetId) } }
                            is ChartSectionUIModel.SetPriceAlert -> item { LinkRow(section.model) { onAddPriceAlertTarget(viewModel.assetId) } }
                            is ChartSectionUIModel.Market -> itemsPositioned(section.rows) { position, row -> GemListRowView(row = row, listPosition = position) }
                            is ChartSectionUIModel.Links -> links(section)
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun LinkRow(model: ListItemModel, onClick: () -> Unit) {
    ListItem(
        model = model,
        listPosition = ListPosition.Single,
        modifier = Modifier
            .clickable(onClick = onClick)
            .testTag("assetChart"),
        accessory = { DataBadgeChevron() },
    )
}

private fun LazyListScope.links(section: ChartSectionUIModel.Links) {
    item { SubheaderItem(section.title) }
    item { GemListRowView(row = section.row, listPosition = ListPosition.Single) }
}
