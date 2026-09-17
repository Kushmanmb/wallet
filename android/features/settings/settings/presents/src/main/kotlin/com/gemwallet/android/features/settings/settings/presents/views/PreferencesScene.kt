package com.gemwallet.android.features.settings.settings.presents.views

import android.content.Intent
import android.net.Uri
import android.os.Build
import android.provider.Settings
import androidx.annotation.DrawableRes
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.hilt.lifecycle.viewmodel.compose.hiltViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.gemwallet.android.features.settings.settings.presents.localization.stringRes
import com.gemwallet.android.features.settings.settings.viewmodels.PreferencesViewModel
import com.gemwallet.android.features.settings.settings.viewmodels.models.PreferencesRowUIModel
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.components.list_item.LinkItem
import com.gemwallet.android.ui.components.list_item.property.DataBadgeChevron
import com.gemwallet.android.ui.components.list_item.property.PropertyDataText
import com.gemwallet.android.ui.components.screen.Scene
import com.gemwallet.android.ui.icons.AppIcons
import com.gemwallet.android.ui.models.ListPosition
import com.gemwallet.android.ui.models.actions.PreferencesAction
import com.gemwallet.android.ui.theme.Spacer4
import com.gemwallet.android.ui.theme.compactIconSize
import com.wallet.core.primitives.Appearance
import java.util.Locale

@Composable
fun PreferencesScene(
    onAction: (PreferencesAction) -> Unit,
    viewModel: PreferencesViewModel = hiltViewModel(),
) {
    val rows by viewModel.rows.collectAsStateWithLifecycle()
    val configuration = LocalConfiguration.current
    val context = LocalContext.current

    Scene(
        title = stringResource(id = (R.string.settings_preferences_title)),
        onClose = { onAction(PreferencesAction.Cancel) },
    ) {
        LazyColumn {
            rows.forEach { section ->
                itemsIndexed(section) { index, row ->
                    val listPosition = ListPosition.getPosition(index, section.size)
                    when (row) {
                        is PreferencesRowUIModel.Link -> LinkItem(
                            title = stringResource(row.title),
                            painter = row.icon?.let { painterResource(it) },
                            listPosition = listPosition,
                            trailingContent = row.trailing?.let { trailing ->
                                @Composable {
                                    PropertyDataText(
                                        text = trailing,
                                        badge = { DataBadgeChevron() },
                                    )
                                }
                            },
                            onClick = { onAction(row.action) },
                        )
                        is PreferencesRowUIModel.Language -> if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                            val language = configuration.locales
                                .get(0).displayLanguage.replaceFirstChar {
                                    if (it.isLowerCase()) it.titlecase(Locale.ROOT) else it.toString()
                                }
                            LinkItem(
                                title = stringResource(row.title),
                                painter = row.icon?.let { painterResource(it) },
                                listPosition = listPosition,
                                trailingContent = {
                                    PropertyDataText(
                                        text = language,
                                        badge = { DataBadgeChevron() },
                                    )
                                },
                                onClick = {
                                    val intent = Intent(Settings.ACTION_APP_LOCALE_SETTINGS)
                                    intent.data = Uri.fromParts("package", context.packageName, null)
                                    context.startActivity(intent)
                                }
                            )
                        }
                        is PreferencesRowUIModel.AppearancePicker -> OptionPickerLinkItem(
                            title = stringResource(row.title),
                            current = row.current,
                            options = Appearance.entries,
                            listPosition = listPosition,
                            icon = row.icon,
                            indented = false,
                            label = { stringResource(it.stringRes()) },
                            onSelect = { viewModel.setAppearance(it) },
                        )
                        is PreferencesRowUIModel.PerpetualsSwitch -> LinkItem(
                            title = stringResource(row.title),
                            painter = row.icon?.let { painterResource(it) },
                            listPosition = listPosition,
                            trailingContent = {
                                Switch(
                                    checked = row.isEnabled,
                                    onCheckedChange = viewModel::setPerpetualEnabled,
                                )
                            },
                            onClick = { viewModel.setPerpetualEnabled(!row.isEnabled) },
                        )
                        is PreferencesRowUIModel.Picker -> OptionPickerLinkItem(
                            title = stringResource(row.title),
                            current = row.current,
                            options = row.options.map { it.value },
                            listPosition = listPosition,
                            label = { value -> row.options.first { it.value == value }.label },
                            onSelect = { viewModel.setPerpetualOption(row.setting, it) },
                        )
                    }
                }
            }
        }
    }
}
@Composable
private fun <T> OptionPickerLinkItem(
    title: String,
    current: T,
    options: List<T>,
    listPosition: ListPosition,
    label: @Composable (T) -> String,
    onSelect: (T) -> Unit,
    @DrawableRes icon: Int? = null,
    indented: Boolean = true,
) {
    var expanded by remember { mutableStateOf(false) }
    LinkItem(
        title = title,
        painter = icon?.let { painterResource(id = it) },
        listPosition = listPosition,
        indented = indented,
        trailingContent = {
            PropertyDataText(text = label(current), badge = { DataBadgeChevron() })
            DropdownMenu(
                expanded = expanded,
                onDismissRequest = { expanded = false },
                containerColor = MaterialTheme.colorScheme.background,
            ) {
                options.forEach { option ->
                    DropdownMenuItem(
                        text = {
                            Row(verticalAlignment = Alignment.CenterVertically) {
                                if (option == current) {
                                    Icon(AppIcons.Check, null, modifier = Modifier.size(compactIconSize))
                                } else {
                                    Spacer(modifier = Modifier.size(compactIconSize))
                                }
                                Spacer4()
                                Text(label(option))
                            }
                        },
                        onClick = {
                            onSelect(option)
                            expanded = false
                        },
                    )
                }
            }
        },
        onClick = { expanded = true },
    )
}
