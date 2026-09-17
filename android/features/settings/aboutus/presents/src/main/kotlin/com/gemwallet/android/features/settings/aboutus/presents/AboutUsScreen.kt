package com.gemwallet.android.features.settings.aboutus.presents

import android.content.pm.PackageManager
import android.os.Build
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalUriHandler
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import com.gemwallet.android.features.settings.aboutus.presents.models.AboutRowUIModel
import com.gemwallet.android.features.settings.aboutus.presents.models.aboutRows
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.list_item.LinkItem
import com.gemwallet.android.ui.components.list_item.SubheaderItem
import com.gemwallet.android.ui.components.screen.Scene
import com.gemwallet.android.ui.models.ListPosition
import com.gemwallet.android.ui.open

@Composable
fun AboutUsScreen(
    onCancel: () -> Unit
) {
    val context = LocalContext.current
    val uriHandler = LocalUriHandler.current
    val version = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
        context.packageManager.getPackageInfo(context.packageName, PackageManager.PackageInfoFlags.of(0))
    } else {
        context.packageManager.getPackageInfo(context.packageName, 0)
    }.versionName
    val rows = remember(version) { aboutRows(version ?: "") }
    Scene(title = stringResource(id = R.string.settings_aboutus), onClose = onCancel) {
        LazyColumn {
            rows.forEach { section ->
                itemsIndexed(section) { index, row ->
                    val listPosition = ListPosition.getPosition(index, section.size)
                    when (row) {
                        is AboutRowUIModel.Link -> LinkItem(
                            title = stringResource(row.title),
                            listPosition = listPosition,
                        ) {
                            uriHandler.open(context, row.url)
                        }
                        is AboutRowUIModel.Community -> Column {
                            SubheaderItem(row.title)
                            row.links.forEachIndexed { linkIndex, link ->
                                LinkItem(
                                    title = stringResource(id = link.label),
                                    icon = link.icon,
                                    listPosition = ListPosition.getPosition(linkIndex, row.links.size),
                                ) {
                                    uriHandler.open(context, link.url)
                                }
                            }
                        }
                        is AboutRowUIModel.Version -> LinkItem(
                            title = stringResource(row.title),
                            listPosition = listPosition,
                            trailingContent = {
                                Text(
                                    text = row.version,
                                    textAlign = TextAlign.Center,
                                    style = MaterialTheme.typography.bodyMedium,
                                )
                            },
                            onClick = {},
                        )
                    }
                }
            }
        }
    }
}
