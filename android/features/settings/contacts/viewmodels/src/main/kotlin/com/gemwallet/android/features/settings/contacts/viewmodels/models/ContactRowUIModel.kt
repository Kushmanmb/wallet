package com.gemwallet.android.features.settings.contacts.viewmodels.models

import com.gemwallet.android.ext.toGem
import com.wallet.core.primitives.Contact
import com.wallet.core.primitives.ContactData
import uniffi.gemstone.contactRow

data class ContactRowUIModel(
    val id: String,
    val title: String,
    val subtitle: String?,
    val initials: String,
    val avatar: ContactAvatarState,
    val contact: Contact,
)

internal fun ContactData.uiModel(): ContactRowUIModel {
    val row = contactRow(contact.toGem())
    return ContactRowUIModel(
        id = contact.id,
        title = row.title,
        subtitle = row.subtitle,
        initials = row.initials,
        avatar = ContactAvatarState.from(contact.imageUrl),
        contact = contact,
    )
}
