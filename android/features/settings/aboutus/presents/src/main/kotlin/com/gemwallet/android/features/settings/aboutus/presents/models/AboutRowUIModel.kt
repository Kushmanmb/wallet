package com.gemwallet.android.features.settings.aboutus.presents.models

import androidx.annotation.StringRes
import com.gemwallet.android.AppUrl
import com.gemwallet.android.features.settings.aboutus.presents.localization.stringRes
import com.gemwallet.android.ui.components.list_item.property.SocialLinkUIModel
import com.gemwallet.android.ui.components.list_item.property.toSocialLinks
import uniffi.gemstone.GemAboutRow
import uniffi.gemstone.PublicUrl
import uniffi.gemstone.aboutSections
import uniffi.gemstone.communityLinks

sealed interface AboutRowUIModel {
    @get:StringRes val title: Int

    data class Link(@StringRes override val title: Int, val url: String) : AboutRowUIModel
    data class Community(@StringRes override val title: Int, val links: List<SocialLinkUIModel>) : AboutRowUIModel
    data class Version(@StringRes override val title: Int, val version: String) : AboutRowUIModel
}

internal fun aboutRows(version: String): List<List<AboutRowUIModel>> =
    aboutSections().map { section -> section.rows.map { it.uiModel(version) } }

private fun GemAboutRow.uiModel(version: String): AboutRowUIModel = when (this) {
    GemAboutRow.TERMS_OF_SERVICE -> AboutRowUIModel.Link(stringRes(), AppUrl.page(PublicUrl.TERMS_OF_SERVICE))
    GemAboutRow.PRIVACY_POLICY -> AboutRowUIModel.Link(stringRes(), AppUrl.page(PublicUrl.PRIVACY_POLICY))
    GemAboutRow.WEBSITE -> AboutRowUIModel.Link(stringRes(), AppUrl.page(PublicUrl.WEBSITE))
    GemAboutRow.COMMUNITY -> AboutRowUIModel.Community(stringRes(), communityLinks().toSocialLinks())
    GemAboutRow.VERSION -> AboutRowUIModel.Version(stringRes(), version)
}
