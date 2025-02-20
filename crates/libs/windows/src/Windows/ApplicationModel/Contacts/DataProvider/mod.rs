#[doc(hidden)]
#[repr(transparent)]
pub struct IContactDataProviderConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactDataProviderConnection {
    type Vtable = IContactDataProviderConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactDataProviderConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a398a52_8c9d_4d6f_a4e0_111e9a125a30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ServerSearchReadBatchRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerSearchReadBatchRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerSearchReadBatchRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerSearchReadBatchRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactDataProviderConnection2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactDataProviderConnection2 {
    type Vtable = IContactDataProviderConnection2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactDataProviderConnection2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1d327b0_196c_4bfd_8f0f_c68d67f249d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderConnection2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateOrUpdateContactRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrUpdateContactRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCreateOrUpdateContactRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCreateOrUpdateContactRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteContactRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContactRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeleteContactRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeleteContactRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactDataProviderTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactDataProviderTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x527104be_3c62_43c8_9ae7_db531685cd99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListCreateOrUpdateContactRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListCreateOrUpdateContactRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4af411f_c849_47d0_b119_91cf605b2f2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, createdorupdatedcontact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListCreateOrUpdateContactRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x851c1690_1a51_4b0c_aeef_1240ac5bed75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListDeleteContactRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListDeleteContactRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e114687_ce03_4de5_8557_9ccf552d472a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListDeleteContactRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListDeleteContactRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListDeleteContactRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22054a1_e8fa_4db5_9389_2d12ee7d15ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListDeleteContactRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListServerSearchReadBatchRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListServerSearchReadBatchRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba776a97_4030_4925_9fb4_143b295e653b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuggestedBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, batchstatus: super::ContactBatchStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListServerSearchReadBatchRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListServerSearchReadBatchRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a27e87b_69d7_4e4e_8042_861cba61471e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncManagerSyncRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListSyncManagerSyncRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c0e57a4_c4e7_4970_9a8f_9a66a2bb6c1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListSyncManagerSyncRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x158e4dac_446d_4f10_afc2_02683ec533a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactDataProviderConnection(::windows::core::IUnknown);
impl ContactDataProviderConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SyncRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSyncRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerSearchReadBatchRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServerSearchReadBatchRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerSearchReadBatchRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveServerSearchReadBatchRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateOrUpdateContactRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOrUpdateContactRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCreateOrUpdateContactRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCreateOrUpdateContactRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteContactRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteContactRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeleteContactRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDeleteContactRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for ContactDataProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactDataProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactDataProviderConnection {}
impl ::core::fmt::Debug for ContactDataProviderConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactDataProviderConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactDataProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection;{1a398a52-8c9d-4d6f-a4e0-111e9a125a30})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactDataProviderConnection {
    type Vtable = IContactDataProviderConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactDataProviderConnection {
    const IID: ::windows::core::GUID = <IContactDataProviderConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection";
}
::windows::core::interface_hierarchy!(ContactDataProviderConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactDataProviderConnection {}
unsafe impl ::core::marker::Sync for ContactDataProviderConnection {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactDataProviderTriggerDetails(::windows::core::IUnknown);
impl ContactDataProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows::core::Result<ContactDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ContactDataProviderConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactDataProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactDataProviderTriggerDetails {}
impl ::core::fmt::Debug for ContactDataProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactDataProviderTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactDataProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails;{527104be-3c62-43c8-9ae7-db531685cd99})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactDataProviderTriggerDetails {
    const IID: ::windows::core::GUID = <IContactDataProviderTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails";
}
::windows::core::interface_hierarchy!(ContactDataProviderTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for ContactDataProviderTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactListCreateOrUpdateContactRequest(::windows::core::IUnknown);
impl ContactListCreateOrUpdateContactRequest {
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Contact(&self) -> ::windows::core::Result<super::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contact)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contact>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self, createdorupdatedcontact: &super::Contact) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(createdorupdatedcontact), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListCreateOrUpdateContactRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListCreateOrUpdateContactRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListCreateOrUpdateContactRequest {}
impl ::core::fmt::Debug for ContactListCreateOrUpdateContactRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListCreateOrUpdateContactRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListCreateOrUpdateContactRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest;{b4af411f-c849-47d0-b119-91cf605b2f2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListCreateOrUpdateContactRequest {
    const IID: ::windows::core::GUID = <IContactListCreateOrUpdateContactRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListCreateOrUpdateContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest";
}
::windows::core::interface_hierarchy!(ContactListCreateOrUpdateContactRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListCreateOrUpdateContactRequest {}
unsafe impl ::core::marker::Sync for ContactListCreateOrUpdateContactRequest {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactListCreateOrUpdateContactRequestEventArgs(::windows::core::IUnknown);
impl ContactListCreateOrUpdateContactRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<ContactListCreateOrUpdateContactRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ContactListCreateOrUpdateContactRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListCreateOrUpdateContactRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListCreateOrUpdateContactRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListCreateOrUpdateContactRequestEventArgs {}
impl ::core::fmt::Debug for ContactListCreateOrUpdateContactRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListCreateOrUpdateContactRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListCreateOrUpdateContactRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs;{851c1690-1a51-4b0c-aeef-1240ac5bed75})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListCreateOrUpdateContactRequestEventArgs {
    const IID: ::windows::core::GUID = <IContactListCreateOrUpdateContactRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListCreateOrUpdateContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs";
}
::windows::core::interface_hierarchy!(ContactListCreateOrUpdateContactRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListCreateOrUpdateContactRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListCreateOrUpdateContactRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactListDeleteContactRequest(::windows::core::IUnknown);
impl ContactListDeleteContactRequest {
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListDeleteContactRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListDeleteContactRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListDeleteContactRequest {}
impl ::core::fmt::Debug for ContactListDeleteContactRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListDeleteContactRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListDeleteContactRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest;{5e114687-ce03-4de5-8557-9ccf552d472a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListDeleteContactRequest {
    const IID: ::windows::core::GUID = <IContactListDeleteContactRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListDeleteContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest";
}
::windows::core::interface_hierarchy!(ContactListDeleteContactRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListDeleteContactRequest {}
unsafe impl ::core::marker::Sync for ContactListDeleteContactRequest {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactListDeleteContactRequestEventArgs(::windows::core::IUnknown);
impl ContactListDeleteContactRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<ContactListDeleteContactRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ContactListDeleteContactRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListDeleteContactRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListDeleteContactRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListDeleteContactRequestEventArgs {}
impl ::core::fmt::Debug for ContactListDeleteContactRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListDeleteContactRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListDeleteContactRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs;{b22054a1-e8fa-4db5-9389-2d12ee7d15ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListDeleteContactRequestEventArgs {
    const IID: ::windows::core::GUID = <IContactListDeleteContactRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListDeleteContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs";
}
::windows::core::interface_hierarchy!(ContactListDeleteContactRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListDeleteContactRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListDeleteContactRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactListServerSearchReadBatchRequest(::windows::core::IUnknown);
impl ContactListServerSearchReadBatchRequest {
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<super::ContactQueryOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Options)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ContactQueryOptions>(result__)
        }
    }
    pub fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuggestedBatchSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveContactAsync(&self, contact: &super::Contact) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self, batchstatus: super::ContactBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), batchstatus, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListServerSearchReadBatchRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListServerSearchReadBatchRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListServerSearchReadBatchRequest {}
impl ::core::fmt::Debug for ContactListServerSearchReadBatchRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListServerSearchReadBatchRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListServerSearchReadBatchRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest;{ba776a97-4030-4925-9fb4-143b295e653b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListServerSearchReadBatchRequest {
    const IID: ::windows::core::GUID = <IContactListServerSearchReadBatchRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest";
}
::windows::core::interface_hierarchy!(ContactListServerSearchReadBatchRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListServerSearchReadBatchRequest {}
unsafe impl ::core::marker::Sync for ContactListServerSearchReadBatchRequest {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactListServerSearchReadBatchRequestEventArgs(::windows::core::IUnknown);
impl ContactListServerSearchReadBatchRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<ContactListServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ContactListServerSearchReadBatchRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListServerSearchReadBatchRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListServerSearchReadBatchRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListServerSearchReadBatchRequestEventArgs {}
impl ::core::fmt::Debug for ContactListServerSearchReadBatchRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListServerSearchReadBatchRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs;{1a27e87b-69d7-4e4e-8042-861cba61471e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListServerSearchReadBatchRequestEventArgs {
    const IID: ::windows::core::GUID = <IContactListServerSearchReadBatchRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs";
}
::windows::core::interface_hierarchy!(ContactListServerSearchReadBatchRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListServerSearchReadBatchRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListServerSearchReadBatchRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactListSyncManagerSyncRequest(::windows::core::IUnknown);
impl ContactListSyncManagerSyncRequest {
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListSyncManagerSyncRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListSyncManagerSyncRequest {}
impl ::core::fmt::Debug for ContactListSyncManagerSyncRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListSyncManagerSyncRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncManagerSyncRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest;{3c0e57a4-c4e7-4970-9a8f-9a66a2bb6c1a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListSyncManagerSyncRequest {
    const IID: ::windows::core::GUID = <IContactListSyncManagerSyncRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest";
}
::windows::core::interface_hierarchy!(ContactListSyncManagerSyncRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for ContactListSyncManagerSyncRequest {}
#[doc = "*Required features: `\"ApplicationModel_Contacts_DataProvider\"`*"]
#[repr(transparent)]
pub struct ContactListSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
impl ContactListSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<ContactListSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ContactListSyncManagerSyncRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListSyncManagerSyncRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListSyncManagerSyncRequestEventArgs {}
impl ::core::fmt::Debug for ContactListSyncManagerSyncRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListSyncManagerSyncRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs;{158e4dac-446d-4f10-afc2-02683ec533a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListSyncManagerSyncRequestEventArgs {
    const IID: ::windows::core::GUID = <IContactListSyncManagerSyncRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs";
}
::windows::core::interface_hierarchy!(ContactListSyncManagerSyncRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListSyncManagerSyncRequestEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
