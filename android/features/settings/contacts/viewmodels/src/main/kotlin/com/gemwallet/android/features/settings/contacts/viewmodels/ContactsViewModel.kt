package com.gemwallet.android.features.settings.contacts.viewmodels

import android.content.Context
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.gemwallet.android.application.contacts.cases.GetContacts
import com.gemwallet.android.ext.errorText
import com.gemwallet.android.ext.runCatchingCancellable
import com.gemwallet.android.ext.toGem
import com.gemwallet.android.features.settings.contacts.viewmodels.models.ContactRowUIModel
import com.gemwallet.android.features.settings.contacts.viewmodels.models.uiModel
import com.gemwallet.android.ui.localization.text
import com.wallet.core.primitives.Contact
import dagger.hilt.android.lifecycle.HiltViewModel
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import uniffi.gemstone.GemContactServiceInterface

@HiltViewModel
class ContactsViewModel @Inject constructor(
    getContacts: GetContacts,
    private val service: GemContactServiceInterface,
    @param:ApplicationContext private val context: Context,
) : ViewModel() {

    val contacts: StateFlow<List<ContactRowUIModel>> = getContacts.getContacts()
        .map { contacts -> contacts.map { it.uiModel() } }
        .stateIn(viewModelScope, SharingStarted.Eagerly, emptyList())

    private val errorState = MutableStateFlow<String?>(null)
    val errorText: StateFlow<String?> = errorState.asStateFlow()

    fun deleteContact(contact: Contact) {
        viewModelScope.launch(Dispatchers.IO) {
            runCatchingCancellable { service.deleteContact(contact.toGem()) }
                .onFailure { errorState.value = it.errorText().text(context) }
        }
    }

    fun clearError() = errorState.update { null }

}
