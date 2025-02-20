#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeed(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeed {
    pub unsafe fn Xml(&self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Xml)(::windows::core::Vtable::as_raw(self), count, sortproperty, sortorder, filterflags, includeflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Rename(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Rename)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Url)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetUrl(&self, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedurl)).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Move(&self, newparentpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(newparentpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn LastWriteTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastWriteTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Download)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelAsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows::core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SyncSetting)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_SYNC_SETTING>(result__)
    }
    pub unsafe fn SetSyncSetting(&self, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSyncSetting)(::windows::core::Vtable::as_raw(self), syncsetting).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Interval)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalEnclosurePath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Items(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Items)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItem(&self, itemid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItem)(::windows::core::Vtable::as_raw(self), itemid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Title)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Link)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Image(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Image)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn LastBuildDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastBuildDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PubDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Ttl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Language(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Language)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Copyright)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxItemCount)(::windows::core::Vtable::as_raw(self), count).ok()
    }
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDownloadEnclosuresAutomatically(&self, downloadenclosuresautomatically: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), downloadenclosuresautomatically).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_STATUS>(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastDownloadError)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_ERROR>(result__)
    }
    pub unsafe fn Merge(&self, feedxml: &::windows::core::BSTR, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Merge)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedxml), ::core::mem::transmute_copy(feedurl)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn IsList(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MarkAllItemsRead)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWatcher)(::windows::core::Vtable::as_raw(self), scope, mask, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UnreadItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeed, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeed {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeed").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeed {
    type Vtable = IFeed_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeed {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7f915d8_2ede_42bc_98e7_a5d05063a757);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeed_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedguid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    pub LastWriteTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastwrite: *mut f64) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelAsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SyncSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT,
    pub SetSyncSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub LastDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT,
    pub LocalEnclosurePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Items: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetItem: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, homepage: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LastBuildDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastbuilddate: *mut f64) -> ::windows::core::HRESULT,
    pub PubDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastpopulatedate: *mut f64) -> ::windows::core::HRESULT,
    pub Ttl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ttl: *mut i32) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Copyright: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copyright: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub MaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT,
    pub DownloadEnclosuresAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: *mut i16) -> ::windows::core::HRESULT,
    pub SetDownloadEnclosuresAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: i16) -> ::windows::core::HRESULT,
    pub DownloadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT,
    pub LastDownloadError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT,
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedxml: ::core::mem::ManuallyDrop<::windows::core::BSTR>, feedurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub IsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, islist: *mut i16) -> ::windows::core::HRESULT,
    pub MarkAllItemsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWatcher: usize,
    pub UnreadItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeed2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeed2 {
    pub unsafe fn Xml(&self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Xml)(::windows::core::Vtable::as_raw(self), count, sortproperty, sortorder, filterflags, includeflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Rename(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Rename)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Url)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetUrl(&self, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedurl)).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Move(&self, newparentpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(newparentpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn LastWriteTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastWriteTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Download)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelAsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows::core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SyncSetting)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_SYNC_SETTING>(result__)
    }
    pub unsafe fn SetSyncSetting(&self, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSyncSetting)(::windows::core::Vtable::as_raw(self), syncsetting).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Interval)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalEnclosurePath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Items(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Items)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItem(&self, itemid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), itemid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Link)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Image(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Image)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn LastBuildDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastBuildDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PubDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Ttl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Language(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Language)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Copyright)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MaxItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaxItemCount)(::windows::core::Vtable::as_raw(self), count).ok()
    }
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDownloadEnclosuresAutomatically(&self, downloadenclosuresautomatically: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), downloadenclosuresautomatically).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_STATUS>(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadError)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_ERROR>(result__)
    }
    pub unsafe fn Merge(&self, feedxml: &::windows::core::BSTR, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Merge)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedxml), ::core::mem::transmute_copy(feedurl)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn IsList(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MarkAllItemsRead)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWatcher)(::windows::core::Vtable::as_raw(self), scope, mask, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UnreadItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItemByEffectiveId(&self, itemeffectiveid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItemByEffectiveId)(::windows::core::Vtable::as_raw(self), itemeffectiveid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn LastItemDownloadTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastItemDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Username(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Username)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Password(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Password)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetCredentials(&self, username: &::windows::core::BSTR, password: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCredentials)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(username), ::core::mem::transmute_copy(password)).ok()
    }
    pub unsafe fn ClearCredentials(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ClearCredentials)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeed2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFeed);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeed2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeed2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeed2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeed2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeed2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeed2 {
    type Vtable = IFeed2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeed2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f2ea09_1398_4ab9_b6a4_f94b49d0a42e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeed2_Vtbl {
    pub base__: IFeed_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetItemByEffectiveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemeffectiveid: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetItemByEffectiveId: usize,
    pub LastItemDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastitemdownloadtime: *mut f64) -> ::windows::core::HRESULT,
    pub Username: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, password: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::BSTR>, password: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ClearCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeedEnclosure(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedEnclosure {
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Url)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Length)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelAsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_STATUS>(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastDownloadError)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_ERROR>(result__)
    }
    pub unsafe fn LocalPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn DownloadMimeType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadMimeType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn RemoveFile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveFile)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetFile(&self, downloadurl: &::windows::core::BSTR, downloadfilepath: &::windows::core::BSTR, downloadmimetype: &::windows::core::BSTR, enclosurefilename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(downloadurl), ::core::mem::transmute_copy(downloadfilepath), ::core::mem::transmute_copy(downloadmimetype), ::core::mem::transmute_copy(enclosurefilename)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeedEnclosure, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeedEnclosure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedEnclosure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedEnclosure {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedEnclosure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedEnclosure").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeedEnclosure {
    type Vtable = IFeedEnclosure_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeedEnclosure {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x361c26f7_90a4_4e67_ae09_3a36a546436a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedEnclosure_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enclosureurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub AsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelAsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DownloadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT,
    pub LastDownloadError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT,
    pub LocalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpath: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enclosureurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DownloadMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub RemoveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, downloadfilepath: ::core::mem::ManuallyDrop<::windows::core::BSTR>, downloadmimetype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, enclosurefilename: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeedEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedEvents {
    pub unsafe fn Error(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FeedDeleted(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FeedRenamed(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedRenamed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(oldpath)).ok()
    }
    pub unsafe fn FeedUrlChanged(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedUrlChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FeedMoved(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedMoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(oldpath)).ok()
    }
    pub unsafe fn FeedDownloading(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedDownloading)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FeedDownloadCompleted(&self, path: &::windows::core::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedDownloadCompleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), error).ok()
    }
    pub unsafe fn FeedItemCountChanged(&self, path: &::windows::core::BSTR, itemcounttype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedItemCountChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), itemcounttype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeedEvents, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeedEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeedEvents {
    type Vtable = IFeedEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeedEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabf35c99_0681_47ea_9a8c_1436a375a99e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FeedDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, oldpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedUrlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, oldpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedDownloading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT,
    pub FeedItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeedFolder(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedFolder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Feeds(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Feeds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subfolders(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Subfolders)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFeed(&self, feedname: &::windows::core::BSTR, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFeed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedname), ::core::mem::transmute_copy(feedurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSubfolder(&self, foldername: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSubfolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(foldername), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn ExistsFeed(&self, feedname: &::windows::core::BSTR) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExistsFeed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeed(&self, feedname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFeed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn ExistsSubfolder(&self, foldername: &::windows::core::BSTR) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExistsSubfolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(foldername), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSubfolder(&self, foldername: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubfolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(foldername), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Rename(&self, foldername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Rename)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(foldername)).ok()
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Move(&self, newparentpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(newparentpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn IsRoot(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsRoot)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn TotalUnreadItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalUnreadItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TotalItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWatcher)(::windows::core::Vtable::as_raw(self), scope, mask, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeedFolder, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeedFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedFolder {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedFolder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeedFolder {
    type Vtable = IFeedFolder_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeedFolder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81f04ad1_4194_4d7d_86d6_11813cec163c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedFolder_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Feeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Feeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Subfolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Subfolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, feedurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFeed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSubfolder: usize,
    pub ExistsFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeed: usize,
    pub ExistsSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<::windows::core::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSubfolder: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderpath: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    pub IsRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isroot: *mut i16) -> ::windows::core::HRESULT,
    pub TotalUnreadItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub TotalItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWatcher: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeedFolderEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedFolderEvents {
    pub unsafe fn Error(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FolderAdded(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FolderAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FolderDeleted(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FolderDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FolderRenamed(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FolderRenamed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(oldpath)).ok()
    }
    pub unsafe fn FolderMovedFrom(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FolderMovedFrom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(oldpath)).ok()
    }
    pub unsafe fn FolderMovedTo(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FolderMovedTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(oldpath)).ok()
    }
    pub unsafe fn FolderItemCountChanged(&self, path: &::windows::core::BSTR, itemcounttype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FolderItemCountChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), itemcounttype).ok()
    }
    pub unsafe fn FeedAdded(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FeedDeleted(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FeedRenamed(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedRenamed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(oldpath)).ok()
    }
    pub unsafe fn FeedUrlChanged(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedUrlChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FeedMovedFrom(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedMovedFrom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(oldpath)).ok()
    }
    pub unsafe fn FeedMovedTo(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedMovedTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(oldpath)).ok()
    }
    pub unsafe fn FeedDownloading(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedDownloading)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path)).ok()
    }
    pub unsafe fn FeedDownloadCompleted(&self, path: &::windows::core::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedDownloadCompleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), error).ok()
    }
    pub unsafe fn FeedItemCountChanged(&self, path: &::windows::core::BSTR, itemcounttype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FeedItemCountChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), itemcounttype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeedFolderEvents, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeedFolderEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedFolderEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedFolderEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedFolderEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedFolderEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeedFolderEvents {
    type Vtable = IFeedFolderEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeedFolderEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20a59fa6_a844_4630_9e98_175f70b4d55b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedFolderEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FolderAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FolderDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FolderRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, oldpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FolderMovedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, oldpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FolderMovedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, oldpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FolderItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT,
    pub FeedAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, oldpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedUrlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedMovedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, oldpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedMovedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, oldpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedDownloading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FeedDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT,
    pub FeedItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeedItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedItem {
    pub unsafe fn Xml(&self, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Xml)(::windows::core::Vtable::as_raw(self), includeflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Title)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Link)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Guid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PubDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Comments)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Author(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Author)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enclosure(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Enclosure)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn IsRead(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsRead)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsRead(&self, isread: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetIsRead)(::windows::core::Vtable::as_raw(self), isread).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Modified(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Modified)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeedItem, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeedItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeedItem {
    type Vtable = IFeedItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeedItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a1e6cad_0a47_4da2_a13d_5baaa5c8bd4f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linkurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemguid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub PubDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pubdate: *mut f64) -> ::windows::core::HRESULT,
    pub Comments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comments: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, author: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Enclosure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Enclosure: usize,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isread: *mut i16) -> ::windows::core::HRESULT,
    pub SetIsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isread: i16) -> ::windows::core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LastDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT,
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeedItem2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedItem2 {
    pub unsafe fn Xml(&self, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Xml)(::windows::core::Vtable::as_raw(self), includeflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Link)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Guid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PubDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Comments)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Author(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Author)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enclosure(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Enclosure)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn IsRead(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRead)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsRead(&self, isread: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetIsRead)(::windows::core::Vtable::as_raw(self), isread).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Modified(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Modified)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn EffectiveId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EffectiveId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeedItem2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFeedItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeedItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeedItem2 {
    type Vtable = IFeedItem2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeedItem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79ac9ef4_f9c1_4d2b_a50b_a7ffba4dcf37);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedItem2_Vtbl {
    pub base__: IFeedItem_Vtbl,
    pub EffectiveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectiveid: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeedsEnum(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedsEnum {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Item)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeedsEnum, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeedsEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedsEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedsEnum {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedsEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedsEnum").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeedsEnum {
    type Vtable = IFeedsEnum_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeedsEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3cd0028_2eed_4c60_8fae_a3225309a836);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedsEnum_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvar: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFeedsManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFeedsManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootFolder(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RootFolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn IsSubscribed(&self, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn ExistsFeed(&self, feedpath: &::windows::core::BSTR) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExistsFeed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedpath), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeed(&self, feedpath: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFeed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedpath), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeedByUrl(&self, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFeedByUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn ExistsFolder(&self, folderpath: &::windows::core::BSTR) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExistsFolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(folderpath), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFolder(&self, folderpath: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(folderpath), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    pub unsafe fn DeleteFeed(&self, feedpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteFeed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedpath)).ok()
    }
    pub unsafe fn DeleteFolder(&self, folderpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteFolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(folderpath)).ok()
    }
    pub unsafe fn BackgroundSync(&self, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BackgroundSync)(::windows::core::Vtable::as_raw(self), action).ok()
    }
    pub unsafe fn BackgroundSyncStatus(&self) -> ::windows::core::Result<FEEDS_BACKGROUNDSYNC_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BackgroundSyncStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_BACKGROUNDSYNC_STATUS>(result__)
    }
    pub unsafe fn DefaultInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DefaultInterval)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDefaultInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn AsyncSyncAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AsyncSyncAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Normalize(&self, feedxmlin: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Normalize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(feedxmlin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn ItemCountLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ItemCountLimit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFeedsManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFeedsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFeedsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFeedsManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFeedsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedsManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFeedsManager {
    type Vtable = IFeedsManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFeedsManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74029cc_1f1a_4906_88f0_810638d86591);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFeedsManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RootFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootFolder: usize,
    pub IsSubscribed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, subscribed: *mut i16) -> ::windows::core::HRESULT,
    pub ExistsFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeedByUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeedByUrl: usize,
    pub ExistsFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFolder: usize,
    pub DeleteFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub BackgroundSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT,
    pub BackgroundSyncStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT,
    pub DefaultInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub SetDefaultInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub AsyncSyncAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedxmlin: ::core::mem::ManuallyDrop<::windows::core::BSTR>, feedxmlout: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ItemCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemcountlimit: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPAudioRenderConfig(::windows::core::IUnknown);
impl IWMPAudioRenderConfig {
    pub unsafe fn audioOutputDevice(&self, pbstroutputdevice: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).audioOutputDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstroutputdevice)).ok()
    }
    pub unsafe fn SetaudioOutputDevice(&self, bstroutputdevice: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetaudioOutputDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstroutputdevice)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPAudioRenderConfig, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPAudioRenderConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPAudioRenderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPAudioRenderConfig {}
impl ::core::fmt::Debug for IWMPAudioRenderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPAudioRenderConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPAudioRenderConfig {
    type Vtable = IWMPAudioRenderConfig_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPAudioRenderConfig {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79c6349_5997_4ce4_917c_22a3391ec564);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPAudioRenderConfig_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub audioOutputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstroutputdevice: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetaudioOutputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroutputdevice: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPCdrom(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdrom {
    pub unsafe fn driveSpecifier(&self, pbstrdrive: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).driveSpecifier)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdrive)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).playlist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    pub unsafe fn eject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).eject)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPCdrom, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPCdrom {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCdrom {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCdrom {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCdrom {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCdrom").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPCdrom {
    type Vtable = IWMPCdrom_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPCdrom {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfab6e98_8730_11d3_b388_00c04f68574b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdrom_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub driveSpecifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdrive: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub playlist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    playlist: usize,
    pub eject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPCdromBurn(::windows::core::IUnknown);
impl IWMPCdromBurn {
    pub unsafe fn isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), ::core::mem::transmute(pisavailable)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritem: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn label(&self, pbstrlabel: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).label)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrlabel)).ok()
    }
    pub unsafe fn Setlabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setlabel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrlabel)).ok()
    }
    pub unsafe fn burnFormat(&self, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).burnFormat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpbf)).ok()
    }
    pub unsafe fn SetburnFormat(&self, wmpbf: WMPBurnFormat) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetburnFormat)(::windows::core::Vtable::as_raw(self), wmpbf).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn burnPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).burnPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetburnPlaylist<'a, P0>(&self, pplaylist: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).SetburnPlaylist)(::windows::core::Vtable::as_raw(self), pplaylist.into().abi()).ok()
    }
    pub unsafe fn refreshStatus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).refreshStatus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn burnState(&self, pwmpbs: *mut WMPBurnState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).burnState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpbs)).ok()
    }
    pub unsafe fn burnProgress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).burnProgress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plprogress)).ok()
    }
    pub unsafe fn startBurn(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).startBurn)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stopBurn(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stopBurn)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn erase(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).erase)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPCdromBurn, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPCdromBurn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPCdromBurn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPCdromBurn {}
impl ::core::fmt::Debug for IWMPCdromBurn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCdromBurn").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPCdromBurn {
    type Vtable = IWMPCdromBurn_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPCdromBurn {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd94dbeb_417f_4928_aa06_087d56ed9b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromBurn_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub isAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setlabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub burnFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::HRESULT,
    pub SetburnFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmpbf: WMPBurnFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub burnPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    burnPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetburnPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplaylist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetburnPlaylist: usize,
    pub refreshStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub burnState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> ::windows::core::HRESULT,
    pub burnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT,
    pub startBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub stopBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub erase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPCdromCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdromCollection {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn item(&self, lindex: i32) -> ::windows::core::Result<IWMPCdrom> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).item)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdrom>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByDriveSpecifier(&self, bstrdrivespecifier: &::windows::core::BSTR) -> ::windows::core::Result<IWMPCdrom> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getByDriveSpecifier)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdrivespecifier), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdrom>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPCdromCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPCdromCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCdromCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCdromCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCdromCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCdromCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPCdromCollection {
    type Vtable = IWMPCdromCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPCdromCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee4c8fe2_34b2_11d3_a3bf_006097c9b344);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByDriveSpecifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdrivespecifier: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppcdrom: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByDriveSpecifier: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPCdromRip(::windows::core::IUnknown);
impl IWMPCdromRip {
    pub unsafe fn ripState(&self, pwmprs: *mut WMPRipState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ripState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmprs)).ok()
    }
    pub unsafe fn ripProgress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ripProgress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plprogress)).ok()
    }
    pub unsafe fn startRip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).startRip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stopRip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stopRip)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPCdromRip, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPCdromRip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPCdromRip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPCdromRip {}
impl ::core::fmt::Debug for IWMPCdromRip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCdromRip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPCdromRip {
    type Vtable = IWMPCdromRip_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPCdromRip {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56e2294f_69ed_4629_a869_aea72c0dcc2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromRip_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ripState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmprs: *mut WMPRipState) -> ::windows::core::HRESULT,
    pub ripProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT,
    pub startRip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub stopRip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPClosedCaption(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaption {
    pub unsafe fn SAMIStyle(&self, pbstrsamistyle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SAMIStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamistyle)).ok()
    }
    pub unsafe fn SetSAMIStyle(&self, bstrsamistyle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSAMIStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamistyle)).ok()
    }
    pub unsafe fn SAMILang(&self, pbstrsamilang: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SAMILang)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamilang)).ok()
    }
    pub unsafe fn SetSAMILang(&self, bstrsamilang: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSAMILang)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamilang)).ok()
    }
    pub unsafe fn SAMIFileName(&self, pbstrsamifilename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SAMIFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamifilename)).ok()
    }
    pub unsafe fn SetSAMIFileName(&self, bstrsamifilename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSAMIFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamifilename)).ok()
    }
    pub unsafe fn captioningId(&self, pbstrcaptioningid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).captioningId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcaptioningid)).ok()
    }
    pub unsafe fn SetcaptioningId(&self, bstrcaptioningid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetcaptioningId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcaptioningid)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPClosedCaption, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPClosedCaption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPClosedCaption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPClosedCaption {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPClosedCaption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPClosedCaption").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPClosedCaption {
    type Vtable = IWMPClosedCaption_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPClosedCaption {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f2df574_c588_11d3_9ed0_00c04fb6e937);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPClosedCaption_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SAMIStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsamistyle: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSAMIStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsamistyle: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SAMILang: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsamilang: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSAMILang: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsamilang: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SAMIFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsamifilename: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSAMIFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsamifilename: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub captioningId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcaptioningid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetcaptioningId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcaptioningid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPClosedCaption2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaption2 {
    pub unsafe fn SAMIStyle(&self, pbstrsamistyle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SAMIStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamistyle)).ok()
    }
    pub unsafe fn SetSAMIStyle(&self, bstrsamistyle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSAMIStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamistyle)).ok()
    }
    pub unsafe fn SAMILang(&self, pbstrsamilang: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SAMILang)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamilang)).ok()
    }
    pub unsafe fn SetSAMILang(&self, bstrsamilang: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSAMILang)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamilang)).ok()
    }
    pub unsafe fn SAMIFileName(&self, pbstrsamifilename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SAMIFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsamifilename)).ok()
    }
    pub unsafe fn SetSAMIFileName(&self, bstrsamifilename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSAMIFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsamifilename)).ok()
    }
    pub unsafe fn captioningId(&self, pbstrcaptioningid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.captioningId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcaptioningid)).ok()
    }
    pub unsafe fn SetcaptioningId(&self, bstrcaptioningid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetcaptioningId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcaptioningid)).ok()
    }
    pub unsafe fn SAMILangCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SAMILangCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn getSAMILangName(&self, nindex: i32, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getSAMILangName)(::windows::core::Vtable::as_raw(self), nindex, ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn getSAMILangID(&self, nindex: i32, pllangid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getSAMILangID)(::windows::core::Vtable::as_raw(self), nindex, ::core::mem::transmute(pllangid)).ok()
    }
    pub unsafe fn SAMIStyleCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SAMIStyleCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn getSAMIStyleName(&self, nindex: i32, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getSAMIStyleName)(::windows::core::Vtable::as_raw(self), nindex, ::core::mem::transmute(pbstrname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPClosedCaption2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPClosedCaption);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPClosedCaption2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPClosedCaption2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPClosedCaption2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPClosedCaption2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPClosedCaption2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPClosedCaption2 {
    type Vtable = IWMPClosedCaption2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPClosedCaption2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x350ba78b_6bc8_4113_a5f5_312056934eb6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPClosedCaption2_Vtbl {
    pub base__: IWMPClosedCaption_Vtbl,
    pub SAMILangCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub getSAMILangName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getSAMILangID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT,
    pub SAMIStyleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub getSAMIStyleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPContentContainer(::windows::core::IUnknown);
impl IWMPContentContainer {
    pub unsafe fn GetID(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPrice(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetContentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetContentPrice(&self, idxcontent: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentPrice)(::windows::core::Vtable::as_raw(self), idxcontent, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetContentID(&self, idxcontent: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentID)(::windows::core::Vtable::as_raw(self), idxcontent, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
::windows::core::interface_hierarchy!(IWMPContentContainer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPContentContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPContentContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPContentContainer {}
impl ::core::fmt::Debug for IWMPContentContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPContentContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPContentContainer {
    type Vtable = IWMPContentContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPContentContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad7f4d9c_1a9f_4ed2_9815_ecc0b58cb616);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPContentContainer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontentid: *mut u32) -> ::windows::core::HRESULT,
    pub GetPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprice: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetContentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccontent: *mut u32) -> ::windows::core::HRESULT,
    pub GetContentPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idxcontent: u32, pbstrprice: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetContentID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPContentContainerList(::windows::core::IUnknown);
impl IWMPContentContainerList {
    pub unsafe fn GetTransactionType(&self) -> ::windows::core::Result<WMPTransactionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransactionType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMPTransactionType>(result__)
    }
    pub unsafe fn GetContainerCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContainerCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetContainer(&self, idxcontainer: u32) -> ::windows::core::Result<IWMPContentContainer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContainer)(::windows::core::Vtable::as_raw(self), idxcontainer, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPContentContainer>(result__)
    }
}
::windows::core::interface_hierarchy!(IWMPContentContainerList, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPContentContainerList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPContentContainerList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPContentContainerList {}
impl ::core::fmt::Debug for IWMPContentContainerList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPContentContainerList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPContentContainerList {
    type Vtable = IWMPContentContainerList_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPContentContainerList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9937f78_0802_4af8_8b8d_e3f045bc8ab5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPContentContainerList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTransactionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> ::windows::core::HRESULT,
    pub GetContainerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccontainer: *mut u32) -> ::windows::core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idxcontainer: u32, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPContentPartner(::windows::core::IUnknown);
impl IWMPContentPartner {
    pub unsafe fn SetCallback<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPContentPartnerCallback>>,
    {
        (::windows::core::Vtable::vtable(self).SetCallback)(::windows::core::Vtable::as_raw(self), pcallback.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Notify(&self, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Notify)(::windows::core::Vtable::as_raw(self), r#type, ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetItemInfo(&self, bstrinfoname: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrinfoname), ::core::mem::transmute(pcontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetContentPartnerInfo(&self, bstrinfoname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentPartnerInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrinfoname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCommands(&self, location: &::windows::core::BSTR, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: &::windows::core::BSTR, prgitemids: &[u32], pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCommands)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(location), ::core::mem::transmute(plocationcontext), ::core::mem::transmute_copy(itemlocation), prgitemids.len() as _, ::core::mem::transmute(prgitemids.as_ptr()), ::core::mem::transmute(pcitemids), ::core::mem::transmute(pprgitems)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InvokeCommand(&self, dwcommandid: u32, location: &::windows::core::BSTR, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: &::windows::core::BSTR, rgitemids: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InvokeCommand)(::windows::core::Vtable::as_raw(self), dwcommandid, ::core::mem::transmute_copy(location), ::core::mem::transmute(plocationcontext), ::core::mem::transmute_copy(itemlocation), rgitemids.len() as _, ::core::mem::transmute(rgitemids.as_ptr())).ok()
    }
    pub unsafe fn CanBuySilent<'a, P0>(&self, pinfo: P0, pbstrtotalprice: *mut ::windows::core::BSTR, psilentok: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPContentContainerList>>,
    {
        (::windows::core::Vtable::vtable(self).CanBuySilent)(::windows::core::Vtable::as_raw(self), pinfo.into().abi(), ::core::mem::transmute(pbstrtotalprice), ::core::mem::transmute(psilentok)).ok()
    }
    pub unsafe fn Buy<'a, P0>(&self, pinfo: P0, cookie: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPContentContainerList>>,
    {
        (::windows::core::Vtable::vtable(self).Buy)(::windows::core::Vtable::as_raw(self), pinfo.into().abi(), cookie).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetStreamingURL(&self, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStreamingURL)(::windows::core::Vtable::as_raw(self), st, ::core::mem::transmute(pstreamcontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn Download<'a, P0>(&self, pinfo: P0, cookie: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPContentContainerList>>,
    {
        (::windows::core::Vtable::vtable(self).Download)(::windows::core::Vtable::as_raw(self), pinfo.into().abi(), cookie).ok()
    }
    pub unsafe fn DownloadTrackComplete(&self, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DownloadTrackComplete)(::windows::core::Vtable::as_raw(self), hrresult, contentid, ::core::mem::transmute_copy(downloadtrackparam)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RefreshLicense(&self, dwcookie: u32, flocal: i16, bstrurl: &::windows::core::BSTR, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: &::windows::core::BSTR, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RefreshLicense)(::windows::core::Vtable::as_raw(self), dwcookie, flocal, ::core::mem::transmute_copy(bstrurl), r#type, contentid, ::core::mem::transmute_copy(bstrrefreshreason), ::core::mem::transmute(preasoncontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCatalogURL(&self, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut ::windows::core::BSTR, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCatalogURL)(::windows::core::Vtable::as_raw(self), dwcatalogversion, dwcatalogschemaversion, cataloglcid, ::core::mem::transmute(pdwnewcatalogversion), ::core::mem::transmute(pbstrcatalogurl), ::core::mem::transmute(pexpirationdate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTemplate(&self, task: WMPTaskType, location: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT, clicklocation: &::windows::core::BSTR, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: &::windows::core::BSTR, bstrviewparams: &::windows::core::BSTR, pbstrtemplateurl: *mut ::windows::core::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTemplate)(::windows::core::Vtable::as_raw(self), task, ::core::mem::transmute_copy(location), ::core::mem::transmute(pcontext), ::core::mem::transmute_copy(clicklocation), ::core::mem::transmute(pclickcontext), ::core::mem::transmute_copy(bstrfilter), ::core::mem::transmute_copy(bstrviewparams), ::core::mem::transmute(pbstrtemplateurl), ::core::mem::transmute(ptemplatesize)).ok()
    }
    pub unsafe fn UpdateDevice(&self, bstrdevicename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdevicename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetListContents(&self, location: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: &::windows::core::BSTR, bstrparams: &::windows::core::BSTR, dwlistcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetListContents)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(location), ::core::mem::transmute(pcontext), ::core::mem::transmute_copy(bstrlisttype), ::core::mem::transmute_copy(bstrparams), dwlistcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Login(&self, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: i16, foktocache: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Login)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(userinfo), ::core::mem::transmute(pwdinfo), fusedcachedcreds, foktocache).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Authenticate(&self, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Authenticate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(userinfo), ::core::mem::transmute(pwdinfo)).ok()
    }
    pub unsafe fn Logout(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Logout)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SendMessage(&self, bstrmsg: &::windows::core::BSTR, bstrparam: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SendMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmsg), ::core::mem::transmute_copy(bstrparam)).ok()
    }
    pub unsafe fn StationEvent(&self, bstrstationeventtype: &::windows::core::BSTR, stationid: u32, playlistindex: u32, trackid: u32, trackdata: &::windows::core::BSTR, dwsecondsplayed: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StationEvent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrstationeventtype), stationid, playlistindex, trackid, ::core::mem::transmute_copy(trackdata), dwsecondsplayed).ok()
    }
    pub unsafe fn CompareContainerListPrices<'a, P0, P1>(&self, plistbase: P0, plistcompare: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPContentContainerList>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWMPContentContainerList>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CompareContainerListPrices)(::windows::core::Vtable::as_raw(self), plistbase.into().abi(), plistcompare.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VerifyPermission(&self, bstrpermission: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).VerifyPermission)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpermission), ::core::mem::transmute(pcontext)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPContentPartner, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPContentPartner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPContentPartner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPContentPartner {}
impl ::core::fmt::Debug for IWMPContentPartner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPContentPartner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPContentPartner {
    type Vtable = IWMPContentPartner_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPContentPartner {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55455073_41b5_4e75_87b8_f13bdb291d08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPContentPartner_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Notify: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pdata: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetItemInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetContentPartnerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pdata: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetContentPartnerInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<::windows::core::BSTR>, plocationcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, itemlocation: ::core::mem::ManuallyDrop<::windows::core::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut ::core::mem::ManuallyDrop<WMPContextMenuInfo>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCommands: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InvokeCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcommandid: u32, location: ::core::mem::ManuallyDrop<::windows::core::BSTR>, plocationcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, itemlocation: ::core::mem::ManuallyDrop<::windows::core::BSTR>, citemids: u32, rgitemids: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InvokeCommand: usize,
    pub CanBuySilent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, pbstrtotalprice: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>, psilentok: *mut i16) -> ::windows::core::HRESULT,
    pub Buy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetStreamingURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetStreamingURL: usize,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub DownloadTrackComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RefreshLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, flocal: i16, bstrurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: ::core::mem::ManuallyDrop<::windows::core::BSTR>, preasoncontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RefreshLicense: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCatalogURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>, pexpirationdate: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCatalogURL: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: WMPTaskType, location: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, clicklocation: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pclickcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bstrfilter: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrviewparams: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrtemplateurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetTemplate: usize,
    pub UpdateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetListContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bstrlisttype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrparams: ::core::mem::ManuallyDrop<::windows::core::BSTR>, dwlistcookie: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetListContents: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Login: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: i16, foktocache: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Login: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Authenticate: usize,
    pub Logout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrparam: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub StationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstationeventtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: ::core::mem::ManuallyDrop<::windows::core::BSTR>, dwsecondsplayed: u32) -> ::windows::core::HRESULT,
    pub CompareContainerListPrices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plistbase: *mut ::core::ffi::c_void, plistcompare: *mut ::core::ffi::c_void, presult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub VerifyPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    VerifyPermission: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPContentPartnerCallback(::windows::core::IUnknown);
impl IWMPContentPartnerCallback {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Notify(&self, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Notify)(::windows::core::Vtable::as_raw(self), r#type, ::core::mem::transmute(pcontext)).ok()
    }
    pub unsafe fn BuyComplete(&self, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BuyComplete)(::windows::core::Vtable::as_raw(self), hrresult, dwbuycookie).ok()
    }
    pub unsafe fn DownloadTrack(&self, cookie: u32, bstrtrackurl: &::windows::core::BSTR, dwservicetrackid: u32, bstrdownloadparams: &::windows::core::BSTR, hrdownload: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DownloadTrack)(::windows::core::Vtable::as_raw(self), cookie, ::core::mem::transmute_copy(bstrtrackurl), dwservicetrackid, ::core::mem::transmute_copy(bstrdownloadparams), hrdownload).ok()
    }
    pub unsafe fn GetCatalogVersion(&self, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCatalogVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwversion), ::core::mem::transmute(pdwschemaversion), ::core::mem::transmute(plcid)).ok()
    }
    pub unsafe fn UpdateDeviceComplete(&self, bstrdevicename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateDeviceComplete)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdevicename)).ok()
    }
    pub unsafe fn ChangeView(&self, bstrtype: &::windows::core::BSTR, bstrid: &::windows::core::BSTR, bstrfilter: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ChangeView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype), ::core::mem::transmute_copy(bstrid), ::core::mem::transmute_copy(bstrfilter)).ok()
    }
    pub unsafe fn AddListContents(&self, dwlistcookie: u32, prgitems: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddListContents)(::windows::core::Vtable::as_raw(self), dwlistcookie, prgitems.len() as _, ::core::mem::transmute(prgitems.as_ptr())).ok()
    }
    pub unsafe fn ListContentsComplete(&self, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ListContentsComplete)(::windows::core::Vtable::as_raw(self), dwlistcookie, hrsuccess).ok()
    }
    pub unsafe fn SendMessageComplete(&self, bstrmsg: &::windows::core::BSTR, bstrparam: &::windows::core::BSTR, bstrresult: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SendMessageComplete)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmsg), ::core::mem::transmute_copy(bstrparam), ::core::mem::transmute_copy(bstrresult)).ok()
    }
    pub unsafe fn GetContentIDsInLibrary(&self, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetContentIDsInLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pccontentids), ::core::mem::transmute(pprgids)).ok()
    }
    pub unsafe fn RefreshLicenseComplete(&self, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RefreshLicenseComplete)(::windows::core::Vtable::as_raw(self), dwcookie, contentid, hrrefresh).ok()
    }
    pub unsafe fn ShowPopup(&self, lindex: i32, bstrparameters: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShowPopup)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute_copy(bstrparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VerifyPermissionComplete(&self, bstrpermission: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).VerifyPermissionComplete)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpermission), ::core::mem::transmute(pcontext), hrpermission).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPContentPartnerCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPContentPartnerCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPContentPartnerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPContentPartnerCallback {}
impl ::core::fmt::Debug for IWMPContentPartnerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPContentPartnerCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPContentPartnerCallback {
    type Vtable = IWMPContentPartnerCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPContentPartnerCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e8f7da2_0695_403c_b697_da10fafaa676);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPContentPartnerCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Notify: usize,
    pub BuyComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::HRESULT,
    pub DownloadTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32, bstrtrackurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, dwservicetrackid: u32, bstrdownloadparams: ::core::mem::ManuallyDrop<::windows::core::BSTR>, hrdownload: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetCatalogVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub UpdateDeviceComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ChangeView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrid: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrfilter: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub AddListContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::HRESULT,
    pub ListContentsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub SendMessageComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrparam: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrresult: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetContentIDsInLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::HRESULT,
    pub RefreshLicenseComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ShowPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, bstrparameters: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub VerifyPermissionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pcontext: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, hrpermission: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    VerifyPermissionComplete: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPControls(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls {
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), ::core::mem::transmute(pisavailable)).ok()
    }
    pub unsafe fn play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).play)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastForward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).fastForward)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastReverse(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).fastReverse)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).currentPosition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdcurrentposition)).ok()
    }
    pub unsafe fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetcurrentPosition)(::windows::core::Vtable::as_raw(self), dcurrentposition).ok()
    }
    pub unsafe fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).currentPositionString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcurrentposition)).ok()
    }
    pub unsafe fn next(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).next)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn previous(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).previous)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentItem(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).currentItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentItem<'a, P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).SetcurrentItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn currentMarker(&self, plmarker: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).currentMarker)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plmarker)).ok()
    }
    pub unsafe fn SetcurrentMarker(&self, lmarker: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetcurrentMarker)(::windows::core::Vtable::as_raw(self), lmarker).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playItem<'a, P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).playItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPControls, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPControls {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPControls").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPControls {
    type Vtable = IWMPControls_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPControls {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74c09e02_f828_11d2_a74b_00a0c905f36e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_isAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT,
    pub play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub fastForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub fastReverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub currentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcurrentposition: *mut f64) -> ::windows::core::HRESULT,
    pub SetcurrentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dcurrentposition: f64) -> ::windows::core::HRESULT,
    pub currentPositionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcurrentposition: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub previous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub currentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwmpmedia: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    currentItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetcurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetcurrentItem: usize,
    pub currentMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmarker: *mut i32) -> ::windows::core::HRESULT,
    pub SetcurrentMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmarker: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub playItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    playItem: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPControls2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls2 {
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), ::core::mem::transmute(pisavailable)).ok()
    }
    pub unsafe fn play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.play)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastForward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.fastForward)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastReverse(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.fastReverse)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.currentPosition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdcurrentposition)).ok()
    }
    pub unsafe fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentPosition)(::windows::core::Vtable::as_raw(self), dcurrentposition).ok()
    }
    pub unsafe fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.currentPositionString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcurrentposition)).ok()
    }
    pub unsafe fn next(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.next)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn previous(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.previous)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentItem(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentItem<'a, P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn currentMarker(&self, plmarker: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.currentMarker)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plmarker)).ok()
    }
    pub unsafe fn SetcurrentMarker(&self, lmarker: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentMarker)(::windows::core::Vtable::as_raw(self), lmarker).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playItem<'a, P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.playItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn step(&self, lstep: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).step)(::windows::core::Vtable::as_raw(self), lstep).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPControls2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPControls);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPControls2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPControls2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPControls2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPControls2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPControls2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPControls2 {
    type Vtable = IWMPControls2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPControls2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f030d25_0890_480f_9775_1f7e40ab5b8e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls2_Vtbl {
    pub base__: IWMPControls_Vtbl,
    pub step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstep: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPControls3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls3 {
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), ::core::mem::transmute(pisavailable)).ok()
    }
    pub unsafe fn play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.play)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastForward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.fastForward)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn fastReverse(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.fastReverse)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.currentPosition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdcurrentposition)).ok()
    }
    pub unsafe fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentPosition)(::windows::core::Vtable::as_raw(self), dcurrentposition).ok()
    }
    pub unsafe fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.currentPositionString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcurrentposition)).ok()
    }
    pub unsafe fn next(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.next)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn previous(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.previous)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentItem(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentItem<'a, P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn currentMarker(&self, plmarker: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.currentMarker)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plmarker)).ok()
    }
    pub unsafe fn SetcurrentMarker(&self, lmarker: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentMarker)(::windows::core::Vtable::as_raw(self), lmarker).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playItem<'a, P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.playItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn step(&self, lstep: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.step)(::windows::core::Vtable::as_raw(self), lstep).ok()
    }
    pub unsafe fn audioLanguageCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).audioLanguageCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn getAudioLanguageID(&self, lindex: i32, pllangid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getAudioLanguageID)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pllangid)).ok()
    }
    pub unsafe fn getAudioLanguageDescription(&self, lindex: i32, pbstrlangdesc: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getAudioLanguageDescription)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstrlangdesc)).ok()
    }
    pub unsafe fn currentAudioLanguage(&self, pllangid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).currentAudioLanguage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pllangid)).ok()
    }
    pub unsafe fn SetcurrentAudioLanguage(&self, llangid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetcurrentAudioLanguage)(::windows::core::Vtable::as_raw(self), llangid).ok()
    }
    pub unsafe fn currentAudioLanguageIndex(&self, plindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).currentAudioLanguageIndex)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plindex)).ok()
    }
    pub unsafe fn SetcurrentAudioLanguageIndex(&self, lindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetcurrentAudioLanguageIndex)(::windows::core::Vtable::as_raw(self), lindex).ok()
    }
    pub unsafe fn getLanguageName(&self, llangid: i32, pbstrlangname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getLanguageName)(::windows::core::Vtable::as_raw(self), llangid, ::core::mem::transmute(pbstrlangname)).ok()
    }
    pub unsafe fn currentPositionTimecode(&self, bstrtimecode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).currentPositionTimecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(bstrtimecode)).ok()
    }
    pub unsafe fn SetcurrentPositionTimecode(&self, bstrtimecode: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetcurrentPositionTimecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtimecode)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPControls3, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPControls, IWMPControls2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPControls3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPControls3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPControls3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPControls3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPControls3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPControls3 {
    type Vtable = IWMPControls3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPControls3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1d1110e_d545_476a_9a78_ac3e4cb1e6bd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls3_Vtbl {
    pub base__: IWMPControls2_Vtbl,
    pub audioLanguageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub getAudioLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT,
    pub getAudioLanguageDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub currentAudioLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT,
    pub SetcurrentAudioLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llangid: i32) -> ::windows::core::HRESULT,
    pub currentAudioLanguageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT,
    pub SetcurrentAudioLanguageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT,
    pub getLanguageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llangid: i32, pbstrlangname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub currentPositionTimecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtimecode: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetcurrentPositionTimecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtimecode: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPConvert(::windows::core::IUnknown);
impl IWMPConvert {
    pub unsafe fn ConvertFile(&self, bstrinputfile: &::windows::core::BSTR, bstrdestinationfolder: &::windows::core::BSTR, pbstroutputfile: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ConvertFile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrinputfile), ::core::mem::transmute_copy(bstrdestinationfolder), ::core::mem::transmute(pbstroutputfile)).ok()
    }
    pub unsafe fn GetErrorURL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetErrorURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPConvert, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPConvert {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPConvert {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPConvert {}
impl ::core::fmt::Debug for IWMPConvert {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPConvert").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPConvert {
    type Vtable = IWMPConvert_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPConvert {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd683162f_57d4_4108_8373_4a9676d1c2e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPConvert_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConvertFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinputfile: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrdestinationfolder: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstroutputfile: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetErrorURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPCore(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).openState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpos)).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).playState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpps)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).controls)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPControls>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).settings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).currentMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<'a, P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).playlistCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistCollection>(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).network)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPNetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).currentPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<'a, P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).cdromCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdromCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).closedCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPClosedCaption>(result__)
    }
    pub unsafe fn isOnline(&self, pfonline: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).isOnline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfonline)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPError>(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPCore, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPCore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCore {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCore").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPCore {
    type Vtable = IWMPCore_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPCore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd84cca99_cce2_11d2_9ecc_0000f8085981);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub URL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub openState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpos: *mut WMPOpenState) -> ::windows::core::HRESULT,
    pub playState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpps: *mut WMPPlayState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub controls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    controls: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub currentMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmedia: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    currentMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetcurrentMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetcurrentMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub mediaCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmediacollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    mediaCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub playlistCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplaylistcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    playlistCollection: usize,
    pub versionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrversioninfo: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub launchURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub network: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqni: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    network: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub currentPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    currentPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetcurrentPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetcurrentPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub cdromCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcdromcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    cdromCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub closedCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclosedcaption: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    closedCaption: usize,
    pub isOnline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfonline: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperror: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    error: usize,
    pub status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatus: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPCore2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore2 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.openState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpos)).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.playState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpps)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.controls)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPControls>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.settings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<'a, P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.playlistCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistCollection>(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.network)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPNetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<'a, P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.cdromCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdromCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.closedCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPClosedCaption>(result__)
    }
    pub unsafe fn isOnline(&self, pfonline: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.isOnline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfonline)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPError>(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows::core::Result<IWMPDVD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).dvd)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPDVD>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPCore2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPCore);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPCore2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCore2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCore2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCore2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCore2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPCore2 {
    type Vtable = IWMPCore2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPCore2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc17e5b7_7561_4c18_bb90_17d485775659);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore2_Vtbl {
    pub base__: IWMPCore_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub dvd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdvd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    dvd: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPCore3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore3 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.openState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpos)).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.playState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpps)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.controls)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPControls>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.settings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<'a, P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.playlistCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistCollection>(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.network)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPNetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<'a, P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.cdromCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdromCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.closedCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPClosedCaption>(result__)
    }
    pub unsafe fn isOnline(&self, pfonline: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.isOnline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfonline)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPError>(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows::core::Result<IWMPDVD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.dvd)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPDVD>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newPlaylist(&self, bstrname: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).newPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newMedia(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).newMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPCore3, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPCore, IWMPCore2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPCore3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPCore3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPCore3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPCore3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPCore3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPCore3 {
    type Vtable = IWMPCore3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPCore3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7587c667_628f_499f_88e7_6a6f4e888464);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore3_Vtbl {
    pub base__: IWMPCore2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub newPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    newPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub newMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppmedia: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    newMedia: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPDVD(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDVD {
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), ::core::mem::transmute(pisavailable)).ok()
    }
    pub unsafe fn domain(&self, strdomain: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).domain)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(strdomain)).ok()
    }
    pub unsafe fn topMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).topMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn titleMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).titleMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn back(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).back)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPDVD, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPDVD {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDVD {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDVD {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDVD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDVD").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPDVD {
    type Vtable = IWMPDVD_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPDVD {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8da61686_4668_4a5c_ae5d_803193293dbe);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDVD_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_isAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT,
    pub domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdomain: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub topMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub titleMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub back: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPDownloadCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadCollection {
    pub unsafe fn id(&self, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).id)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plid)).ok()
    }
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn item(&self, litem: i32) -> ::windows::core::Result<IWMPDownloadItem2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).item)(::windows::core::Vtable::as_raw(self), litem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPDownloadItem2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn startDownload(&self, bstrsourceurl: &::windows::core::BSTR, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<IWMPDownloadItem2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).startDownload)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsourceurl), ::core::mem::transmute_copy(bstrtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPDownloadItem2>(result__)
    }
    pub unsafe fn removeItem(&self, litem: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).removeItem)(::windows::core::Vtable::as_raw(self), litem).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPDownloadCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPDownloadCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDownloadCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDownloadCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDownloadCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDownloadCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPDownloadCollection {
    type Vtable = IWMPDownloadCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPDownloadCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a319c7f_85f9_436c_b88e_82fd88000e1c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDownloadCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, litem: i32, ppdownload: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub startDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsourceurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppdownload: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    startDownload: usize,
    pub removeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, litem: i32) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPDownloadItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItem {
    pub unsafe fn sourceURL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).sourceURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn size(&self, plsize: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).size)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plsize)).ok()
    }
    pub unsafe fn r#type(&self, pbstrtype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).r#type)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrtype)).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).progress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plprogress)).ok()
    }
    pub unsafe fn downloadState(&self, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).downloadState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpsdls)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPDownloadItem, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPDownloadItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDownloadItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDownloadItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDownloadItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDownloadItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPDownloadItem {
    type Vtable = IWMPDownloadItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPDownloadItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9470e8e_3f6b_46a9_a0a9_452815c34297);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDownloadItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub sourceURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT,
    pub r#type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT,
    pub downloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::HRESULT,
    pub pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPDownloadItem2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItem2 {
    pub unsafe fn sourceURL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.sourceURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn size(&self, plsize: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.size)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plsize)).ok()
    }
    pub unsafe fn r#type(&self, pbstrtype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.r#type)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrtype)).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.progress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plprogress)).ok()
    }
    pub unsafe fn downloadState(&self, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.downloadState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpsdls)).ok()
    }
    pub unsafe fn pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPDownloadItem2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPDownloadItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPDownloadItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDownloadItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDownloadItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDownloadItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDownloadItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPDownloadItem2 {
    type Vtable = IWMPDownloadItem2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPDownloadItem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fbb3336_6da3_479d_b8ff_67d46e20a987);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDownloadItem2_Vtbl {
    pub base__: IWMPDownloadItem_Vtbl,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPDownloadManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getDownloadCollection(&self, lcollectionid: i32) -> ::windows::core::Result<IWMPDownloadCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getDownloadCollection)(::windows::core::Vtable::as_raw(self), lcollectionid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPDownloadCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createDownloadCollection(&self) -> ::windows::core::Result<IWMPDownloadCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).createDownloadCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPDownloadCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPDownloadManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPDownloadManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPDownloadManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPDownloadManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPDownloadManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPDownloadManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPDownloadManager {
    type Vtable = IWMPDownloadManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPDownloadManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe15e9ad1_8f20_4cc4_9ec7_1a328ca86a0d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDownloadManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub getDownloadCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcollectionid: i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getDownloadCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createDownloadCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createDownloadCollection: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPEffects(::windows::core::IUnknown);
impl IWMPEffects {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Render<'a, P0>(&self, plevels: *mut TimedLevel, hdc: P0, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).Render)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plevels), hdc.into(), ::core::mem::transmute(prc)).ok()
    }
    pub unsafe fn MediaInfo(&self, lchannelcount: i32, lsamplerate: i32, bstrtitle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MediaInfo)(::windows::core::Vtable::as_raw(self), lchannelcount, lsamplerate, ::core::mem::transmute_copy(bstrtitle)).ok()
    }
    pub unsafe fn GetCapabilities(&self, pdwcapabilities: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCapabilities)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwcapabilities)).ok()
    }
    pub unsafe fn GetTitle(&self, bstrtitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTitle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(bstrtitle)).ok()
    }
    pub unsafe fn GetPresetTitle(&self, npreset: i32, bstrpresettitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPresetTitle)(::windows::core::Vtable::as_raw(self), npreset, ::core::mem::transmute(bstrpresettitle)).ok()
    }
    pub unsafe fn GetPresetCount(&self, pnpresetcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPresetCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pnpresetcount)).ok()
    }
    pub unsafe fn SetCurrentPreset(&self, npreset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCurrentPreset)(::windows::core::Vtable::as_raw(self), npreset).ok()
    }
    pub unsafe fn GetCurrentPreset(&self, pnpreset: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCurrentPreset)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pnpreset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayPropertyPage<'a, P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DisplayPropertyPage)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GoFullscreen<'a, P0>(&self, ffullscreen: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).GoFullscreen)(::windows::core::Vtable::as_raw(self), ffullscreen.into()).ok()
    }
    pub unsafe fn RenderFullScreen(&self, plevels: *mut TimedLevel) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RenderFullScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plevels)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPEffects, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPEffects {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPEffects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEffects {}
impl ::core::fmt::Debug for IWMPEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEffects").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPEffects {
    type Vtable = IWMPEffects_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPEffects {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3984c13_c3cb_48e2_8be5_5168340b4f35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEffects_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Render: usize,
    pub MediaInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtitle: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetPresetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npreset: i32, bstrpresettitle: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetPresetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpresetcount: *mut i32) -> ::windows::core::HRESULT,
    pub SetCurrentPreset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npreset: i32) -> ::windows::core::HRESULT,
    pub GetCurrentPreset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpreset: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayPropertyPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayPropertyPage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GoFullscreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GoFullscreen: usize,
    pub RenderFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPEffects2(::windows::core::IUnknown);
impl IWMPEffects2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Render<'a, P0>(&self, plevels: *mut TimedLevel, hdc: P0, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.Render)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plevels), hdc.into(), ::core::mem::transmute(prc)).ok()
    }
    pub unsafe fn MediaInfo(&self, lchannelcount: i32, lsamplerate: i32, bstrtitle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MediaInfo)(::windows::core::Vtable::as_raw(self), lchannelcount, lsamplerate, ::core::mem::transmute_copy(bstrtitle)).ok()
    }
    pub unsafe fn GetCapabilities(&self, pdwcapabilities: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwcapabilities)).ok()
    }
    pub unsafe fn GetTitle(&self, bstrtitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTitle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(bstrtitle)).ok()
    }
    pub unsafe fn GetPresetTitle(&self, npreset: i32, bstrpresettitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPresetTitle)(::windows::core::Vtable::as_raw(self), npreset, ::core::mem::transmute(bstrpresettitle)).ok()
    }
    pub unsafe fn GetPresetCount(&self, pnpresetcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPresetCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pnpresetcount)).ok()
    }
    pub unsafe fn SetCurrentPreset(&self, npreset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentPreset)(::windows::core::Vtable::as_raw(self), npreset).ok()
    }
    pub unsafe fn GetCurrentPreset(&self, pnpreset: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPreset)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pnpreset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayPropertyPage<'a, P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.DisplayPropertyPage)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GoFullscreen<'a, P0>(&self, ffullscreen: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GoFullscreen)(::windows::core::Vtable::as_raw(self), ffullscreen.into()).ok()
    }
    pub unsafe fn RenderFullScreen(&self, plevels: *mut TimedLevel) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RenderFullScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plevels)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCore<'a, P0>(&self, pplayer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCore>>,
    {
        (::windows::core::Vtable::vtable(self).SetCore)(::windows::core::Vtable::as_raw(self), pplayer.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, P0>(&self, hwndparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).Create)(::windows::core::Vtable::as_raw(self), hwndparent.into()).ok()
    }
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Destroy)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NotifyNewMedia<'a, P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).NotifyNewMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessage<'a, P0, P1>(&self, msg: u32, wparam: P0, lparam: P1, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).OnWindowMessage)(::windows::core::Vtable::as_raw(self), msg, wparam.into(), lparam.into(), ::core::mem::transmute(plresultparam)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenderWindowed<'a, P0>(&self, pdata: *mut TimedLevel, frequiredrender: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).RenderWindowed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata), frequiredrender.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPEffects2, ::windows::core::IUnknown, IWMPEffects);
impl ::core::clone::Clone for IWMPEffects2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEffects2 {}
impl ::core::fmt::Debug for IWMPEffects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEffects2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPEffects2 {
    type Vtable = IWMPEffects2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPEffects2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x695386ec_aa3c_4618_a5e1_dd9a8b987632);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEffects2_Vtbl {
    pub base__: IWMPEffects_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NotifyNewMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NotifyNewMedia: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RenderWindowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenderWindowed: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPError(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPError {
    pub unsafe fn clearErrorQueue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).clearErrorQueue)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn errorCount(&self, plnumerrors: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).errorCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plnumerrors)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, dwindex: i32) -> ::windows::core::Result<IWMPErrorItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_item)(::windows::core::Vtable::as_raw(self), dwindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPErrorItem>(result__)
    }
    pub unsafe fn webHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).webHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPError, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPError {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPError").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPError {
    type Vtable = IWMPError_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa12dcf7d_14ab_4c1b_a8cd_63909f06025b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPError_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub clearErrorQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub errorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plnumerrors: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: i32, pperroritem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_item: usize,
    pub webHelp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPErrorItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItem {
    pub unsafe fn errorCode(&self, phr: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).errorCode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(phr)).ok()
    }
    pub unsafe fn errorDescription(&self, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).errorDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn errorContext(&self, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).errorContext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvarcontext)).ok()
    }
    pub unsafe fn remedy(&self, plremedy: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).remedy)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plremedy)).ok()
    }
    pub unsafe fn customUrl(&self, pbstrcustomurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).customUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcustomurl)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPErrorItem, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPErrorItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPErrorItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPErrorItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPErrorItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPErrorItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPErrorItem {
    type Vtable = IWMPErrorItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPErrorItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3614c646_3b3b_4de7_a81e_930e3f2127b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPErrorItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub errorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phr: *mut i32) -> ::windows::core::HRESULT,
    pub errorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub errorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcontext: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    errorContext: usize,
    pub remedy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plremedy: *mut i32) -> ::windows::core::HRESULT,
    pub customUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcustomurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPErrorItem2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItem2 {
    pub unsafe fn errorCode(&self, phr: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.errorCode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(phr)).ok()
    }
    pub unsafe fn errorDescription(&self, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.errorDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn errorContext(&self, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.errorContext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvarcontext)).ok()
    }
    pub unsafe fn remedy(&self, plremedy: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.remedy)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plremedy)).ok()
    }
    pub unsafe fn customUrl(&self, pbstrcustomurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.customUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrcustomurl)).ok()
    }
    pub unsafe fn condition(&self, plcondition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).condition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcondition)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPErrorItem2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPErrorItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPErrorItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPErrorItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPErrorItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPErrorItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPErrorItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPErrorItem2 {
    type Vtable = IWMPErrorItem2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPErrorItem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf75ccec0_c67c_475c_931e_8719870bee7d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPErrorItem2_Vtbl {
    pub base__: IWMPErrorItem_Vtbl,
    pub condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcondition: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPEvents(::windows::core::IUnknown);
impl IWMPEvents {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).OpenStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).PlayStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows::core::Vtable::vtable(self).AudioLanguageChange)(::windows::core::Vtable::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows::core::Vtable::vtable(self).StatusChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ScriptCommand(&self, sctype: &::windows::core::BSTR, param: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).ScriptCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sctype), ::core::mem::transmute_copy(param))
    }
    pub unsafe fn NewStream(&self) {
        (::windows::core::Vtable::vtable(self).NewStream)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).Disconnect)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn Buffering(&self, start: i16) {
        (::windows::core::Vtable::vtable(self).Buffering)(::windows::core::Vtable::as_raw(self), start)
    }
    pub unsafe fn Error(&self) {
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Warning(&self, warningtype: i32, param: i32, description: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).Warning)(::windows::core::Vtable::as_raw(self), warningtype, param, ::core::mem::transmute_copy(description))
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).EndOfStream)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows::core::Vtable::vtable(self).PositionChange)(::windows::core::Vtable::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows::core::Vtable::vtable(self).MarkerHit)(::windows::core::Vtable::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows::core::Vtable::vtable(self).DurationUnitChange)(::windows::core::Vtable::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows::core::Vtable::vtable(self).CdromMediaChange)(::windows::core::Vtable::as_raw(self), cdromnum)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<'a, P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).PlaylistChange)(::windows::core::Vtable::as_raw(self), playlist.into().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows::core::Vtable::vtable(self).CurrentPlaylistChange)(::windows::core::Vtable::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).CurrentPlaylistItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<'a, P0>(&self, item: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).MediaChange)(::windows::core::Vtable::as_raw(self), item.into().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).CurrentMediaItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<'a, P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).CurrentItemChange)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).MediaCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).MediaCollectionAttributeStringAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).MediaCollectionAttributeStringRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &::windows::core::BSTR, bstroldattribval: &::windows::core::BSTR, bstrnewattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).MediaCollectionAttributeStringChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstroldattribval), ::core::mem::transmute_copy(bstrnewattribval))
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).PlaylistCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).PlaylistCollectionPlaylistAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).PlaylistCollectionPlaylistRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted(&self, bstrplaylistname: &::windows::core::BSTR, varfisdeleted: i16) {
        (::windows::core::Vtable::vtable(self).PlaylistCollectionPlaylistSetAsDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname), varfisdeleted)
    }
    pub unsafe fn ModeChange(&self, modename: &::windows::core::BSTR, newvalue: i16) {
        (::windows::core::Vtable::vtable(self).ModeChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modename), newvalue)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<'a, P0>(&self, pmediaobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).MediaError)(::windows::core::Vtable::as_raw(self), pmediaobject.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<'a, P0>(&self, pitem: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).OpenPlaylistSwitch)(::windows::core::Vtable::as_raw(self), pitem.into().abi())
    }
    pub unsafe fn DomainChange(&self, strdomain: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).DomainChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strdomain))
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows::core::Vtable::vtable(self).SwitchedToPlayerApplication)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows::core::Vtable::vtable(self).SwitchedToControl)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows::core::Vtable::vtable(self).PlayerDockedStateChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows::core::Vtable::vtable(self).PlayerReconnect)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).Click)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).DoubleClick)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).KeyDown)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows::core::Vtable::vtable(self).KeyPress)(::windows::core::Vtable::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).KeyUp)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).MouseDown)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).MouseMove)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).MouseUp)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
}
::windows::core::interface_hierarchy!(IWMPEvents, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEvents {}
impl ::core::fmt::Debug for IWMPEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPEvents {
    type Vtable = IWMPEvents_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19a6627b_da9e_47c1_bb23_00b5e668236a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OpenStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: i32),
    pub PlayStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: i32),
    pub AudioLanguageChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: i32),
    pub StatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub ScriptCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sctype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, param: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    pub NewStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: i32),
    pub Buffering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: i16),
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Warning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, warningtype: i32, param: i32, description: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    pub EndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: i32),
    pub PositionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldposition: f64, newposition: f64),
    pub MarkerHit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, markernum: i32),
    pub DurationUnitChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newdurationunit: i32),
    pub CdromMediaChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cdromnum: i32),
    #[cfg(feature = "Win32_System_Com")]
    pub PlaylistChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, playlist: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType),
    #[cfg(not(feature = "Win32_System_Com"))]
    PlaylistChange: usize,
    pub CurrentPlaylistChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType),
    pub CurrentPlaylistItemAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    #[cfg(feature = "Win32_System_Com")]
    pub MediaChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    MediaChange: usize,
    pub CurrentMediaItemAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentItemChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentItemChange: usize,
    pub MediaCollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub MediaCollectionAttributeStringAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    pub MediaCollectionAttributeStringRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    pub MediaCollectionAttributeStringChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstroldattribval: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrnewattribval: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    pub PlaylistCollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PlaylistCollectionPlaylistAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    pub PlaylistCollectionPlaylistRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    pub PlaylistCollectionPlaylistSetAsDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, varfisdeleted: i16),
    pub ModeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modename: ::core::mem::ManuallyDrop<::windows::core::BSTR>, newvalue: i16),
    #[cfg(feature = "Win32_System_Com")]
    pub MediaError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediaobject: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    MediaError: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenPlaylistSwitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenPlaylistSwitch: usize,
    pub DomainChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdomain: ::core::mem::ManuallyDrop<::windows::core::BSTR>),
    pub SwitchedToPlayerApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub SwitchedToControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PlayerDockedStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PlayerReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Click: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
    pub DoubleClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
    pub KeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16),
    pub KeyPress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nkeyascii: i16),
    pub KeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16),
    pub MouseDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
    pub MouseMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
    pub MouseUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32),
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPEvents2(::windows::core::IUnknown);
impl IWMPEvents2 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.OpenStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.PlayStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows::core::Vtable::vtable(self).base__.AudioLanguageChange)(::windows::core::Vtable::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.StatusChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ScriptCommand(&self, sctype: &::windows::core::BSTR, param: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.ScriptCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sctype), ::core::mem::transmute_copy(param))
    }
    pub unsafe fn NewStream(&self) {
        (::windows::core::Vtable::vtable(self).base__.NewStream)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.Disconnect)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn Buffering(&self, start: i16) {
        (::windows::core::Vtable::vtable(self).base__.Buffering)(::windows::core::Vtable::as_raw(self), start)
    }
    pub unsafe fn Error(&self) {
        (::windows::core::Vtable::vtable(self).base__.Error)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Warning(&self, warningtype: i32, param: i32, description: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.Warning)(::windows::core::Vtable::as_raw(self), warningtype, param, ::core::mem::transmute_copy(description))
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.EndOfStream)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows::core::Vtable::vtable(self).base__.PositionChange)(::windows::core::Vtable::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows::core::Vtable::vtable(self).base__.MarkerHit)(::windows::core::Vtable::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows::core::Vtable::vtable(self).base__.DurationUnitChange)(::windows::core::Vtable::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows::core::Vtable::vtable(self).base__.CdromMediaChange)(::windows::core::Vtable::as_raw(self), cdromnum)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<'a, P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PlaylistChange)(::windows::core::Vtable::as_raw(self), playlist.into().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows::core::Vtable::vtable(self).base__.CurrentPlaylistChange)(::windows::core::Vtable::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.CurrentPlaylistItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<'a, P0>(&self, item: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MediaChange)(::windows::core::Vtable::as_raw(self), item.into().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.CurrentMediaItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<'a, P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CurrentItemChange)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionAttributeStringAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionAttributeStringRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &::windows::core::BSTR, bstroldattribval: &::windows::core::BSTR, bstrnewattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionAttributeStringChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstroldattribval), ::core::mem::transmute_copy(bstrnewattribval))
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.PlaylistCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.PlaylistCollectionPlaylistAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.PlaylistCollectionPlaylistRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted(&self, bstrplaylistname: &::windows::core::BSTR, varfisdeleted: i16) {
        (::windows::core::Vtable::vtable(self).base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname), varfisdeleted)
    }
    pub unsafe fn ModeChange(&self, modename: &::windows::core::BSTR, newvalue: i16) {
        (::windows::core::Vtable::vtable(self).base__.ModeChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modename), newvalue)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<'a, P0>(&self, pmediaobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MediaError)(::windows::core::Vtable::as_raw(self), pmediaobject.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<'a, P0>(&self, pitem: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OpenPlaylistSwitch)(::windows::core::Vtable::as_raw(self), pitem.into().abi())
    }
    pub unsafe fn DomainChange(&self, strdomain: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.DomainChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strdomain))
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows::core::Vtable::vtable(self).base__.SwitchedToPlayerApplication)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows::core::Vtable::vtable(self).base__.SwitchedToControl)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.PlayerDockedStateChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows::core::Vtable::vtable(self).base__.PlayerReconnect)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.Click)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.DoubleClick)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.KeyDown)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows::core::Vtable::vtable(self).base__.KeyPress)(::windows::core::Vtable::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.KeyUp)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.MouseDown)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.MouseMove)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.MouseUp)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DeviceConnect<'a, P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceConnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceDisconnect<'a, P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceDisconnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceStatusChange<'a, P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceStatusChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstatus)
    }
    pub unsafe fn DeviceSyncStateChange<'a, P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceSyncStateChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstate)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSyncError<'a, P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceSyncError)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CreatePartnershipComplete<'a, P0>(&self, pdevice: P0, hrresult: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).CreatePartnershipComplete)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hrresult)
    }
}
::windows::core::interface_hierarchy!(IWMPEvents2, ::windows::core::IUnknown, IWMPEvents);
impl ::core::clone::Clone for IWMPEvents2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPEvents2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEvents2 {}
impl ::core::fmt::Debug for IWMPEvents2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEvents2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPEvents2 {
    type Vtable = IWMPEvents2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPEvents2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e7601fa_47ea_4107_9ea9_9004ed9684ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents2_Vtbl {
    pub base__: IWMPEvents_Vtbl,
    pub DeviceConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void),
    pub DeviceDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void),
    pub DeviceStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, newstatus: WMPDeviceStatus),
    pub DeviceSyncStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, newstate: WMPSyncState),
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceSyncError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceSyncError: usize,
    pub CreatePartnershipComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT),
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPEvents3(::windows::core::IUnknown);
impl IWMPEvents3 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlayStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.AudioLanguageChange)(::windows::core::Vtable::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.StatusChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ScriptCommand(&self, sctype: &::windows::core::BSTR, param: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.ScriptCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sctype), ::core::mem::transmute_copy(param))
    }
    pub unsafe fn NewStream(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.NewStream)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.Disconnect)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn Buffering(&self, start: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.Buffering)(::windows::core::Vtable::as_raw(self), start)
    }
    pub unsafe fn Error(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.Error)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Warning(&self, warningtype: i32, param: i32, description: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.Warning)(::windows::core::Vtable::as_raw(self), warningtype, param, ::core::mem::transmute_copy(description))
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.EndOfStream)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows::core::Vtable::vtable(self).base__.base__.PositionChange)(::windows::core::Vtable::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.MarkerHit)(::windows::core::Vtable::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DurationUnitChange)(::windows::core::Vtable::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.CdromMediaChange)(::windows::core::Vtable::as_raw(self), cdromnum)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<'a, P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistChange)(::windows::core::Vtable::as_raw(self), playlist.into().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentPlaylistChange)(::windows::core::Vtable::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentPlaylistItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<'a, P0>(&self, item: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaChange)(::windows::core::Vtable::as_raw(self), item.into().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentMediaItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<'a, P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentItemChange)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaCollectionAttributeStringAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaCollectionAttributeStringRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &::windows::core::BSTR, bstroldattribval: &::windows::core::BSTR, bstrnewattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaCollectionAttributeStringChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstroldattribval), ::core::mem::transmute_copy(bstrnewattribval))
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistCollectionPlaylistAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistCollectionPlaylistRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted(&self, bstrplaylistname: &::windows::core::BSTR, varfisdeleted: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname), varfisdeleted)
    }
    pub unsafe fn ModeChange(&self, modename: &::windows::core::BSTR, newvalue: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.ModeChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modename), newvalue)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<'a, P0>(&self, pmediaobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MediaError)(::windows::core::Vtable::as_raw(self), pmediaobject.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<'a, P0>(&self, pitem: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenPlaylistSwitch)(::windows::core::Vtable::as_raw(self), pitem.into().abi())
    }
    pub unsafe fn DomainChange(&self, strdomain: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.DomainChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strdomain))
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.SwitchedToPlayerApplication)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.SwitchedToControl)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlayerDockedStateChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PlayerReconnect)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.Click)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DoubleClick)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.KeyDown)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.KeyPress)(::windows::core::Vtable::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.KeyUp)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.MouseDown)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.MouseMove)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.MouseUp)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DeviceConnect<'a, P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceConnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceDisconnect<'a, P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceDisconnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceStatusChange<'a, P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceStatusChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstatus)
    }
    pub unsafe fn DeviceSyncStateChange<'a, P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceSyncStateChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstate)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSyncError<'a, P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceSyncError)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CreatePartnershipComplete<'a, P0>(&self, pdevice: P0, hrresult: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreatePartnershipComplete)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hrresult)
    }
    pub unsafe fn CdromRipStateChange<'a, P0>(&self, pcdromrip: P0, wmprs: WMPRipState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromRip>>,
    {
        (::windows::core::Vtable::vtable(self).CdromRipStateChange)(::windows::core::Vtable::as_raw(self), pcdromrip.into().abi(), wmprs)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromRipMediaError<'a, P0, P1>(&self, pcdromrip: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromRip>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).CdromRipMediaError)(::windows::core::Vtable::as_raw(self), pcdromrip.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CdromBurnStateChange<'a, P0>(&self, pcdromburn: P0, wmpbs: WMPBurnState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromBurn>>,
    {
        (::windows::core::Vtable::vtable(self).CdromBurnStateChange)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), wmpbs)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromBurnMediaError<'a, P0, P1>(&self, pcdromburn: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromBurn>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).CdromBurnMediaError)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CdromBurnError<'a, P0>(&self, pcdromburn: P0, hrerror: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromBurn>>,
    {
        (::windows::core::Vtable::vtable(self).CdromBurnError)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), hrerror)
    }
    pub unsafe fn LibraryConnect<'a, P0>(&self, plibrary: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).LibraryConnect)(::windows::core::Vtable::as_raw(self), plibrary.into().abi())
    }
    pub unsafe fn LibraryDisconnect<'a, P0>(&self, plibrary: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).LibraryDisconnect)(::windows::core::Vtable::as_raw(self), plibrary.into().abi())
    }
    pub unsafe fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState) {
        (::windows::core::Vtable::vtable(self).FolderScanStateChange)(::windows::core::Vtable::as_raw(self), wmpfss)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StringCollectionChange<'a, P0>(&self, pdispstringcollection: P0, change: WMPStringCollectionChangeEventType, lcollectionindex: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).StringCollectionChange)(::windows::core::Vtable::as_raw(self), pdispstringcollection.into().abi(), change, lcollectionindex)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaAdded<'a, P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).MediaCollectionMediaAdded)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaRemoved<'a, P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).MediaCollectionMediaRemoved)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
}
::windows::core::interface_hierarchy!(IWMPEvents3, ::windows::core::IUnknown, IWMPEvents, IWMPEvents2);
impl ::core::clone::Clone for IWMPEvents3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPEvents3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEvents3 {}
impl ::core::fmt::Debug for IWMPEvents3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEvents3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPEvents3 {
    type Vtable = IWMPEvents3_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPEvents3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f504270_a66b_4223_8e96_26a06c63d69f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents3_Vtbl {
    pub base__: IWMPEvents2_Vtbl,
    pub CdromRipStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromrip: *mut ::core::ffi::c_void, wmprs: WMPRipState),
    #[cfg(feature = "Win32_System_Com")]
    pub CdromRipMediaError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromrip: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    CdromRipMediaError: usize,
    pub CdromBurnStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, wmpbs: WMPBurnState),
    #[cfg(feature = "Win32_System_Com")]
    pub CdromBurnMediaError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    CdromBurnMediaError: usize,
    pub CdromBurnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT),
    pub LibraryConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibrary: *mut ::core::ffi::c_void),
    pub LibraryDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibrary: *mut ::core::ffi::c_void),
    pub FolderScanStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmpfss: WMPFolderScanState),
    #[cfg(feature = "Win32_System_Com")]
    pub StringCollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispstringcollection: *mut ::core::ffi::c_void, change: WMPStringCollectionChangeEventType, lcollectionindex: i32),
    #[cfg(not(feature = "Win32_System_Com"))]
    StringCollectionChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MediaCollectionMediaAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    MediaCollectionMediaAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MediaCollectionMediaRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_System_Com"))]
    MediaCollectionMediaRemoved: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPEvents4(::windows::core::IUnknown);
impl IWMPEvents4 {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlayStateChange)(::windows::core::Vtable::as_raw(self), newstate)
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AudioLanguageChange)(::windows::core::Vtable::as_raw(self), langid)
    }
    pub unsafe fn StatusChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StatusChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ScriptCommand(&self, sctype: &::windows::core::BSTR, param: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ScriptCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sctype), ::core::mem::transmute_copy(param))
    }
    pub unsafe fn NewStream(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NewStream)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Disconnect)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn Buffering(&self, start: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Buffering)(::windows::core::Vtable::as_raw(self), start)
    }
    pub unsafe fn Error(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Error)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Warning(&self, warningtype: i32, param: i32, description: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Warning)(::windows::core::Vtable::as_raw(self), warningtype, param, ::core::mem::transmute_copy(description))
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EndOfStream)(::windows::core::Vtable::as_raw(self), result)
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PositionChange)(::windows::core::Vtable::as_raw(self), oldposition, newposition)
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MarkerHit)(::windows::core::Vtable::as_raw(self), markernum)
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DurationUnitChange)(::windows::core::Vtable::as_raw(self), newdurationunit)
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CdromMediaChange)(::windows::core::Vtable::as_raw(self), cdromnum)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PlaylistChange<'a, P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistChange)(::windows::core::Vtable::as_raw(self), playlist.into().abi(), change)
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentPlaylistChange)(::windows::core::Vtable::as_raw(self), change)
    }
    pub unsafe fn CurrentPlaylistItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentPlaylistItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaChange<'a, P0>(&self, item: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaChange)(::windows::core::Vtable::as_raw(self), item.into().abi())
    }
    pub unsafe fn CurrentMediaItemAvailable(&self, bstritemname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentMediaItemAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentItemChange<'a, P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentItemChange)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    pub unsafe fn MediaCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaCollectionAttributeStringAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaCollectionAttributeStringRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstrattribval))
    }
    pub unsafe fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &::windows::core::BSTR, bstroldattribval: &::windows::core::BSTR, bstrnewattribval: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaCollectionAttributeStringChanged)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribname), ::core::mem::transmute_copy(bstroldattribval), ::core::mem::transmute_copy(bstrnewattribval))
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistCollectionChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistAdded)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistRemoved)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname))
    }
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted(&self, bstrplaylistname: &::windows::core::BSTR, varfisdeleted: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlaylistCollectionPlaylistSetAsDeleted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplaylistname), varfisdeleted)
    }
    pub unsafe fn ModeChange(&self, modename: &::windows::core::BSTR, newvalue: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ModeChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modename), newvalue)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaError<'a, P0>(&self, pmediaobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MediaError)(::windows::core::Vtable::as_raw(self), pmediaobject.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenPlaylistSwitch<'a, P0>(&self, pitem: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenPlaylistSwitch)(::windows::core::Vtable::as_raw(self), pitem.into().abi())
    }
    pub unsafe fn DomainChange(&self, strdomain: &::windows::core::BSTR) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DomainChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strdomain))
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SwitchedToPlayerApplication)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SwitchedToControl(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SwitchedToControl)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlayerDockedStateChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PlayerReconnect(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PlayerReconnect)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Click)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DoubleClick)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.KeyDown)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.KeyPress)(::windows::core::Vtable::as_raw(self), nkeyascii)
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.KeyUp)(::windows::core::Vtable::as_raw(self), nkeycode, nshiftstate)
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MouseDown)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MouseMove)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MouseUp)(::windows::core::Vtable::as_raw(self), nbutton, nshiftstate, fx, fy)
    }
    pub unsafe fn DeviceConnect<'a, P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceConnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceDisconnect<'a, P0>(&self, pdevice: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceDisconnect)(::windows::core::Vtable::as_raw(self), pdevice.into().abi())
    }
    pub unsafe fn DeviceStatusChange<'a, P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceStatusChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstatus)
    }
    pub unsafe fn DeviceSyncStateChange<'a, P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceSyncStateChange)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), newstate)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSyncError<'a, P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeviceSyncError)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CreatePartnershipComplete<'a, P0>(&self, pdevice: P0, hrresult: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePartnershipComplete)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hrresult)
    }
    pub unsafe fn CdromRipStateChange<'a, P0>(&self, pcdromrip: P0, wmprs: WMPRipState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromRip>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromRipStateChange)(::windows::core::Vtable::as_raw(self), pcdromrip.into().abi(), wmprs)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromRipMediaError<'a, P0, P1>(&self, pcdromrip: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromRip>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromRipMediaError)(::windows::core::Vtable::as_raw(self), pcdromrip.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CdromBurnStateChange<'a, P0>(&self, pcdromburn: P0, wmpbs: WMPBurnState)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromBurn>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromBurnStateChange)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), wmpbs)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CdromBurnMediaError<'a, P0, P1>(&self, pcdromburn: P0, pmedia: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromBurn>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromBurnMediaError)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), pmedia.into().abi())
    }
    pub unsafe fn CdromBurnError<'a, P0>(&self, pcdromburn: P0, hrerror: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCdromBurn>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CdromBurnError)(::windows::core::Vtable::as_raw(self), pcdromburn.into().abi(), hrerror)
    }
    pub unsafe fn LibraryConnect<'a, P0>(&self, plibrary: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).base__.LibraryConnect)(::windows::core::Vtable::as_raw(self), plibrary.into().abi())
    }
    pub unsafe fn LibraryDisconnect<'a, P0>(&self, plibrary: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).base__.LibraryDisconnect)(::windows::core::Vtable::as_raw(self), plibrary.into().abi())
    }
    pub unsafe fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState) {
        (::windows::core::Vtable::vtable(self).base__.FolderScanStateChange)(::windows::core::Vtable::as_raw(self), wmpfss)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StringCollectionChange<'a, P0>(&self, pdispstringcollection: P0, change: WMPStringCollectionChangeEventType, lcollectionindex: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StringCollectionChange)(::windows::core::Vtable::as_raw(self), pdispstringcollection.into().abi(), change, lcollectionindex)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaAdded<'a, P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionMediaAdded)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MediaCollectionMediaRemoved<'a, P0>(&self, pdispmedia: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MediaCollectionMediaRemoved)(::windows::core::Vtable::as_raw(self), pdispmedia.into().abi())
    }
    pub unsafe fn DeviceEstimation<'a, P0>(&self, pdevice: P0, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceEstimation)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hrresult, qwestimatedusedspace, qwestimatedspace)
    }
}
::windows::core::interface_hierarchy!(IWMPEvents4, ::windows::core::IUnknown, IWMPEvents, IWMPEvents2, IWMPEvents3);
impl ::core::clone::Clone for IWMPEvents4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPEvents4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPEvents4 {}
impl ::core::fmt::Debug for IWMPEvents4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPEvents4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPEvents4 {
    type Vtable = IWMPEvents4_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPEvents4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26dabcfa_306b_404d_9a6f_630a8405048d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents4_Vtbl {
    pub base__: IWMPEvents3_Vtbl,
    pub DeviceEstimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64),
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPFolderMonitorServices(::windows::core::IUnknown);
impl IWMPFolderMonitorServices {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn item(&self, lindex: i32, pbstrfolder: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).item)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstrfolder)).ok()
    }
    pub unsafe fn add(&self, bstrfolder: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).add)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfolder)).ok()
    }
    pub unsafe fn remove(&self, lindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).remove)(::windows::core::Vtable::as_raw(self), lindex).ok()
    }
    pub unsafe fn scanState(&self, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).scanState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpfss)).ok()
    }
    pub unsafe fn currentFolder(&self, pbstrfolder: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).currentFolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrfolder)).ok()
    }
    pub unsafe fn scannedFilesCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).scannedFilesCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn addedFilesCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).addedFilesCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn updateProgress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).updateProgress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plprogress)).ok()
    }
    pub unsafe fn startScan(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).startScan)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stopScan(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stopScan)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPFolderMonitorServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPFolderMonitorServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPFolderMonitorServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPFolderMonitorServices {}
impl ::core::fmt::Debug for IWMPFolderMonitorServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPFolderMonitorServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPFolderMonitorServices {
    type Vtable = IWMPFolderMonitorServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPFolderMonitorServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x788c8743_e57f_439d_a468_5bc77f2e59c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPFolderMonitorServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstrfolder: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfolder: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT,
    pub scanState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::HRESULT,
    pub currentFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfolder: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub scannedFilesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub addedFilesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub updateProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT,
    pub startScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub stopScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPGraphCreation(::windows::core::IUnknown);
impl IWMPGraphCreation {
    pub unsafe fn GraphCreationPreRender<'a, P0, P1>(&self, pfiltergraph: P0, preserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).GraphCreationPreRender)(::windows::core::Vtable::as_raw(self), pfiltergraph.into().abi(), preserved.into().abi()).ok()
    }
    pub unsafe fn GraphCreationPostRender<'a, P0>(&self, pfiltergraph: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).GraphCreationPostRender)(::windows::core::Vtable::as_raw(self), pfiltergraph.into().abi()).ok()
    }
    pub unsafe fn GetGraphCreationFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetGraphCreationFlags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwflags)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPGraphCreation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPGraphCreation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPGraphCreation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPGraphCreation {}
impl ::core::fmt::Debug for IWMPGraphCreation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPGraphCreation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPGraphCreation {
    type Vtable = IWMPGraphCreation_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPGraphCreation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfb377e5_c594_4369_a970_de896d5ece74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPGraphCreation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GraphCreationPreRender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GraphCreationPostRender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGraphCreationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPLibrary(::windows::core::IUnknown);
impl IWMPLibrary {
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn r#type(&self, pwmplt: *mut WMPLibraryType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).r#type)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmplt)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    pub unsafe fn isIdentical<'a, P0>(&self, piwmplibrary: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).isIdentical)(::windows::core::Vtable::as_raw(self), piwmplibrary.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPLibrary, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPLibrary {}
impl ::core::fmt::Debug for IWMPLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPLibrary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPLibrary {
    type Vtable = IWMPLibrary_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPLibrary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df47861_7df1_4c1f_a81b_4c26f0f7a7c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrary_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub r#type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub mediaCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwmpmediacollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    mediaCollection: usize,
    pub isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmplibrary: *mut ::core::ffi::c_void, pvbool: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPLibrary2(::windows::core::IUnknown);
impl IWMPLibrary2 {
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn r#type(&self, pwmplt: *mut WMPLibraryType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.r#type)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmplt)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    pub unsafe fn isIdentical<'a, P0>(&self, piwmplibrary: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPLibrary>>,
    {
        (::windows::core::Vtable::vtable(self).base__.isIdentical)(::windows::core::Vtable::as_raw(self), piwmplibrary.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPLibrary2, ::windows::core::IUnknown, IWMPLibrary);
impl ::core::clone::Clone for IWMPLibrary2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPLibrary2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPLibrary2 {}
impl ::core::fmt::Debug for IWMPLibrary2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPLibrary2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPLibrary2 {
    type Vtable = IWMPLibrary2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPLibrary2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd578a4e_79b1_426c_bf8f_3add9072500b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrary2_Vtbl {
    pub base__: IWMPLibrary_Vtbl,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPLibraryServices(::windows::core::IUnknown);
impl IWMPLibraryServices {
    pub unsafe fn getCountByType(&self, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getCountByType)(::windows::core::Vtable::as_raw(self), wmplt, ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn getLibraryByType(&self, wmplt: WMPLibraryType, lindex: i32) -> ::windows::core::Result<IWMPLibrary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getLibraryByType)(::windows::core::Vtable::as_raw(self), wmplt, lindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPLibrary>(result__)
    }
}
::windows::core::interface_hierarchy!(IWMPLibraryServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPLibraryServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPLibraryServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPLibraryServices {}
impl ::core::fmt::Debug for IWMPLibraryServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPLibraryServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPLibraryServices {
    type Vtable = IWMPLibraryServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPLibraryServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39c2f8d5_1cf2_4d5e_ae09_d73492cf9eaa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibraryServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub getCountByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub getLibraryByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPLibrarySharingServices(::windows::core::IUnknown);
impl IWMPLibrarySharingServices {
    pub unsafe fn isLibraryShared(&self, pvbshared: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).isLibraryShared)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvbshared)).ok()
    }
    pub unsafe fn isLibrarySharingEnabled(&self, pvbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).isLibrarySharingEnabled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvbenabled)).ok()
    }
    pub unsafe fn showLibrarySharing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).showLibrarySharing)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPLibrarySharingServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPLibrarySharingServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPLibrarySharingServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPLibrarySharingServices {}
impl ::core::fmt::Debug for IWMPLibrarySharingServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPLibrarySharingServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPLibrarySharingServices {
    type Vtable = IWMPLibrarySharingServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPLibrarySharingServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82cba86b_9f04_474b_a365_d6dd1466e541);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrarySharingServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub isLibraryShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbshared: *mut i16) -> ::windows::core::HRESULT,
    pub isLibrarySharingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub showLibrarySharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPMedia(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_isIdentical<'a, P0>(&self, piwmpmedia: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).get_isIdentical)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
    pub unsafe fn sourceURL(&self, pbstrsourceurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).sourceURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsourceurl)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setname)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).imageSourceWidth)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwidth)).ok()
    }
    pub unsafe fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).imageSourceHeight)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pheight)).ok()
    }
    pub unsafe fn markerCount(&self, pmarkercount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).markerCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pmarkercount)).ok()
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getMarkerTime)(::windows::core::Vtable::as_raw(self), markernum, ::core::mem::transmute(pmarkertime)).ok()
    }
    pub unsafe fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getMarkerName)(::windows::core::Vtable::as_raw(self), markernum, ::core::mem::transmute(pbstrmarkername)).ok()
    }
    pub unsafe fn duration(&self, pduration: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).duration)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pduration)).ok()
    }
    pub unsafe fn durationString(&self, pbstrduration: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).durationString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrduration)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).attributeCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getAttributeName)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstritemname)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute_copy(bstrval)).ok()
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfoByAtom)(::windows::core::Vtable::as_raw(self), latom, ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn isMemberOf<'a, P0>(&self, pplaylist: P0, pvarfismemberof: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).isMemberOf)(::windows::core::Vtable::as_raw(self), pplaylist.into().abi(), ::core::mem::transmute(pvarfismemberof)).ok()
    }
    pub unsafe fn isReadOnlyItem(&self, bstritemname: &::windows::core::BSTR, pvarfisreadonly: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).isReadOnlyItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pvarfisreadonly)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPMedia, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPMedia {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMedia {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMedia {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMedia").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPMedia {
    type Vtable = IWMPMedia_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPMedia {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d55e95_3fac_11d3_b155_00c04f79faa6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void, pvbool: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_isIdentical: usize,
    pub sourceURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsourceurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub imageSourceWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT,
    pub imageSourceHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT,
    pub markerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmarkercount: *mut i32) -> ::windows::core::HRESULT,
    pub getMarkerTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::HRESULT,
    pub getMarkerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, markernum: i32, pbstrmarkername: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduration: *mut f64) -> ::windows::core::HRESULT,
    pub durationString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrduration: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub attributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub getAttributeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstritemname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrval: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getItemInfoByAtom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latom: i32, pbstrval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub isMemberOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplaylist: *mut ::core::ffi::c_void, pvarfismemberof: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    isMemberOf: usize,
    pub isReadOnlyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pvarfisreadonly: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPMedia2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_isIdentical<'a, P0>(&self, piwmpmedia: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.get_isIdentical)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
    pub unsafe fn sourceURL(&self, pbstrsourceurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.sourceURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsourceurl)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setname)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.imageSourceWidth)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwidth)).ok()
    }
    pub unsafe fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.imageSourceHeight)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pheight)).ok()
    }
    pub unsafe fn markerCount(&self, pmarkercount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.markerCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pmarkercount)).ok()
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getMarkerTime)(::windows::core::Vtable::as_raw(self), markernum, ::core::mem::transmute(pmarkertime)).ok()
    }
    pub unsafe fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getMarkerName)(::windows::core::Vtable::as_raw(self), markernum, ::core::mem::transmute(pbstrmarkername)).ok()
    }
    pub unsafe fn duration(&self, pduration: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.duration)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pduration)).ok()
    }
    pub unsafe fn durationString(&self, pbstrduration: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.durationString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrduration)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.attributeCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getAttributeName)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstritemname)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute_copy(bstrval)).ok()
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getItemInfoByAtom)(::windows::core::Vtable::as_raw(self), latom, ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn isMemberOf<'a, P0>(&self, pplaylist: P0, pvarfismemberof: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.isMemberOf)(::windows::core::Vtable::as_raw(self), pplaylist.into().abi(), ::core::mem::transmute(pvarfismemberof)).ok()
    }
    pub unsafe fn isReadOnlyItem(&self, bstritemname: &::windows::core::BSTR, pvarfisreadonly: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.isReadOnlyItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pvarfisreadonly)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPErrorItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPErrorItem>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPMedia2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPMedia);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPMedia2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMedia2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMedia2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMedia2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMedia2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPMedia2 {
    type Vtable = IWMPMedia2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPMedia2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab7c88bb_143e_4ea4_acc3_e4350b2106c3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia2_Vtbl {
    pub base__: IWMPMedia_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwmperroritem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    error: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPMedia3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_isIdentical<'a, P0>(&self, piwmpmedia: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.get_isIdentical)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
    pub unsafe fn sourceURL(&self, pbstrsourceurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.sourceURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsourceurl)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Setname)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.imageSourceWidth)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwidth)).ok()
    }
    pub unsafe fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.imageSourceHeight)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pheight)).ok()
    }
    pub unsafe fn markerCount(&self, pmarkercount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.markerCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pmarkercount)).ok()
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getMarkerTime)(::windows::core::Vtable::as_raw(self), markernum, ::core::mem::transmute(pmarkertime)).ok()
    }
    pub unsafe fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getMarkerName)(::windows::core::Vtable::as_raw(self), markernum, ::core::mem::transmute(pbstrmarkername)).ok()
    }
    pub unsafe fn duration(&self, pduration: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.duration)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pduration)).ok()
    }
    pub unsafe fn durationString(&self, pbstrduration: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.durationString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrduration)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.attributeCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getAttributeName)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstritemname)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute_copy(bstrval)).ok()
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getItemInfoByAtom)(::windows::core::Vtable::as_raw(self), latom, ::core::mem::transmute(pbstrval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn isMemberOf<'a, P0>(&self, pplaylist: P0, pvarfismemberof: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.isMemberOf)(::windows::core::Vtable::as_raw(self), pplaylist.into().abi(), ::core::mem::transmute(pvarfismemberof)).ok()
    }
    pub unsafe fn isReadOnlyItem(&self, bstritemname: &::windows::core::BSTR, pvarfisreadonly: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.isReadOnlyItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pvarfisreadonly)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPErrorItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPErrorItem>(result__)
    }
    pub unsafe fn getAttributeCountByType(&self, bstrtype: &::windows::core::BSTR, bstrlanguage: &::windows::core::BSTR, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getAttributeCountByType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype), ::core::mem::transmute_copy(bstrlanguage), ::core::mem::transmute(plcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getItemInfoByType(&self, bstrtype: &::windows::core::BSTR, bstrlanguage: &::windows::core::BSTR, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfoByType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype), ::core::mem::transmute_copy(bstrlanguage), lindex, ::core::mem::transmute(pvarvalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPMedia3, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPMedia, IWMPMedia2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPMedia3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMedia3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMedia3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMedia3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMedia3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPMedia3 {
    type Vtable = IWMPMedia3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPMedia3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf118efc7_f03a_4fb4_99c9_1c02a5c1065b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia3_Vtbl {
    pub base__: IWMPMedia2_Vtbl,
    pub getAttributeCountByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<::windows::core::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getItemInfoByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<::windows::core::BSTR>, lindex: i32, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getItemInfoByType: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPMediaCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollection {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn add(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).add)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAll(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getAll)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getByName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByGenre(&self, bstrgenre: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getByGenre)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrgenre), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAuthor(&self, bstrauthor: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getByAuthor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrauthor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAlbum(&self, bstralbum: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getByAlbum)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstralbum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAttribute(&self, bstrattribute: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getByAttribute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), ::core::mem::transmute_copy(bstrvalue), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn remove<'a, P0>(&self, pitem: P0, varfdeletefile: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).remove)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), varfdeletefile).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAttributeStringCollection(&self, bstrattribute: &::windows::core::BSTR, bstrmediatype: &::windows::core::BSTR) -> ::windows::core::Result<IWMPStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getAttributeStringCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), ::core::mem::transmute_copy(bstrmediatype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPStringCollection>(result__)
    }
    pub unsafe fn getMediaAtom(&self, bstritemname: &::windows::core::BSTR, platom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getMediaAtom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(platom)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn setDeleted<'a, P0>(&self, pitem: P0, varfisdeleted: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).setDeleted)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), varfisdeleted).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn isDeleted<'a, P0>(&self, pitem: P0, pvarfisdeleted: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).isDeleted)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), ::core::mem::transmute(pvarfisdeleted)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPMediaCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPMediaCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMediaCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMediaCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMediaCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMediaCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPMediaCollection {
    type Vtable = IWMPMediaCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPMediaCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8363bc22_b4b4_4b19_989d_1cd765749dd1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMediaCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAll: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByGenre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgenre: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByGenre: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByAuthor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthor: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByAuthor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByAlbum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstralbum: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByAlbum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByAttribute: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfdeletefile: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    remove: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAttributeStringCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppstringcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAttributeStringCollection: usize,
    pub getMediaAtom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, platom: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub setDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfisdeleted: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    setDeleted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub isDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    isDeleted: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPMediaCollection2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollection2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn add(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.add)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAll(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getAll)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByGenre(&self, bstrgenre: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByGenre)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrgenre), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAuthor(&self, bstrauthor: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByAuthor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrauthor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAlbum(&self, bstralbum: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByAlbum)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstralbum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAttribute(&self, bstrattribute: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getByAttribute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), ::core::mem::transmute_copy(bstrvalue), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn remove<'a, P0>(&self, pitem: P0, varfdeletefile: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.remove)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), varfdeletefile).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAttributeStringCollection(&self, bstrattribute: &::windows::core::BSTR, bstrmediatype: &::windows::core::BSTR) -> ::windows::core::Result<IWMPStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getAttributeStringCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), ::core::mem::transmute_copy(bstrmediatype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPStringCollection>(result__)
    }
    pub unsafe fn getMediaAtom(&self, bstritemname: &::windows::core::BSTR, platom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getMediaAtom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(platom)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn setDeleted<'a, P0>(&self, pitem: P0, varfisdeleted: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.setDeleted)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), varfisdeleted).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn isDeleted<'a, P0>(&self, pitem: P0, pvarfisdeleted: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.isDeleted)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), ::core::mem::transmute(pvarfisdeleted)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createQuery(&self) -> ::windows::core::Result<IWMPQuery> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).createQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPQuery>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getPlaylistByQuery<'a, P0>(&self, pquery: P0, bstrmediatype: &::windows::core::BSTR, bstrsortattribute: &::windows::core::BSTR, fsortascending: i16) -> ::windows::core::Result<IWMPPlaylist>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPQuery>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getPlaylistByQuery)(::windows::core::Vtable::as_raw(self), pquery.into().abi(), ::core::mem::transmute_copy(bstrmediatype), ::core::mem::transmute_copy(bstrsortattribute), fsortascending, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getStringCollectionByQuery<'a, P0>(&self, bstrattribute: &::windows::core::BSTR, pquery: P0, bstrmediatype: &::windows::core::BSTR, bstrsortattribute: &::windows::core::BSTR, fsortascending: i16) -> ::windows::core::Result<IWMPStringCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPQuery>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getStringCollectionByQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), pquery.into().abi(), ::core::mem::transmute_copy(bstrmediatype), ::core::mem::transmute_copy(bstrsortattribute), fsortascending, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByAttributeAndMediaType(&self, bstrattribute: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR, bstrmediatype: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getByAttributeAndMediaType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), ::core::mem::transmute_copy(bstrvalue), ::core::mem::transmute_copy(bstrmediatype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPMediaCollection2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPMediaCollection);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPMediaCollection2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMediaCollection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMediaCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMediaCollection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMediaCollection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPMediaCollection2 {
    type Vtable = IWMPMediaCollection2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPMediaCollection2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ba957f5_fd8c_4791_b82d_f840401ee474);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMediaCollection2_Vtbl {
    pub base__: IWMPMediaCollection_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub createQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createQuery: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getPlaylistByQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquery: *mut ::core::ffi::c_void, bstrmediatype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<::windows::core::BSTR>, fsortascending: i16, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getPlaylistByQuery: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getStringCollectionByQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pquery: *mut ::core::ffi::c_void, bstrmediatype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<::windows::core::BSTR>, fsortascending: i16, ppstringcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getStringCollectionByQuery: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByAttributeAndMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByAttributeAndMediaType: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPMediaPluginRegistrar(::windows::core::IUnknown);
impl IWMPMediaPluginRegistrar {
    pub unsafe fn WMPRegisterPlayerPlugin<'a, P0, P1, P2>(&self, pwszfriendlyname: P0, pwszdescription: P1, pwszuninstallstring: P2, dwpriority: u32, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).WMPRegisterPlayerPlugin)(::windows::core::Vtable::as_raw(self), pwszfriendlyname.into(), pwszdescription.into(), pwszuninstallstring.into(), dwpriority, ::core::mem::transmute(guidplugintype), ::core::mem::transmute(clsid), cmediatypes, ::core::mem::transmute(pmediatypes)).ok()
    }
    pub unsafe fn WMPUnRegisterPlayerPlugin(&self, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WMPUnRegisterPlayerPlugin)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidplugintype), ::core::mem::transmute(clsid)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPMediaPluginRegistrar, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPMediaPluginRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPMediaPluginRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPMediaPluginRegistrar {}
impl ::core::fmt::Debug for IWMPMediaPluginRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMediaPluginRegistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPMediaPluginRegistrar {
    type Vtable = IWMPMediaPluginRegistrar_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPMediaPluginRegistrar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68e27045_05bd_40b2_9720_23088c78e390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMediaPluginRegistrar_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub WMPRegisterPlayerPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows::core::PCWSTR, pwszdescription: ::windows::core::PCWSTR, pwszuninstallstring: ::windows::core::PCWSTR, dwpriority: u32, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WMPUnRegisterPlayerPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPMetadataPicture(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMetadataPicture {
    pub unsafe fn mimeType(&self, pbstrmimetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).mimeType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrmimetype)).ok()
    }
    pub unsafe fn pictureType(&self, pbstrpicturetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).pictureType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrpicturetype)).ok()
    }
    pub unsafe fn description(&self, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPMetadataPicture, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPMetadataPicture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMetadataPicture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMetadataPicture {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMetadataPicture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMetadataPicture").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPMetadataPicture {
    type Vtable = IWMPMetadataPicture_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPMetadataPicture {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c29bbe0_f87d_4c45_aa28_a70f0230ffa9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMetadataPicture_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub mimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmimetype: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub pictureType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpicturetype: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub URL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPMetadataText(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPMetadataText {
    pub unsafe fn description(&self, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdescription)).ok()
    }
    pub unsafe fn text(&self, pbstrtext: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).text)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrtext)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPMetadataText, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPMetadataText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPMetadataText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPMetadataText {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPMetadataText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPMetadataText").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPMetadataText {
    type Vtable = IWMPMetadataText_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPMetadataText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x769a72db_13d2_45e2_9c48_53ca9d5b7450);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMetadataText_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPNetwork(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPNetwork {
    pub unsafe fn bandWidth(&self, plbandwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).bandWidth)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plbandwidth)).ok()
    }
    pub unsafe fn recoveredPackets(&self, plrecoveredpackets: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).recoveredPackets)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plrecoveredpackets)).ok()
    }
    pub unsafe fn sourceProtocol(&self, pbstrsourceprotocol: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).sourceProtocol)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrsourceprotocol)).ok()
    }
    pub unsafe fn receivedPackets(&self, plreceivedpackets: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).receivedPackets)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plreceivedpackets)).ok()
    }
    pub unsafe fn lostPackets(&self, pllostpackets: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).lostPackets)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pllostpackets)).ok()
    }
    pub unsafe fn receptionQuality(&self, plreceptionquality: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).receptionQuality)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plreceptionquality)).ok()
    }
    pub unsafe fn bufferingCount(&self, plbufferingcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).bufferingCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plbufferingcount)).ok()
    }
    pub unsafe fn bufferingProgress(&self, plbufferingprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).bufferingProgress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plbufferingprogress)).ok()
    }
    pub unsafe fn bufferingTime(&self, plbufferingtime: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).bufferingTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plbufferingtime)).ok()
    }
    pub unsafe fn SetbufferingTime(&self, lbufferingtime: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetbufferingTime)(::windows::core::Vtable::as_raw(self), lbufferingtime).ok()
    }
    pub unsafe fn frameRate(&self, plframerate: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).frameRate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plframerate)).ok()
    }
    pub unsafe fn maxBitRate(&self, plbitrate: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).maxBitRate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plbitrate)).ok()
    }
    pub unsafe fn bitRate(&self, plbitrate: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).bitRate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plbitrate)).ok()
    }
    pub unsafe fn getProxySettings(&self, bstrprotocol: &::windows::core::BSTR, plproxysetting: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getProxySettings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), ::core::mem::transmute(plproxysetting)).ok()
    }
    pub unsafe fn setProxySettings(&self, bstrprotocol: &::windows::core::BSTR, lproxysetting: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setProxySettings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), lproxysetting).ok()
    }
    pub unsafe fn getProxyName(&self, bstrprotocol: &::windows::core::BSTR, pbstrproxyname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getProxyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), ::core::mem::transmute(pbstrproxyname)).ok()
    }
    pub unsafe fn setProxyName(&self, bstrprotocol: &::windows::core::BSTR, bstrproxyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setProxyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), ::core::mem::transmute_copy(bstrproxyname)).ok()
    }
    pub unsafe fn getProxyPort(&self, bstrprotocol: &::windows::core::BSTR, lproxyport: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getProxyPort)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), ::core::mem::transmute(lproxyport)).ok()
    }
    pub unsafe fn setProxyPort(&self, bstrprotocol: &::windows::core::BSTR, lproxyport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setProxyPort)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), lproxyport).ok()
    }
    pub unsafe fn getProxyExceptionList(&self, bstrprotocol: &::windows::core::BSTR, pbstrexceptionlist: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getProxyExceptionList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), ::core::mem::transmute(pbstrexceptionlist)).ok()
    }
    pub unsafe fn setProxyExceptionList(&self, bstrprotocol: &::windows::core::BSTR, pbstrexceptionlist: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setProxyExceptionList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), ::core::mem::transmute_copy(pbstrexceptionlist)).ok()
    }
    pub unsafe fn getProxyBypassForLocal(&self, bstrprotocol: &::windows::core::BSTR, pfbypassforlocal: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getProxyBypassForLocal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), ::core::mem::transmute(pfbypassforlocal)).ok()
    }
    pub unsafe fn setProxyBypassForLocal(&self, bstrprotocol: &::windows::core::BSTR, fbypassforlocal: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setProxyBypassForLocal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), fbypassforlocal).ok()
    }
    pub unsafe fn maxBandwidth(&self, lmaxbandwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).maxBandwidth)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(lmaxbandwidth)).ok()
    }
    pub unsafe fn SetmaxBandwidth(&self, lmaxbandwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetmaxBandwidth)(::windows::core::Vtable::as_raw(self), lmaxbandwidth).ok()
    }
    pub unsafe fn downloadProgress(&self, pldownloadprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).downloadProgress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pldownloadprogress)).ok()
    }
    pub unsafe fn encodedFrameRate(&self, plframerate: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).encodedFrameRate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plframerate)).ok()
    }
    pub unsafe fn framesSkipped(&self, plframes: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).framesSkipped)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plframes)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPNetwork, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPNetwork {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNetwork").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPNetwork {
    type Vtable = IWMPNetwork_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPNetwork {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec21b779_edef_462d_bba4_ad9dde2b29a7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNetwork_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub bandWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbandwidth: *mut i32) -> ::windows::core::HRESULT,
    pub recoveredPackets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plrecoveredpackets: *mut i32) -> ::windows::core::HRESULT,
    pub sourceProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsourceprotocol: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub receivedPackets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plreceivedpackets: *mut i32) -> ::windows::core::HRESULT,
    pub lostPackets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllostpackets: *mut i32) -> ::windows::core::HRESULT,
    pub receptionQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plreceptionquality: *mut i32) -> ::windows::core::HRESULT,
    pub bufferingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbufferingcount: *mut i32) -> ::windows::core::HRESULT,
    pub bufferingProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbufferingprogress: *mut i32) -> ::windows::core::HRESULT,
    pub bufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbufferingtime: *mut i32) -> ::windows::core::HRESULT,
    pub SetbufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbufferingtime: i32) -> ::windows::core::HRESULT,
    pub frameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT,
    pub maxBitRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT,
    pub bitRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT,
    pub getProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, plproxysetting: *mut i32) -> ::windows::core::HRESULT,
    pub setProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, lproxysetting: i32) -> ::windows::core::HRESULT,
    pub getProxyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrproxyname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setProxyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrproxyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, lproxyport: *mut i32) -> ::windows::core::HRESULT,
    pub setProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, lproxyport: i32) -> ::windows::core::HRESULT,
    pub getProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrexceptionlist: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrexceptionlist: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pfbypassforlocal: *mut i16) -> ::windows::core::HRESULT,
    pub setProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::windows::core::BSTR>, fbypassforlocal: i16) -> ::windows::core::HRESULT,
    pub maxBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxbandwidth: *mut i32) -> ::windows::core::HRESULT,
    pub SetmaxBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxbandwidth: i32) -> ::windows::core::HRESULT,
    pub downloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldownloadprogress: *mut i32) -> ::windows::core::HRESULT,
    pub encodedFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT,
    pub framesSkipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plframes: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPNodeRealEstate(::windows::core::IUnknown);
impl IWMPNodeRealEstate {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesiredSize(&self, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDesiredSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRects(&self, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psrc), ::core::mem::transmute(pdest), ::core::mem::transmute(pclip)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRects(&self, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetRects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psrc), ::core::mem::transmute(pdest), ::core::mem::transmute(pclip)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWindowless<'a, P0>(&self, fwindowless: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetWindowless)(::windows::core::Vtable::as_raw(self), fwindowless.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowless(&self, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetWindowless)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfwindowless)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullScreen<'a, P0>(&self, ffullscreen: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetFullScreen)(::windows::core::Vtable::as_raw(self), ffullscreen.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullScreen(&self, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFullScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pffullscreen)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPNodeRealEstate, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPNodeRealEstate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPNodeRealEstate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeRealEstate {}
impl ::core::fmt::Debug for IWMPNodeRealEstate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeRealEstate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPNodeRealEstate {
    type Vtable = IWMPNodeRealEstate_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPNodeRealEstate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42751198_5a50_4460_bcb4_709f8bdc8e59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeRealEstate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesiredSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesiredSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWindowless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWindowless: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindowless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindowless: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFullScreen: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPNodeRealEstateHost(::windows::core::IUnknown);
impl IWMPNodeRealEstateHost {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDesiredSizeChange(&self, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnDesiredSizeChange)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnFullScreenTransition<'a, P0>(&self, ffullscreen: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnFullScreenTransition)(::windows::core::Vtable::as_raw(self), ffullscreen.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPNodeRealEstateHost, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPNodeRealEstateHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPNodeRealEstateHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeRealEstateHost {}
impl ::core::fmt::Debug for IWMPNodeRealEstateHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeRealEstateHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPNodeRealEstateHost {
    type Vtable = IWMPNodeRealEstateHost_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPNodeRealEstateHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1491087d_2c6b_44c8_b019_b3c929d2ada9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeRealEstateHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDesiredSizeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDesiredSizeChange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnFullScreenTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnFullScreenTransition: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPNodeWindowed(::windows::core::IUnknown);
impl IWMPNodeWindowed {
    pub unsafe fn SetOwnerWindow(&self, hwnd: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOwnerWindow)(::windows::core::Vtable::as_raw(self), hwnd).ok()
    }
    pub unsafe fn GetOwnerWindow(&self, phwnd: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOwnerWindow)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(phwnd)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPNodeWindowed, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPNodeWindowed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPNodeWindowed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeWindowed {}
impl ::core::fmt::Debug for IWMPNodeWindowed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeWindowed").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPNodeWindowed {
    type Vtable = IWMPNodeWindowed_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPNodeWindowed {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96740bfa_c56a_45d1_a3a4_762914d4ade9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeWindowed_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetOwnerWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: isize) -> ::windows::core::HRESULT,
    pub GetOwnerWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut isize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPNodeWindowedHost(::windows::core::IUnknown);
impl IWMPNodeWindowedHost {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessageFromRenderer<'a, P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).OnWindowMessageFromRenderer)(::windows::core::Vtable::as_raw(self), umsg, wparam.into(), lparam.into(), ::core::mem::transmute(plret), ::core::mem::transmute(pfhandled)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPNodeWindowedHost, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPNodeWindowedHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPNodeWindowedHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeWindowedHost {}
impl ::core::fmt::Debug for IWMPNodeWindowedHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeWindowedHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPNodeWindowedHost {
    type Vtable = IWMPNodeWindowedHost_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPNodeWindowedHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa300415a_54aa_4081_adbf_3b13610d8958);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeWindowedHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowMessageFromRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowMessageFromRenderer: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPNodeWindowless(::windows::core::IUnknown);
impl IWMPNodeWindowless {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessage<'a, P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnWindowMessage)(::windows::core::Vtable::as_raw(self), umsg, wparam.into(), lparam.into(), ::core::mem::transmute(plret), ::core::mem::transmute(pfhandled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDraw(&self, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnDraw)(::windows::core::Vtable::as_raw(self), hdc, ::core::mem::transmute(prcdraw)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPNodeWindowless, ::windows::core::IUnknown, IWMPWindowMessageSink);
impl ::core::clone::Clone for IWMPNodeWindowless {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPNodeWindowless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeWindowless {}
impl ::core::fmt::Debug for IWMPNodeWindowless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeWindowless").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPNodeWindowless {
    type Vtable = IWMPNodeWindowless_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPNodeWindowless {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b9199ad_780c_4eda_b816_261eba5d1575);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeWindowless_Vtbl {
    pub base__: IWMPWindowMessageSink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDraw: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPNodeWindowlessHost(::windows::core::IUnknown);
impl IWMPNodeWindowlessHost {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvalidateRect<'a, P0>(&self, prc: *const super::super::Foundation::RECT, ferase: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).InvalidateRect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(prc), ferase.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPNodeWindowlessHost, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPNodeWindowlessHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPNodeWindowlessHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPNodeWindowlessHost {}
impl ::core::fmt::Debug for IWMPNodeWindowlessHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPNodeWindowlessHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPNodeWindowlessHost {
    type Vtable = IWMPNodeWindowlessHost_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPNodeWindowlessHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe7017c6_ce34_4901_8106_770381aa6e3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNodeWindowlessHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InvalidateRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvalidateRect: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPPlayer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.openState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpos)).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.playState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpps)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.controls)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPControls>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.settings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<'a, P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.playlistCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistCollection>(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.network)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPNetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<'a, P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.cdromCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdromCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.closedCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPClosedCaption>(result__)
    }
    pub unsafe fn isOnline(&self, pfonline: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.isOnline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfonline)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPError>(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    pub unsafe fn enabled(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enabled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn Setenabled(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setenabled)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
    pub unsafe fn fullScreen(&self, pbfullscreen: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).fullScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbfullscreen)).ok()
    }
    pub unsafe fn SetfullScreen(&self, bfullscreen: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetfullScreen)(::windows::core::Vtable::as_raw(self), bfullscreen).ok()
    }
    pub unsafe fn enableContextMenu(&self, pbenablecontextmenu: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enableContextMenu)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenablecontextmenu)).ok()
    }
    pub unsafe fn SetenableContextMenu(&self, benablecontextmenu: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetenableContextMenu)(::windows::core::Vtable::as_raw(self), benablecontextmenu).ok()
    }
    pub unsafe fn SetuiMode(&self, bstrmode: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetuiMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode)).ok()
    }
    pub unsafe fn uiMode(&self, pbstrmode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).uiMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrmode)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPPlayer, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPCore);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPPlayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPPlayer {
    type Vtable = IWMPPlayer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPPlayer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf52a4f_394a_11d3_b153_00c04f79faa6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer_Vtbl {
    pub base__: IWMPCore_Vtbl,
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
    pub fullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT,
    pub SetfullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT,
    pub enableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT,
    pub SetenableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT,
    pub SetuiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub uiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPPlayer2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer2 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.openState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpos)).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.playState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpps)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.controls)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPControls>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.settings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<'a, P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.playlistCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistCollection>(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.network)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPNetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<'a, P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.cdromCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdromCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.closedCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPClosedCaption>(result__)
    }
    pub unsafe fn isOnline(&self, pfonline: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.isOnline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfonline)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPError>(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    pub unsafe fn enabled(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enabled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn Setenabled(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setenabled)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
    pub unsafe fn fullScreen(&self, pbfullscreen: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).fullScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbfullscreen)).ok()
    }
    pub unsafe fn SetfullScreen(&self, bfullscreen: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetfullScreen)(::windows::core::Vtable::as_raw(self), bfullscreen).ok()
    }
    pub unsafe fn enableContextMenu(&self, pbenablecontextmenu: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enableContextMenu)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenablecontextmenu)).ok()
    }
    pub unsafe fn SetenableContextMenu(&self, benablecontextmenu: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetenableContextMenu)(::windows::core::Vtable::as_raw(self), benablecontextmenu).ok()
    }
    pub unsafe fn SetuiMode(&self, bstrmode: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetuiMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode)).ok()
    }
    pub unsafe fn uiMode(&self, pbstrmode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).uiMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrmode)).ok()
    }
    pub unsafe fn stretchToFit(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stretchToFit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn SetstretchToFit(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetstretchToFit)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
    pub unsafe fn windowlessVideo(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).windowlessVideo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn SetwindowlessVideo(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetwindowlessVideo)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPPlayer2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPCore);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPPlayer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayer2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayer2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPPlayer2 {
    type Vtable = IWMPPlayer2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPPlayer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e6b01d1_d407_4c85_bf5f_1c01f6150280);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer2_Vtbl {
    pub base__: IWMPCore_Vtbl,
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
    pub fullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT,
    pub SetfullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT,
    pub enableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT,
    pub SetenableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT,
    pub SetuiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub uiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub stretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetstretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
    pub windowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetwindowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPPlayer3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer3 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.openState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpos)).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.playState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpps)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.controls)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPControls>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.settings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<'a, P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.playlistCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistCollection>(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.network)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPNetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<'a, P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.cdromCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdromCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.closedCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPClosedCaption>(result__)
    }
    pub unsafe fn isOnline(&self, pfonline: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.isOnline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfonline)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPError>(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows::core::Result<IWMPDVD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.dvd)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPDVD>(result__)
    }
    pub unsafe fn enabled(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enabled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn Setenabled(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setenabled)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
    pub unsafe fn fullScreen(&self, pbfullscreen: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).fullScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbfullscreen)).ok()
    }
    pub unsafe fn SetfullScreen(&self, bfullscreen: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetfullScreen)(::windows::core::Vtable::as_raw(self), bfullscreen).ok()
    }
    pub unsafe fn enableContextMenu(&self, pbenablecontextmenu: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enableContextMenu)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenablecontextmenu)).ok()
    }
    pub unsafe fn SetenableContextMenu(&self, benablecontextmenu: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetenableContextMenu)(::windows::core::Vtable::as_raw(self), benablecontextmenu).ok()
    }
    pub unsafe fn SetuiMode(&self, bstrmode: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetuiMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode)).ok()
    }
    pub unsafe fn uiMode(&self, pbstrmode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).uiMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrmode)).ok()
    }
    pub unsafe fn stretchToFit(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stretchToFit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn SetstretchToFit(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetstretchToFit)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
    pub unsafe fn windowlessVideo(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).windowlessVideo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn SetwindowlessVideo(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetwindowlessVideo)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPPlayer3, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPCore, IWMPCore2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPPlayer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayer3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayer3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPPlayer3 {
    type Vtable = IWMPPlayer3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPPlayer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54062b68_052a_4c25_a39f_8b63346511d4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer3_Vtbl {
    pub base__: IWMPCore2_Vtbl,
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
    pub fullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT,
    pub SetfullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT,
    pub enableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT,
    pub SetenableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT,
    pub SetuiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub uiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub stretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetstretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
    pub windowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetwindowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPPlayer4(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer4 {
    pub unsafe fn close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.URL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrurl)).ok()
    }
    pub unsafe fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    pub unsafe fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.openState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpos)).ok()
    }
    pub unsafe fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.playState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpps)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn controls(&self) -> ::windows::core::Result<IWMPControls> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.controls)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPControls>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn settings(&self) -> ::windows::core::Result<IWMPSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.settings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.currentMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentMedia<'a, P0>(&self, pmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetcurrentMedia)(::windows::core::Vtable::as_raw(self), pmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.mediaCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMediaCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.playlistCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistCollection>(result__)
    }
    pub unsafe fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.versionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrversioninfo)).ok()
    }
    pub unsafe fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.launchURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn network(&self) -> ::windows::core::Result<IWMPNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.network)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPNetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.currentPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetcurrentPlaylist<'a, P0>(&self, ppl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetcurrentPlaylist)(::windows::core::Vtable::as_raw(self), ppl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.cdromCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPCdromCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.closedCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPClosedCaption>(result__)
    }
    pub unsafe fn isOnline(&self, pfonline: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.isOnline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfonline)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error(&self) -> ::windows::core::Result<IWMPError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.error)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPError>(result__)
    }
    pub unsafe fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dvd(&self) -> ::windows::core::Result<IWMPDVD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.dvd)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPDVD>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newPlaylist(&self, bstrname: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.newPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newMedia(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.newMedia)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    pub unsafe fn enabled(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enabled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn Setenabled(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setenabled)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
    pub unsafe fn fullScreen(&self, pbfullscreen: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).fullScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbfullscreen)).ok()
    }
    pub unsafe fn SetfullScreen(&self, bfullscreen: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetfullScreen)(::windows::core::Vtable::as_raw(self), bfullscreen).ok()
    }
    pub unsafe fn enableContextMenu(&self, pbenablecontextmenu: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enableContextMenu)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenablecontextmenu)).ok()
    }
    pub unsafe fn SetenableContextMenu(&self, benablecontextmenu: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetenableContextMenu)(::windows::core::Vtable::as_raw(self), benablecontextmenu).ok()
    }
    pub unsafe fn SetuiMode(&self, bstrmode: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetuiMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode)).ok()
    }
    pub unsafe fn uiMode(&self, pbstrmode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).uiMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrmode)).ok()
    }
    pub unsafe fn stretchToFit(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stretchToFit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn SetstretchToFit(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetstretchToFit)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
    pub unsafe fn windowlessVideo(&self, pbenabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).windowlessVideo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbenabled)).ok()
    }
    pub unsafe fn SetwindowlessVideo(&self, benabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetwindowlessVideo)(::windows::core::Vtable::as_raw(self), benabled).ok()
    }
    pub unsafe fn isRemote(&self, pvarfisremote: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).isRemote)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvarfisremote)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn playerApplication(&self) -> ::windows::core::Result<IWMPPlayerApplication> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).playerApplication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlayerApplication>(result__)
    }
    pub unsafe fn openPlayer(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).openPlayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPPlayer4, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPCore, IWMPCore2, IWMPCore3);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPPlayer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayer4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayer4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayer4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayer4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPPlayer4 {
    type Vtable = IWMPPlayer4_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPPlayer4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c497d62_8919_413c_82db_e935fb3ec584);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer4_Vtbl {
    pub base__: IWMPCore3_Vtbl,
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
    pub fullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT,
    pub SetfullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT,
    pub enableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT,
    pub SetenableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT,
    pub SetuiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub uiMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub stretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetstretchToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
    pub windowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetwindowlessVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT,
    pub isRemote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarfisremote: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub playerApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwmpplayerapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    playerApplication: usize,
    pub openPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPPlayerApplication(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayerApplication {
    pub unsafe fn switchToPlayerApplication(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).switchToPlayerApplication)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn switchToControl(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).switchToControl)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn playerDocked(&self, pbplayerdocked: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).playerDocked)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbplayerdocked)).ok()
    }
    pub unsafe fn hasDisplay(&self, pbhasdisplay: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).hasDisplay)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbhasdisplay)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPPlayerApplication, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPPlayerApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlayerApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlayerApplication {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlayerApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayerApplication").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPPlayerApplication {
    type Vtable = IWMPPlayerApplication_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPPlayerApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40897764_ceab_47be_ad4a_8e28537f9bbf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub switchToPlayerApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub switchToControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub playerDocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbplayerdocked: *mut i16) -> ::windows::core::HRESULT,
    pub hasDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbhasdisplay: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPPlayerServices(::windows::core::IUnknown);
impl IWMPPlayerServices {
    pub unsafe fn activateUIPlugin(&self, bstrplugin: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).activateUIPlugin)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplugin)).ok()
    }
    pub unsafe fn setTaskPane(&self, bstrtaskpane: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setTaskPane)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtaskpane)).ok()
    }
    pub unsafe fn setTaskPaneURL(&self, bstrtaskpane: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, bstrfriendlyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setTaskPaneURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtaskpane), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute_copy(bstrfriendlyname)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPPlayerServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPPlayerServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPPlayerServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPlayerServices {}
impl ::core::fmt::Debug for IWMPPlayerServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayerServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPPlayerServices {
    type Vtable = IWMPPlayerServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPPlayerServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d01fbdb_ade2_4c8d_9842_c190b95c3306);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub activateUIPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrplugin: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setTaskPane: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setTaskPaneURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPPlayerServices2(::windows::core::IUnknown);
impl IWMPPlayerServices2 {
    pub unsafe fn activateUIPlugin(&self, bstrplugin: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.activateUIPlugin)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrplugin)).ok()
    }
    pub unsafe fn setTaskPane(&self, bstrtaskpane: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setTaskPane)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtaskpane)).ok()
    }
    pub unsafe fn setTaskPaneURL(&self, bstrtaskpane: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, bstrfriendlyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setTaskPaneURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtaskpane), ::core::mem::transmute_copy(bstrurl), ::core::mem::transmute_copy(bstrfriendlyname)).ok()
    }
    pub unsafe fn setBackgroundProcessingPriority(&self, bstrpriority: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setBackgroundProcessingPriority)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpriority)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPPlayerServices2, ::windows::core::IUnknown, IWMPPlayerServices);
impl ::core::clone::Clone for IWMPPlayerServices2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPPlayerServices2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPlayerServices2 {}
impl ::core::fmt::Debug for IWMPPlayerServices2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlayerServices2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPPlayerServices2 {
    type Vtable = IWMPPlayerServices2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPPlayerServices2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb1592f_f040_418a_9f71_17c7512b4d70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerServices2_Vtbl {
    pub base__: IWMPPlayerServices_Vtbl,
    pub setBackgroundProcessingPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpriority: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPPlaylist(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylist {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn Setname(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setname)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn attributeCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).attributeCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn get_attributeName(&self, lindex: i32, pbstrattributename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).get_attributeName)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstrattributename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, lindex: i32) -> ::windows::core::Result<IWMPMedia> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_item)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPMedia>(result__)
    }
    pub unsafe fn getItemInfo(&self, bstrname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn setItemInfo(&self, bstrname: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_isIdentical<'a, P0>(&self, piwmpplaylist: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).get_isIdentical)(::windows::core::Vtable::as_raw(self), piwmpplaylist.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
    pub unsafe fn clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn insertItem<'a, P0>(&self, lindex: i32, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).insertItem)(::windows::core::Vtable::as_raw(self), lindex, piwmpmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendItem<'a, P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).appendItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeItem<'a, P0>(&self, piwmpmedia: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).removeItem)(::windows::core::Vtable::as_raw(self), piwmpmedia.into().abi()).ok()
    }
    pub unsafe fn moveItem(&self, lindexold: i32, lindexnew: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).moveItem)(::windows::core::Vtable::as_raw(self), lindexold, lindexnew).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPPlaylist, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPPlaylist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlaylist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlaylist {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlaylist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlaylist").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPPlaylist {
    type Vtable = IWMPPlaylist_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPPlaylist {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f0f4f1_130c_11d3_b14e_00c04f79faa6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylist_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub attributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub get_attributeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstrattributename: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_item: usize,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpplaylist: *mut ::core::ffi::c_void, pvbool: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_isIdentical: usize,
    pub clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub insertItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    insertItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub appendItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    appendItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeItem: usize,
    pub moveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindexold: i32, lindexnew: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPPlaylistArray(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistArray {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn item(&self, lindex: i32) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).item)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPPlaylistArray, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPPlaylistArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlaylistArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlaylistArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlaylistArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlaylistArray").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPPlaylistArray {
    type Vtable = IWMPPlaylistArray_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPPlaylistArray {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x679409c0_99f7_11d3_9fb7_00105aa620bb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylistArray_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    item: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPPlaylistCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistCollection {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn newPlaylist(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).newPlaylist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAll(&self) -> ::windows::core::Result<IWMPPlaylistArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getAll)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistArray>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getByName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylistArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getByName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylistArray>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn remove<'a, P0>(&self, pitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).remove)(::windows::core::Vtable::as_raw(self), pitem.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn setDeleted<'a, P0>(&self, pitem: P0, varfisdeleted: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).setDeleted)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), varfisdeleted).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn isDeleted<'a, P0>(&self, pitem: P0, pvarfisdeleted: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).isDeleted)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), ::core::mem::transmute(pvarfisdeleted)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn importPlaylist<'a, P0>(&self, pitem: P0) -> ::windows::core::Result<IWMPPlaylist>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).importPlaylist)(::windows::core::Vtable::as_raw(self), pitem.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPPlaylist>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPPlaylistCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPPlaylistCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPPlaylistCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPPlaylistCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPPlaylistCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlaylistCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPPlaylistCollection {
    type Vtable = IWMPPlaylistCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPPlaylistCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10a13217_23a7_439b_b1c0_d847c79b7774);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylistCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub newPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    newPlaylist: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplaylistarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAll: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppplaylistarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    remove: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub setDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfisdeleted: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    setDeleted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub isDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    isDeleted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub importPlaylist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, ppimporteditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    importPlaylist: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPPlugin(::windows::core::IUnknown);
impl IWMPPlugin {
    pub unsafe fn Init(&self, dwplaybackcontext: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Init)(::windows::core::Vtable::as_raw(self), dwplaybackcontext).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetID(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetCaps(&self, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCaps)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwflags)).ok()
    }
    pub unsafe fn AdviseWMPServices<'a, P0>(&self, pwmpservices: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPServices>>,
    {
        (::windows::core::Vtable::vtable(self).AdviseWMPServices)(::windows::core::Vtable::as_raw(self), pwmpservices.into().abi()).ok()
    }
    pub unsafe fn UnAdviseWMPServices(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnAdviseWMPServices)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPPlugin, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPlugin {}
impl ::core::fmt::Debug for IWMPPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPPlugin {
    type Vtable = IWMPPlugin_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPPlugin {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1392a70_024c_42bb_a998_73dfdfe7d5a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlugin_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwplaybackcontext: usize) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub AdviseWMPServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpservices: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnAdviseWMPServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPPluginEnable(::windows::core::IUnknown);
impl IWMPPluginEnable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnable<'a, P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnable)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnable(&self, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetEnable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfenable)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPPluginEnable, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPPluginEnable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPPluginEnable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPluginEnable {}
impl ::core::fmt::Debug for IWMPPluginEnable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPluginEnable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPPluginEnable {
    type Vtable = IWMPPluginEnable_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPPluginEnable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fca444c_7ad1_479d_a4ef_40566a5309d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPluginEnable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnable: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPPluginUI(::windows::core::IUnknown);
impl IWMPPluginUI {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCore<'a, P0>(&self, pcore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPCore>>,
    {
        (::windows::core::Vtable::vtable(self).SetCore)(::windows::core::Vtable::as_raw(self), pcore.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, P0>(&self, hwndparent: P0, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).Create)(::windows::core::Vtable::as_raw(self), hwndparent.into(), ::core::mem::transmute(phwndwindow)).ok()
    }
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Destroy)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayPropertyPage<'a, P0>(&self, hwndparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DisplayPropertyPage)(::windows::core::Vtable::as_raw(self), hwndparent.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, P0>(&self, pwszname: P0, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), pwszname.into(), ::core::mem::transmute(pvarproperty)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, P0>(&self, pwszname: P0, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetProperty)(::windows::core::Vtable::as_raw(self), pwszname.into(), ::core::mem::transmute(pvarproperty)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TranslateAccelerator)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(lpmsg)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPPluginUI, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPPluginUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPPluginUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPPluginUI {}
impl ::core::fmt::Debug for IWMPPluginUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPPluginUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPPluginUI {
    type Vtable = IWMPPluginUI_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPPluginUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c5e8f9f_ad3e_4bf9_9753_fcd30d6d38dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPluginUI_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayPropertyPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayPropertyPage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pvarproperty: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pvarproperty: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPQuery(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPQuery {
    pub unsafe fn addCondition(&self, bstrattribute: &::windows::core::BSTR, bstroperator: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).addCondition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrattribute), ::core::mem::transmute_copy(bstroperator), ::core::mem::transmute_copy(bstrvalue)).ok()
    }
    pub unsafe fn beginNextGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).beginNextGroup)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPQuery, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPQuery {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPQuery").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPQuery {
    type Vtable = IWMPQuery_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa00918f3_a6b0_4bfb_9189_fd834c7bc5a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPQuery_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub addCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstroperator: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub beginNextGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPRemoteMediaServices(::windows::core::IUnknown);
impl IWMPRemoteMediaServices {
    pub unsafe fn GetServiceType(&self, pbstrtype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetServiceType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrtype)).ok()
    }
    pub unsafe fn GetApplicationName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetApplicationName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetScriptableObject(&self, pbstrname: *mut ::windows::core::BSTR, ppdispatch: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetScriptableObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname), ::core::mem::transmute(ppdispatch)).ok()
    }
    pub unsafe fn GetCustomUIMode(&self, pbstrfile: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCustomUIMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrfile)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPRemoteMediaServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPRemoteMediaServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPRemoteMediaServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPRemoteMediaServices {}
impl ::core::fmt::Debug for IWMPRemoteMediaServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPRemoteMediaServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPRemoteMediaServices {
    type Vtable = IWMPRemoteMediaServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPRemoteMediaServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbb92747_741f_44fe_ab5b_f1a48f3b2a59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPRemoteMediaServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetServiceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetScriptableObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetScriptableObject: usize,
    pub GetCustomUIMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfile: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPRenderConfig(::windows::core::IUnknown);
impl IWMPRenderConfig {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetinProcOnly<'a, P0>(&self, finproc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetinProcOnly)(::windows::core::Vtable::as_raw(self), finproc.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn inProcOnly(&self, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).inProcOnly)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfinproc)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPRenderConfig, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPRenderConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPRenderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPRenderConfig {}
impl ::core::fmt::Debug for IWMPRenderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPRenderConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPRenderConfig {
    type Vtable = IWMPRenderConfig_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPRenderConfig {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x959506c1_0314_4ec5_9e61_8528db5e5478);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPRenderConfig_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetinProcOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetinProcOnly: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub inProcOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    inProcOnly: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPServices(::windows::core::IUnknown);
impl IWMPServices {
    pub unsafe fn GetStreamTime(&self, prt: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetStreamTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(prt)).ok()
    }
    pub unsafe fn GetStreamState(&self, pstate: *mut WMPServices_StreamState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetStreamState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pstate)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPServices {}
impl ::core::fmt::Debug for IWMPServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPServices {
    type Vtable = IWMPServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafb6b76b_1e20_4198_83b3_191db6e0b149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStreamTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT,
    pub GetStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPSettings(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettings {
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), ::core::mem::transmute(pisavailable)).ok()
    }
    pub unsafe fn autoStart(&self, pfautostart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).autoStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfautostart)).ok()
    }
    pub unsafe fn SetautoStart(&self, fautostart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetautoStart)(::windows::core::Vtable::as_raw(self), fautostart).ok()
    }
    pub unsafe fn baseURL(&self, pbstrbaseurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).baseURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbaseurl)).ok()
    }
    pub unsafe fn SetbaseURL(&self, bstrbaseurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetbaseURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrbaseurl)).ok()
    }
    pub unsafe fn defaultFrame(&self, pbstrdefaultframe: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).defaultFrame)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdefaultframe)).ok()
    }
    pub unsafe fn SetdefaultFrame(&self, bstrdefaultframe: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetdefaultFrame)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdefaultframe)).ok()
    }
    pub unsafe fn invokeURLs(&self, pfinvokeurls: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).invokeURLs)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfinvokeurls)).ok()
    }
    pub unsafe fn SetinvokeURLs(&self, finvokeurls: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetinvokeURLs)(::windows::core::Vtable::as_raw(self), finvokeurls).ok()
    }
    pub unsafe fn mute(&self, pfmute: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).mute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfmute)).ok()
    }
    pub unsafe fn Setmute(&self, fmute: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setmute)(::windows::core::Vtable::as_raw(self), fmute).ok()
    }
    pub unsafe fn playCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).playCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn SetplayCount(&self, lcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetplayCount)(::windows::core::Vtable::as_raw(self), lcount).ok()
    }
    pub unsafe fn rate(&self, pdrate: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).rate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdrate)).ok()
    }
    pub unsafe fn Setrate(&self, drate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setrate)(::windows::core::Vtable::as_raw(self), drate).ok()
    }
    pub unsafe fn balance(&self, plbalance: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).balance)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plbalance)).ok()
    }
    pub unsafe fn Setbalance(&self, lbalance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setbalance)(::windows::core::Vtable::as_raw(self), lbalance).ok()
    }
    pub unsafe fn volume(&self, plvolume: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).volume)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plvolume)).ok()
    }
    pub unsafe fn Setvolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Setvolume)(::windows::core::Vtable::as_raw(self), lvolume).ok()
    }
    pub unsafe fn getMode(&self, bstrmode: &::windows::core::BSTR, pvarfmode: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode), ::core::mem::transmute(pvarfmode)).ok()
    }
    pub unsafe fn setMode(&self, bstrmode: &::windows::core::BSTR, varfmode: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode), varfmode).ok()
    }
    pub unsafe fn enableErrorDialogs(&self, pfenableerrordialogs: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).enableErrorDialogs)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfenableerrordialogs)).ok()
    }
    pub unsafe fn SetenableErrorDialogs(&self, fenableerrordialogs: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetenableErrorDialogs)(::windows::core::Vtable::as_raw(self), fenableerrordialogs).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPSettings, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPSettings {
    type Vtable = IWMPSettings_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9104d1ab_80c9_4fed_abf0_2e6417a6df14);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSettings_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_isAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT,
    pub autoStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfautostart: *mut i16) -> ::windows::core::HRESULT,
    pub SetautoStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fautostart: i16) -> ::windows::core::HRESULT,
    pub baseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbaseurl: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetbaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbaseurl: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub defaultFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdefaultframe: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetdefaultFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdefaultframe: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub invokeURLs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfinvokeurls: *mut i16) -> ::windows::core::HRESULT,
    pub SetinvokeURLs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finvokeurls: i16) -> ::windows::core::HRESULT,
    pub mute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmute: *mut i16) -> ::windows::core::HRESULT,
    pub Setmute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmute: i16) -> ::windows::core::HRESULT,
    pub playCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub SetplayCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcount: i32) -> ::windows::core::HRESULT,
    pub rate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdrate: *mut f64) -> ::windows::core::HRESULT,
    pub Setrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drate: f64) -> ::windows::core::HRESULT,
    pub balance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT,
    pub Setbalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT,
    pub volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub Setvolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT,
    pub getMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pvarfmode: *mut i16) -> ::windows::core::HRESULT,
    pub setMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<::windows::core::BSTR>, varfmode: i16) -> ::windows::core::HRESULT,
    pub enableErrorDialogs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableerrordialogs: *mut i16) -> ::windows::core::HRESULT,
    pub SetenableErrorDialogs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableerrordialogs: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPSettings2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettings2 {
    pub unsafe fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.get_isAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritem), ::core::mem::transmute(pisavailable)).ok()
    }
    pub unsafe fn autoStart(&self, pfautostart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.autoStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfautostart)).ok()
    }
    pub unsafe fn SetautoStart(&self, fautostart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetautoStart)(::windows::core::Vtable::as_raw(self), fautostart).ok()
    }
    pub unsafe fn baseURL(&self, pbstrbaseurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.baseURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbaseurl)).ok()
    }
    pub unsafe fn SetbaseURL(&self, bstrbaseurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetbaseURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrbaseurl)).ok()
    }
    pub unsafe fn defaultFrame(&self, pbstrdefaultframe: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.defaultFrame)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdefaultframe)).ok()
    }
    pub unsafe fn SetdefaultFrame(&self, bstrdefaultframe: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetdefaultFrame)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdefaultframe)).ok()
    }
    pub unsafe fn invokeURLs(&self, pfinvokeurls: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.invokeURLs)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfinvokeurls)).ok()
    }
    pub unsafe fn SetinvokeURLs(&self, finvokeurls: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetinvokeURLs)(::windows::core::Vtable::as_raw(self), finvokeurls).ok()
    }
    pub unsafe fn mute(&self, pfmute: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.mute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfmute)).ok()
    }
    pub unsafe fn Setmute(&self, fmute: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setmute)(::windows::core::Vtable::as_raw(self), fmute).ok()
    }
    pub unsafe fn playCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.playCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn SetplayCount(&self, lcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetplayCount)(::windows::core::Vtable::as_raw(self), lcount).ok()
    }
    pub unsafe fn rate(&self, pdrate: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.rate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdrate)).ok()
    }
    pub unsafe fn Setrate(&self, drate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setrate)(::windows::core::Vtable::as_raw(self), drate).ok()
    }
    pub unsafe fn balance(&self, plbalance: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.balance)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plbalance)).ok()
    }
    pub unsafe fn Setbalance(&self, lbalance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setbalance)(::windows::core::Vtable::as_raw(self), lbalance).ok()
    }
    pub unsafe fn volume(&self, plvolume: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.volume)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plvolume)).ok()
    }
    pub unsafe fn Setvolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Setvolume)(::windows::core::Vtable::as_raw(self), lvolume).ok()
    }
    pub unsafe fn getMode(&self, bstrmode: &::windows::core::BSTR, pvarfmode: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode), ::core::mem::transmute(pvarfmode)).ok()
    }
    pub unsafe fn setMode(&self, bstrmode: &::windows::core::BSTR, varfmode: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmode), varfmode).ok()
    }
    pub unsafe fn enableErrorDialogs(&self, pfenableerrordialogs: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.enableErrorDialogs)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfenableerrordialogs)).ok()
    }
    pub unsafe fn SetenableErrorDialogs(&self, fenableerrordialogs: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetenableErrorDialogs)(::windows::core::Vtable::as_raw(self), fenableerrordialogs).ok()
    }
    pub unsafe fn defaultAudioLanguage(&self, pllangid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).defaultAudioLanguage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pllangid)).ok()
    }
    pub unsafe fn mediaAccessRights(&self, pbstrrights: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).mediaAccessRights)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrrights)).ok()
    }
    pub unsafe fn requestMediaAccessRights(&self, bstrdesiredaccess: &::windows::core::BSTR, pvbaccepted: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).requestMediaAccessRights)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdesiredaccess), ::core::mem::transmute(pvbaccepted)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPSettings2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPSettings);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPSettings2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPSettings2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPSettings2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPSettings2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSettings2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPSettings2 {
    type Vtable = IWMPSettings2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPSettings2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfda937a4_eece_4da5_a0b6_39bf89ade2c2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSettings2_Vtbl {
    pub base__: IWMPSettings_Vtbl,
    pub defaultAudioLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT,
    pub mediaAccessRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrights: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub requestMediaAccessRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdesiredaccess: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pvbaccepted: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPSkinManager(::windows::core::IUnknown);
impl IWMPSkinManager {
    pub unsafe fn SetVisualStyle(&self, bstrpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVisualStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpath)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPSkinManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPSkinManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPSkinManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSkinManager {}
impl ::core::fmt::Debug for IWMPSkinManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSkinManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPSkinManager {
    type Vtable = IWMPSkinManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPSkinManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076f2fa6_ed30_448b_8cc5_3f3ef3529c7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSkinManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetVisualStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPStringCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollection {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn item(&self, lindex: i32, pbstrstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).item)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstrstring)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPStringCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPStringCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPStringCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPStringCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPStringCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPStringCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPStringCollection {
    type Vtable = IWMPStringCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPStringCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a976298_8c0d_11d3_b389_00c04f68574b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPStringCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pbstrstring: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMPStringCollection2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollection2 {
    pub unsafe fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn item(&self, lindex: i32, pbstrstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.item)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(pbstrstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn isIdentical<'a, P0>(&self, piwmpstringcollection2: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPStringCollection2>>,
    {
        (::windows::core::Vtable::vtable(self).isIdentical)(::windows::core::Vtable::as_raw(self), piwmpstringcollection2.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
    pub unsafe fn getItemInfo(&self, lcollectionindex: i32, bstritemname: &::windows::core::BSTR, pbstrvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfo)(::windows::core::Vtable::as_raw(self), lcollectionindex, ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrvalue)).ok()
    }
    pub unsafe fn getAttributeCountByType(&self, lcollectionindex: i32, bstrtype: &::windows::core::BSTR, bstrlanguage: &::windows::core::BSTR, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getAttributeCountByType)(::windows::core::Vtable::as_raw(self), lcollectionindex, ::core::mem::transmute_copy(bstrtype), ::core::mem::transmute_copy(bstrlanguage), ::core::mem::transmute(plcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getItemInfoByType(&self, lcollectionindex: i32, bstrtype: &::windows::core::BSTR, bstrlanguage: &::windows::core::BSTR, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfoByType)(::windows::core::Vtable::as_raw(self), lcollectionindex, ::core::mem::transmute_copy(bstrtype), ::core::mem::transmute_copy(bstrlanguage), lattributeindex, ::core::mem::transmute(pvarvalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWMPStringCollection2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IWMPStringCollection);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMPStringCollection2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMPStringCollection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMPStringCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMPStringCollection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPStringCollection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWMPStringCollection2 {
    type Vtable = IWMPStringCollection2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMPStringCollection2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46ad648d_53f1_4a74_92e2_2a1b68d63fd4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPStringCollection2_Vtbl {
    pub base__: IWMPStringCollection_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpstringcollection2: *mut ::core::ffi::c_void, pvbool: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    isIdentical: usize,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrvalue: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getAttributeCountByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<::windows::core::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getItemInfoByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<::windows::core::BSTR>, lattributeindex: i32, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getItemInfoByType: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPSubscriptionService(::windows::core::IUnknown);
impl IWMPSubscriptionService {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPlay<'a, P0, P1>(&self, hwnd: P0, pmedia: P1, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).allowPlay)(::windows::core::Vtable::as_raw(self), hwnd.into(), pmedia.into().abi(), ::core::mem::transmute(pfallowplay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowCDBurn<'a, P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).allowCDBurn)(::windows::core::Vtable::as_raw(self), hwnd.into(), pplaylist.into().abi(), ::core::mem::transmute(pfallowburn)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPDATransfer<'a, P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).allowPDATransfer)(::windows::core::Vtable::as_raw(self), hwnd.into(), pplaylist.into().abi(), ::core::mem::transmute(pfallowtransfer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn startBackgroundProcessing<'a, P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).startBackgroundProcessing)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPSubscriptionService, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPSubscriptionService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPSubscriptionService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSubscriptionService {}
impl ::core::fmt::Debug for IWMPSubscriptionService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSubscriptionService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPSubscriptionService {
    type Vtable = IWMPSubscriptionService_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPSubscriptionService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x376055f8_2a59_4a73_9501_dca5273a7a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSubscriptionService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub allowPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: *mut ::core::ffi::c_void, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    allowPlay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub allowCDBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut ::core::ffi::c_void, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    allowCDBurn: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub allowPDATransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut ::core::ffi::c_void, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    allowPDATransfer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub startBackgroundProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    startBackgroundProcessing: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPSubscriptionService2(::windows::core::IUnknown);
impl IWMPSubscriptionService2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPlay<'a, P0, P1>(&self, hwnd: P0, pmedia: P1, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWMPMedia>>,
    {
        (::windows::core::Vtable::vtable(self).base__.allowPlay)(::windows::core::Vtable::as_raw(self), hwnd.into(), pmedia.into().abi(), ::core::mem::transmute(pfallowplay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowCDBurn<'a, P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.allowCDBurn)(::windows::core::Vtable::as_raw(self), hwnd.into(), pplaylist.into().abi(), ::core::mem::transmute(pfallowburn)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn allowPDATransfer<'a, P0, P1>(&self, hwnd: P0, pplaylist: P1, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).base__.allowPDATransfer)(::windows::core::Vtable::as_raw(self), hwnd.into(), pplaylist.into().abi(), ::core::mem::transmute(pfallowtransfer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn startBackgroundProcessing<'a, P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.startBackgroundProcessing)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    pub unsafe fn stopBackgroundProcessing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stopBackgroundProcessing)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn serviceEvent(&self, event: WMPSubscriptionServiceEvent) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).serviceEvent)(::windows::core::Vtable::as_raw(self), event).ok()
    }
    pub unsafe fn deviceAvailable<'a, P0>(&self, bstrdevicename: &::windows::core::BSTR, pcb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSubscriptionServiceCallback>>,
    {
        (::windows::core::Vtable::vtable(self).deviceAvailable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdevicename), pcb.into().abi()).ok()
    }
    pub unsafe fn prepareForSync<'a, P0>(&self, bstrfilename: &::windows::core::BSTR, bstrdevicename: &::windows::core::BSTR, pcb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSubscriptionServiceCallback>>,
    {
        (::windows::core::Vtable::vtable(self).prepareForSync)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfilename), ::core::mem::transmute_copy(bstrdevicename), pcb.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPSubscriptionService2, ::windows::core::IUnknown, IWMPSubscriptionService);
impl ::core::clone::Clone for IWMPSubscriptionService2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPSubscriptionService2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSubscriptionService2 {}
impl ::core::fmt::Debug for IWMPSubscriptionService2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSubscriptionService2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPSubscriptionService2 {
    type Vtable = IWMPSubscriptionService2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPSubscriptionService2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa94c120e_d600_4ec6_b05e_ec9d56d84de0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSubscriptionService2_Vtbl {
    pub base__: IWMPSubscriptionService_Vtbl,
    pub stopBackgroundProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub serviceEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> ::windows::core::HRESULT,
    pub deviceAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pcb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub prepareForSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrdevicename: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pcb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPSubscriptionServiceCallback(::windows::core::IUnknown);
impl IWMPSubscriptionServiceCallback {
    pub unsafe fn onComplete(&self, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).onComplete)(::windows::core::Vtable::as_raw(self), hrresult).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPSubscriptionServiceCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPSubscriptionServiceCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPSubscriptionServiceCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSubscriptionServiceCallback {}
impl ::core::fmt::Debug for IWMPSubscriptionServiceCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSubscriptionServiceCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPSubscriptionServiceCallback {
    type Vtable = IWMPSubscriptionServiceCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPSubscriptionServiceCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd01d127_2dc2_4c3a_876e_63312079f9b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSubscriptionServiceCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub onComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPSyncDevice(::windows::core::IUnknown);
impl IWMPSyncDevice {
    pub unsafe fn friendlyName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).friendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn SetfriendlyName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetfriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn deviceName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).deviceName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn deviceId(&self, pbstrdeviceid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).deviceId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn partnershipIndex(&self, plindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).partnershipIndex)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plindex)).ok()
    }
    pub unsafe fn connected(&self, pvbconnected: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).connected)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvbconnected)).ok()
    }
    pub unsafe fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpds)).ok()
    }
    pub unsafe fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).syncState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpss)).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).progress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plprogress)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn createPartnership(&self, vbshowui: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).createPartnership)(::windows::core::Vtable::as_raw(self), vbshowui).ok()
    }
    pub unsafe fn deletePartnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).deletePartnership)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn showSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).showSettings)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn isIdentical<'a, P0>(&self, pdevice: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).isIdentical)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPSyncDevice, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPSyncDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPSyncDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSyncDevice {}
impl ::core::fmt::Debug for IWMPSyncDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSyncDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPSyncDevice {
    type Vtable = IWMPSyncDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPSyncDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82a2986c_0293_4fd0_b279_b21b86c058be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub friendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetfriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub deviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub deviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub partnershipIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT,
    pub connected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbconnected: *mut i16) -> ::windows::core::HRESULT,
    pub status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::HRESULT,
    pub syncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwmpss: *mut WMPSyncState) -> ::windows::core::HRESULT,
    pub progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT,
    pub getItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pbstrval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub createPartnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbshowui: i16) -> ::windows::core::HRESULT,
    pub deletePartnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub showSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub isIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pvbool: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPSyncDevice2(::windows::core::IUnknown);
impl IWMPSyncDevice2 {
    pub unsafe fn friendlyName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.friendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn SetfriendlyName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetfriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn deviceName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.deviceName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn deviceId(&self, pbstrdeviceid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.deviceId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn partnershipIndex(&self, plindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.partnershipIndex)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plindex)).ok()
    }
    pub unsafe fn connected(&self, pvbconnected: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.connected)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvbconnected)).ok()
    }
    pub unsafe fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpds)).ok()
    }
    pub unsafe fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.syncState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpss)).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.progress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plprogress)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn createPartnership(&self, vbshowui: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.createPartnership)(::windows::core::Vtable::as_raw(self), vbshowui).ok()
    }
    pub unsafe fn deletePartnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.deletePartnership)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn showSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.showSettings)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn isIdentical<'a, P0>(&self, pdevice: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.isIdentical)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute_copy(bstrval)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPSyncDevice2, ::windows::core::IUnknown, IWMPSyncDevice);
impl ::core::clone::Clone for IWMPSyncDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPSyncDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSyncDevice2 {}
impl ::core::fmt::Debug for IWMPSyncDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSyncDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPSyncDevice2 {
    type Vtable = IWMPSyncDevice2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPSyncDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88afb4b2_140a_44d2_91e6_4543da467cd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice2_Vtbl {
    pub base__: IWMPSyncDevice_Vtbl,
    pub setItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrval: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPSyncDevice3(::windows::core::IUnknown);
impl IWMPSyncDevice3 {
    pub unsafe fn friendlyName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.friendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn SetfriendlyName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetfriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn deviceName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.deviceName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn deviceId(&self, pbstrdeviceid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.deviceId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn partnershipIndex(&self, plindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.partnershipIndex)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plindex)).ok()
    }
    pub unsafe fn connected(&self, pvbconnected: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.connected)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvbconnected)).ok()
    }
    pub unsafe fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.status)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpds)).ok()
    }
    pub unsafe fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.syncState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwmpss)).ok()
    }
    pub unsafe fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.progress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plprogress)).ok()
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.getItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute(pbstrval)).ok()
    }
    pub unsafe fn createPartnership(&self, vbshowui: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.createPartnership)(::windows::core::Vtable::as_raw(self), vbshowui).ok()
    }
    pub unsafe fn deletePartnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.deletePartnership)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn showSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.showSettings)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn isIdentical<'a, P0>(&self, pdevice: P0, pvbool: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPSyncDevice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.isIdentical)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(pvbool)).ok()
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.setItemInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemname), ::core::mem::transmute_copy(bstrval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn estimateSyncSize<'a, P0, P1>(&self, pnonruleplaylist: P0, prulesplaylist: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWMPPlaylist>>,
    {
        (::windows::core::Vtable::vtable(self).estimateSyncSize)(::windows::core::Vtable::as_raw(self), pnonruleplaylist.into().abi(), prulesplaylist.into().abi()).ok()
    }
    pub unsafe fn cancelEstimation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).cancelEstimation)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPSyncDevice3, ::windows::core::IUnknown, IWMPSyncDevice, IWMPSyncDevice2);
impl ::core::clone::Clone for IWMPSyncDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPSyncDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSyncDevice3 {}
impl ::core::fmt::Debug for IWMPSyncDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSyncDevice3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPSyncDevice3 {
    type Vtable = IWMPSyncDevice3_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPSyncDevice3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22c85f9_263c_4372_a0da_b518db9b4098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice3_Vtbl {
    pub base__: IWMPSyncDevice2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub estimateSyncSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnonruleplaylist: *mut ::core::ffi::c_void, prulesplaylist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    estimateSyncSize: usize,
    pub cancelEstimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPSyncServices(::windows::core::IUnknown);
impl IWMPSyncServices {
    pub unsafe fn deviceCount(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).deviceCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    pub unsafe fn getDevice(&self, lindex: i32) -> ::windows::core::Result<IWMPSyncDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getDevice)(::windows::core::Vtable::as_raw(self), lindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMPSyncDevice>(result__)
    }
}
::windows::core::interface_hierarchy!(IWMPSyncServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPSyncServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPSyncServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPSyncServices {}
impl ::core::fmt::Debug for IWMPSyncServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPSyncServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPSyncServices {
    type Vtable = IWMPSyncServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPSyncServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b5050ff_e0a4_4808_b3a8_893a9e1ed894);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub deviceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub getDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPTranscodePolicy(::windows::core::IUnknown);
impl IWMPTranscodePolicy {
    pub unsafe fn allowTranscode(&self, pvballow: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).allowTranscode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvballow)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPTranscodePolicy, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPTranscodePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPTranscodePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPTranscodePolicy {}
impl ::core::fmt::Debug for IWMPTranscodePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPTranscodePolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPTranscodePolicy {
    type Vtable = IWMPTranscodePolicy_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPTranscodePolicy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb64cbac3_401c_4327_a3e8_b9feb3a8c25c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPTranscodePolicy_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub allowTranscode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvballow: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPUserEventSink(::windows::core::IUnknown);
impl IWMPUserEventSink {
    pub unsafe fn NotifyUserEvent(&self, eventcode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).NotifyUserEvent)(::windows::core::Vtable::as_raw(self), eventcode).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPUserEventSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPUserEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPUserEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPUserEventSink {}
impl ::core::fmt::Debug for IWMPUserEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPUserEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPUserEventSink {
    type Vtable = IWMPUserEventSink_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPUserEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfccfa72_c343_48c3_a2de_b7a4402e39f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPUserEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub NotifyUserEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcode: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPVideoRenderConfig(::windows::core::IUnknown);
impl IWMPVideoRenderConfig {
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn SetpresenterActivate<'a, P0>(&self, pactivate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::MediaFoundation::IMFActivate>>,
    {
        (::windows::core::Vtable::vtable(self).SetpresenterActivate)(::windows::core::Vtable::as_raw(self), pactivate.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPVideoRenderConfig, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPVideoRenderConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPVideoRenderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPVideoRenderConfig {}
impl ::core::fmt::Debug for IWMPVideoRenderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPVideoRenderConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPVideoRenderConfig {
    type Vtable = IWMPVideoRenderConfig_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPVideoRenderConfig {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d6cf803_1ec0_4c8d_b3ca_f18e27282074);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPVideoRenderConfig_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub SetpresenterActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    SetpresenterActivate: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IWMPWindowMessageSink(::windows::core::IUnknown);
impl IWMPWindowMessageSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessage<'a, P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).OnWindowMessage)(::windows::core::Vtable::as_raw(self), umsg, wparam.into(), lparam.into(), ::core::mem::transmute(plret), ::core::mem::transmute(pfhandled)).ok()
    }
}
::windows::core::interface_hierarchy!(IWMPWindowMessageSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWMPWindowMessageSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPWindowMessageSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPWindowMessageSink {}
impl ::core::fmt::Debug for IWMPWindowMessageSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPWindowMessageSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWMPWindowMessageSink {
    type Vtable = IWMPWindowMessageSink_Vtbl;
}
unsafe impl ::windows::core::Interface for IWMPWindowMessageSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a0daa30_908d_4789_ba87_aed879b5c49b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPWindowMessageSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowMessage: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeed(::windows::core::IUnknown);
impl IXFeed {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Xml)(::windows::core::Vtable::as_raw(self), uiitemcount, sortproperty, sortorder, filterflags, includeflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Rename<'a, P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Rename)(::windows::core::Vtable::as_raw(self), pszname.into()).ok()
    }
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Url)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetUrl<'a, P0>(&self, pszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetUrl)(::windows::core::Vtable::as_raw(self), pszurl.into()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Move<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn Parent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastWriteTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastWriteTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Download)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelAsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows::core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SyncSetting)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_SYNC_SETTING>(result__)
    }
    pub unsafe fn SetSyncSetting(&self, fss: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSyncSetting)(::windows::core::Vtable::as_raw(self), fss).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Interval)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetInterval(&self, uiinterval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInterval)(::windows::core::Vtable::as_raw(self), uiinterval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalEnclosurePath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Items(&self) -> ::windows::core::Result<IXFeedsEnum> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Items)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IXFeedsEnum>(result__)
    }
    pub unsafe fn GetItem<T>(&self, uiid: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetItem)(::windows::core::Vtable::as_raw(self), uiid, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MarkAllItemsRead)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, uimaxitemcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxItemCount)(::windows::core::Vtable::as_raw(self), uimaxitemcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDownloadEnclosuresAutomatically<'a, P0>(&self, bdownloadenclosuresautomatically: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetDownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), bdownloadenclosuresautomatically.into()).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_STATUS>(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastDownloadError)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_ERROR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Merge<'a, P0, P1>(&self, pstream: P0, pszurl: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Merge)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), pszurl.into()).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Title)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Link)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Image(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Image)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastBuildDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastBuildDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PubDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Ttl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Language(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Language)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Copyright)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsList(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetWatcher<T>(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetWatcher)(::windows::core::Vtable::as_raw(self), scope, mask, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UnreadItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
::windows::core::interface_hierarchy!(IXFeed, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXFeed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeed {}
impl ::core::fmt::Debug for IXFeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeed").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeed {
    type Vtable = IXFeed_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeed {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa44179a4_e0f6_403b_af8d_d080f425a451);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeed_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Xml: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastWriteTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastWriteTime: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelAsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SyncSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT,
    pub SetSyncSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastDownloadTime: usize,
    pub LocalEnclosurePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MarkAllItemsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puimaxitemcount: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uimaxitemcount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DownloadEnclosuresAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DownloadEnclosuresAutomatically: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDownloadEnclosuresAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDownloadEnclosuresAutomatically: usize,
    pub DownloadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT,
    pub LastDownloadError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Merge: usize,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztitle: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszhomepage: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszimageurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastBuildDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastBuildDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PubDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PubDate: usize,
    pub Ttl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puittl: *mut u32) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszlanguage: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Copyright: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcopyright: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsList: usize,
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnreadItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiunreaditemcount: *mut u32) -> ::windows::core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiitemcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeed2(::windows::core::IUnknown);
impl IXFeed2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Xml)(::windows::core::Vtable::as_raw(self), uiitemcount, sortproperty, sortorder, filterflags, includeflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Rename<'a, P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Rename)(::windows::core::Vtable::as_raw(self), pszname.into()).ok()
    }
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Url)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetUrl<'a, P0>(&self, pszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUrl)(::windows::core::Vtable::as_raw(self), pszurl.into()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Move<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn Parent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastWriteTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastWriteTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Download(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Download)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelAsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SyncSetting(&self) -> ::windows::core::Result<FEEDS_SYNC_SETTING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SyncSetting)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_SYNC_SETTING>(result__)
    }
    pub unsafe fn SetSyncSetting(&self, fss: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSyncSetting)(::windows::core::Vtable::as_raw(self), fss).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Interval)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetInterval(&self, uiinterval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInterval)(::windows::core::Vtable::as_raw(self), uiinterval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn LocalEnclosurePath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalEnclosurePath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Items(&self) -> ::windows::core::Result<IXFeedsEnum> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Items)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IXFeedsEnum>(result__)
    }
    pub unsafe fn GetItem<T>(&self, uiid: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), uiid, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn MarkAllItemsRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MarkAllItemsRead)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MaxItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MaxItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxItemCount(&self, uimaxitemcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaxItemCount)(::windows::core::Vtable::as_raw(self), uimaxitemcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadEnclosuresAutomatically(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDownloadEnclosuresAutomatically<'a, P0>(&self, bdownloadenclosuresautomatically: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDownloadEnclosuresAutomatically)(::windows::core::Vtable::as_raw(self), bdownloadenclosuresautomatically.into()).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_STATUS>(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadError)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_ERROR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Merge<'a, P0, P1>(&self, pstream: P0, pszurl: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Merge)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), pszurl.into()).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Link)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Image(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Image)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastBuildDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastBuildDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PubDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn Ttl(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Ttl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Language(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Language)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Copyright(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Copyright)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsList(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetWatcher<T>(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).base__.GetWatcher)(::windows::core::Vtable::as_raw(self), scope, mask, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn UnreadItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UnreadItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetItemByEffectiveId<T>(&self, uieffectiveid: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetItemByEffectiveId)(::windows::core::Vtable::as_raw(self), uieffectiveid, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastItemDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastItemDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn Username(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Username)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Password(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Password)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetCredentials<'a, P0, P1>(&self, pszusername: P0, pszpassword: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetCredentials)(::windows::core::Vtable::as_raw(self), pszusername.into(), pszpassword.into()).ok()
    }
    pub unsafe fn ClearCredentials(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ClearCredentials)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IXFeed2, ::windows::core::IUnknown, IXFeed);
impl ::core::clone::Clone for IXFeed2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeed2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeed2 {}
impl ::core::fmt::Debug for IXFeed2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeed2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeed2 {
    type Vtable = IXFeed2_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeed2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce528e77_3716_4eb7_956d_f5e37502e12a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeed2_Vtbl {
    pub base__: IXFeed_Vtbl,
    pub GetItemByEffectiveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uieffectiveid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastItemDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastItemDownloadTime: usize,
    pub Username: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszusername: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszusername: ::windows::core::PCWSTR, pszpassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ClearCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeedEnclosure(::windows::core::IUnknown);
impl IXFeedEnclosure {
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Url)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Length(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Length)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn AsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelAsyncDownload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelAsyncDownload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_STATUS>(result__)
    }
    pub unsafe fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastDownloadError)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_DOWNLOAD_ERROR>(result__)
    }
    pub unsafe fn LocalPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Parent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn DownloadMimeType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadMimeType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn RemoveFile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveFile)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetFile<'a, P0, P1, P2, P3>(&self, pszdownloadurl: P0, pszdownloadfilepath: P1, pszdownloadmimetype: P2, pszenclosurefilename: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetFile)(::windows::core::Vtable::as_raw(self), pszdownloadurl.into(), pszdownloadfilepath.into(), pszdownloadmimetype.into(), pszenclosurefilename.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IXFeedEnclosure, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXFeedEnclosure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeedEnclosure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedEnclosure {}
impl ::core::fmt::Debug for IXFeedEnclosure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedEnclosure").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeedEnclosure {
    type Vtable = IXFeedEnclosure_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeedEnclosure {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfbfb953_644f_4792_b69c_dfaca4cbf89a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedEnclosure_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszmimetype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puilength: *mut u32) -> ::windows::core::HRESULT,
    pub AsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelAsyncDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DownloadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT,
    pub LastDownloadError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT,
    pub LocalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub DownloadMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszmimetype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub RemoveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdownloadurl: ::windows::core::PCWSTR, pszdownloadfilepath: ::windows::core::PCWSTR, pszdownloadmimetype: ::windows::core::PCWSTR, pszenclosurefilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeedEvents(::windows::core::IUnknown);
impl IXFeedEvents {
    pub unsafe fn Error(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FeedDeleted<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedDeleted)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FeedRenamed<'a, P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedRenamed)(::windows::core::Vtable::as_raw(self), pszpath.into(), pszoldpath.into()).ok()
    }
    pub unsafe fn FeedUrlChanged<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedUrlChanged)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FeedMoved<'a, P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedMoved)(::windows::core::Vtable::as_raw(self), pszpath.into(), pszoldpath.into()).ok()
    }
    pub unsafe fn FeedDownloading<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedDownloading)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FeedDownloadCompleted<'a, P0>(&self, pszpath: P0, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedDownloadCompleted)(::windows::core::Vtable::as_raw(self), pszpath.into(), fde).ok()
    }
    pub unsafe fn FeedItemCountChanged<'a, P0>(&self, pszpath: P0, feicfflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedItemCountChanged)(::windows::core::Vtable::as_raw(self), pszpath.into(), feicfflags).ok()
    }
}
::windows::core::interface_hierarchy!(IXFeedEvents, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXFeedEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeedEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedEvents {}
impl ::core::fmt::Debug for IXFeedEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeedEvents {
    type Vtable = IXFeedEvents_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeedEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1630852e_1263_465b_98e5_fe60ffec4ac2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FeedDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedUrlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedDownloading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT,
    pub FeedItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeedFolder(::windows::core::IUnknown);
impl IXFeedFolder {
    pub unsafe fn Feeds(&self) -> ::windows::core::Result<IXFeedsEnum> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Feeds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IXFeedsEnum>(result__)
    }
    pub unsafe fn Subfolders(&self) -> ::windows::core::Result<IXFeedsEnum> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Subfolders)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IXFeedsEnum>(result__)
    }
    pub unsafe fn CreateFeed<'a, P0, P1, T>(&self, pszname: P0, pszurl: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).CreateFeed)(::windows::core::Vtable::as_raw(self), pszname.into(), pszurl.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateSubfolder<'a, P0, T>(&self, pszname: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).CreateSubfolder)(::windows::core::Vtable::as_raw(self), pszname.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFeed<'a, P0>(&self, pszname: P0, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).ExistsFeed)(::windows::core::Vtable::as_raw(self), pszname.into(), ::core::mem::transmute(pbfeedexists)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsSubfolder<'a, P0>(&self, pszname: P0, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).ExistsSubfolder)(::windows::core::Vtable::as_raw(self), pszname.into(), ::core::mem::transmute(pbsubfolderexists)).ok()
    }
    pub unsafe fn GetFeed<'a, P0, T>(&self, pszname: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetFeed)(::windows::core::Vtable::as_raw(self), pszname.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetSubfolder<'a, P0, T>(&self, pszname: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetSubfolder)(::windows::core::Vtable::as_raw(self), pszname.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Rename<'a, P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Rename)(::windows::core::Vtable::as_raw(self), pszname.into()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Move<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn Parent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRoot(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsRoot)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetWatcher<T>(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetWatcher)(::windows::core::Vtable::as_raw(self), scope, mask, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn TotalUnreadItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalUnreadItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn TotalItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalItemCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
::windows::core::interface_hierarchy!(IXFeedFolder, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXFeedFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeedFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedFolder {}
impl ::core::fmt::Debug for IXFeedFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedFolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeedFolder {
    type Vtable = IXFeedFolder_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeedFolder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c963678_3a51_4b88_8531_98b90b6508f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedFolder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Feeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Subfolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pszurl: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFeed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsSubfolder: usize,
    pub GetFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSubfolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRoot: usize,
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TotalUnreadItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> ::windows::core::HRESULT,
    pub TotalItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puitotalitemcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeedFolderEvents(::windows::core::IUnknown);
impl IXFeedFolderEvents {
    pub unsafe fn Error(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FolderAdded<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FolderAdded)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FolderDeleted<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FolderDeleted)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FolderRenamed<'a, P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FolderRenamed)(::windows::core::Vtable::as_raw(self), pszpath.into(), pszoldpath.into()).ok()
    }
    pub unsafe fn FolderMovedFrom<'a, P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FolderMovedFrom)(::windows::core::Vtable::as_raw(self), pszpath.into(), pszoldpath.into()).ok()
    }
    pub unsafe fn FolderMovedTo<'a, P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FolderMovedTo)(::windows::core::Vtable::as_raw(self), pszpath.into(), pszoldpath.into()).ok()
    }
    pub unsafe fn FolderItemCountChanged<'a, P0>(&self, pszpath: P0, feicfflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FolderItemCountChanged)(::windows::core::Vtable::as_raw(self), pszpath.into(), feicfflags).ok()
    }
    pub unsafe fn FeedAdded<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedAdded)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FeedDeleted<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedDeleted)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FeedRenamed<'a, P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedRenamed)(::windows::core::Vtable::as_raw(self), pszpath.into(), pszoldpath.into()).ok()
    }
    pub unsafe fn FeedUrlChanged<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedUrlChanged)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FeedMovedFrom<'a, P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedMovedFrom)(::windows::core::Vtable::as_raw(self), pszpath.into(), pszoldpath.into()).ok()
    }
    pub unsafe fn FeedMovedTo<'a, P0, P1>(&self, pszpath: P0, pszoldpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedMovedTo)(::windows::core::Vtable::as_raw(self), pszpath.into(), pszoldpath.into()).ok()
    }
    pub unsafe fn FeedDownloading<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedDownloading)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn FeedDownloadCompleted<'a, P0>(&self, pszpath: P0, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedDownloadCompleted)(::windows::core::Vtable::as_raw(self), pszpath.into(), fde).ok()
    }
    pub unsafe fn FeedItemCountChanged<'a, P0>(&self, pszpath: P0, feicfflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).FeedItemCountChanged)(::windows::core::Vtable::as_raw(self), pszpath.into(), feicfflags).ok()
    }
}
::windows::core::interface_hierarchy!(IXFeedFolderEvents, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXFeedFolderEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeedFolderEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedFolderEvents {}
impl ::core::fmt::Debug for IXFeedFolderEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedFolderEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeedFolderEvents {
    type Vtable = IXFeedFolderEvents_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeedFolderEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7964b769_234a_4bb1_a5f4_90454c8ad07e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedFolderEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FolderAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FolderDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FolderRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FolderMovedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FolderMovedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FolderItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::HRESULT,
    pub FeedAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedUrlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedMovedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedMovedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedDownloading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub FeedDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT,
    pub FeedItemCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeedItem(::windows::core::IUnknown);
impl IXFeedItem {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, fxif: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Xml)(::windows::core::Vtable::as_raw(self), fxif, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Title)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Link)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Guid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PubDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Comments)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Author(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Author)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Enclosure<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).Enclosure)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRead(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsRead)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRead<'a, P0>(&self, bisread: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsRead)(::windows::core::Vtable::as_raw(self), bisread.into()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Parent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Modified(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Modified)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
}
::windows::core::interface_hierarchy!(IXFeedItem, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXFeedItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedItem {}
impl ::core::fmt::Debug for IXFeedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeedItem {
    type Vtable = IXFeedItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeedItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe757b2f5_e73e_434e_a1bf_2bd7c3e60fcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Xml: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztitle: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszguid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PubDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PubDate: usize,
    pub Comments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszauthor: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Enclosure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRead: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsRead: usize,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiid: *mut u32) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastDownloadTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Modified: usize,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeedItem2(::windows::core::IUnknown);
impl IXFeedItem2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Xml(&self, fxif: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Xml)(::windows::core::Vtable::as_raw(self), fxif, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Link(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Link)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Guid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PubDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PubDate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn Comments(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Comments)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Author(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Author)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Enclosure<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).base__.Enclosure)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRead(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRead)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRead<'a, P0>(&self, bisread: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIsRead)(::windows::core::Vtable::as_raw(self), bisread.into()).ok()
    }
    pub unsafe fn LocalId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Parent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DownloadUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastDownloadTime)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Modified(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Modified)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn EffectiveId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EffectiveId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
::windows::core::interface_hierarchy!(IXFeedItem2, ::windows::core::IUnknown, IXFeedItem);
impl ::core::clone::Clone for IXFeedItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeedItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedItem2 {}
impl ::core::fmt::Debug for IXFeedItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedItem2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeedItem2 {
    type Vtable = IXFeedItem2_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeedItem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cda2dc7_9013_4522_9970_2a9dd9ead5a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedItem2_Vtbl {
    pub base__: IXFeedItem_Vtbl,
    pub EffectiveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puieffectiveid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeedsEnum(::windows::core::IUnknown);
impl IXFeedsEnum {
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Item<T>(&self, uiindex: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).Item)(::windows::core::Vtable::as_raw(self), uiindex, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
::windows::core::interface_hierarchy!(IXFeedsEnum, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXFeedsEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeedsEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedsEnum {}
impl ::core::fmt::Debug for IXFeedsEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedsEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeedsEnum {
    type Vtable = IXFeedsEnum_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeedsEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc43a9d5_5015_4301_8c96_a47434b4d658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedsEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
pub struct IXFeedsManager(::windows::core::IUnknown);
impl IXFeedsManager {
    pub unsafe fn RootFolder<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).RootFolder)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, P0>(&self, pszurl: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsSubscribed)(::windows::core::Vtable::as_raw(self), pszurl.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFeed<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExistsFeed)(::windows::core::Vtable::as_raw(self), pszpath.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetFeed<'a, P0, T>(&self, pszpath: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetFeed)(::windows::core::Vtable::as_raw(self), pszpath.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetFeedByUrl<'a, P0, T>(&self, pszurl: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetFeedByUrl)(::windows::core::Vtable::as_raw(self), pszurl.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistsFolder<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExistsFolder)(::windows::core::Vtable::as_raw(self), pszpath.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetFolder<'a, P0, T>(&self, pszpath: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetFolder)(::windows::core::Vtable::as_raw(self), pszpath.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn DeleteFeed<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).DeleteFeed)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn DeleteFolder<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).DeleteFolder)(::windows::core::Vtable::as_raw(self), pszpath.into()).ok()
    }
    pub unsafe fn BackgroundSync(&self, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BackgroundSync)(::windows::core::Vtable::as_raw(self), fbsa).ok()
    }
    pub unsafe fn BackgroundSyncStatus(&self) -> ::windows::core::Result<FEEDS_BACKGROUNDSYNC_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BackgroundSyncStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FEEDS_BACKGROUNDSYNC_STATUS>(result__)
    }
    pub unsafe fn DefaultInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DefaultInterval)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetDefaultInterval(&self, uiinterval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultInterval)(::windows::core::Vtable::as_raw(self), uiinterval).ok()
    }
    pub unsafe fn AsyncSyncAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AsyncSyncAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Normalize<'a, P0>(&self, pstreamin: P0) -> ::windows::core::Result<super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Normalize)(::windows::core::Vtable::as_raw(self), pstreamin.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn ItemCountLimit(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ItemCountLimit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
::windows::core::interface_hierarchy!(IXFeedsManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXFeedsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXFeedsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXFeedsManager {}
impl ::core::fmt::Debug for IXFeedsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXFeedsManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXFeedsManager {
    type Vtable = IXFeedsManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IXFeedsManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5357e238_fb12_4aca_a930_cab7832b84bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXFeedsManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RootFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSubscribed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSubscribed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFeed: usize,
    pub GetFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFeedByUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExistsFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExistsFolder: usize,
    pub GetFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub DeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub BackgroundSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT,
    pub BackgroundSyncStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT,
    pub DefaultInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT,
    pub SetDefaultInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT,
    pub AsyncSyncAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstreamin: *mut ::core::ffi::c_void, ppstreamout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Normalize: usize,
    pub ItemCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiitemcountlimit: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _WMPOCXEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _WMPOCXEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(_WMPOCXEvents, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _WMPOCXEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _WMPOCXEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _WMPOCXEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _WMPOCXEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_WMPOCXEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for _WMPOCXEvents {
    type Vtable = _WMPOCXEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _WMPOCXEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf52a51_394a_11d3_b153_00c04f79faa6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _WMPOCXEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
pub const CLSID_WMPMediaPluginRegistrar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5569e7f5_424b_4b93_89ca_79d17924689a);
pub const CLSID_WMPSkinManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2a7fd52_301f_4348_b93a_638c6de49229);
pub const CLSID_XFeedsManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe6b11c3_c72e_4061_86c6_9d163121f229);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_DELTA: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_AsyncDownload: u32 = 24579u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_CancelAsyncDownload: u32 = 24580u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_DownloadMimeType: u32 = 24586u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_DownloadStatus: u32 = 24581u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_DownloadUrl: u32 = 24585u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_LastDownloadError: u32 = 24582u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_Length: u32 = 24578u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_LocalPath: u32 = 24583u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_Parent: u32 = 24584u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_RemoveFile: u32 = 24587u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_SetFile: u32 = 24588u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_Type: u32 = 24577u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDENCLOSURE_Url: u32 = 24576u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDEVENTS_Error: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDEVENTS_FeedDeleted: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDEVENTS_FeedDownloadCompleted: u32 = 32774u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDEVENTS_FeedDownloading: u32 = 32773u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDEVENTS_FeedItemCountChanged: u32 = 32775u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDEVENTS_FeedMoved: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDEVENTS_FeedRenamed: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDEVENTS_FeedUrlChanged: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_Error: u32 = 28672u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedAdded: u32 = 28679u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedDeleted: u32 = 28680u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedDownloadCompleted: u32 = 28686u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedDownloading: u32 = 28685u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedItemCountChanged: u32 = 28687u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedMovedFrom: u32 = 28683u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedMovedTo: u32 = 28684u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedRenamed: u32 = 28681u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FeedUrlChanged: u32 = 28682u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FolderAdded: u32 = 28673u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FolderDeleted: u32 = 28674u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FolderItemCountChanged: u32 = 28678u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FolderMovedFrom: u32 = 28676u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FolderMovedTo: u32 = 28677u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDEREVENTS_FolderRenamed: u32 = 28675u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_CreateFeed: u32 = 12290u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_CreateSubfolder: u32 = 12291u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_Delete: u32 = 12296u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_ExistsFeed: u32 = 12292u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_ExistsSubfolder: u32 = 12294u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_Feeds: u32 = 12288u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_GetFeed: u32 = 12293u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_GetSubfolder: u32 = 12295u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_GetWatcher: u32 = 12305u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_IsRoot: u32 = 12302u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_Move: u32 = 12300u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_Name: u32 = 12297u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_Parent: u32 = 12301u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_Path: u32 = 12299u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_Rename: u32 = 12298u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_Subfolders: u32 = 12289u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_TotalItemCount: u32 = 12304u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDFOLDER_TotalUnreadItemCount: u32 = 12303u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Author: u32 = 20487u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Comments: u32 = 20486u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Delete: u32 = 20492u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Description: u32 = 20484u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_DownloadUrl: u32 = 20493u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_EffectiveId: u32 = 20496u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Enclosure: u32 = 20488u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Guid: u32 = 20483u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_IsRead: u32 = 20489u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_LastDownloadTime: u32 = 20494u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Link: u32 = 20482u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_LocalId: u32 = 20490u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Modified: u32 = 20495u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Parent: u32 = 20491u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_PubDate: u32 = 20485u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Title: u32 = 20481u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDITEM_Xml: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDSENUM_Count: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDSENUM_Item: u32 = 8193u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_AsyncSyncAll: u32 = 4108u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_BackgroundSync: u32 = 4105u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_BackgroundSyncStatus: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_DefaultInterval: u32 = 4107u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_DeleteFeed: u32 = 4102u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_DeleteFolder: u32 = 4103u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_ExistsFeed: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_ExistsFolder: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_GetFeed: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_GetFeedByUrl: u32 = 4104u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_GetFolder: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_IsSubscribed: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_ItemCountLimit: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_Normalize: u32 = 4109u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEEDS_RootFolder: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_AsyncDownload: u32 = 16395u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_CancelAsyncDownload: u32 = 16396u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_ClearCredentials: u32 = 16428u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Copyright: u32 = 16411u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Delete: u32 = 16393u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Description: u32 = 16404u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Download: u32 = 16394u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_DownloadEnclosuresAutomatically: u32 = 16412u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_DownloadStatus: u32 = 16413u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_DownloadUrl: u32 = 16416u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_GetItem: u32 = 16402u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_GetItemByEffectiveId: u32 = 16423u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_GetWatcher: u32 = 16419u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Image: u32 = 16406u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Interval: u32 = 16397u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_IsList: u32 = 16417u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_ItemCount: u32 = 16421u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Items: u32 = 16401u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Language: u32 = 16410u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_LastBuildDate: u32 = 16407u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_LastDownloadError: u32 = 16414u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_LastDownloadTime: u32 = 16399u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_LastItemDownloadTime: u32 = 16424u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_LastWriteTime: u32 = 16392u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Link: u32 = 16405u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_LocalEnclosurePath: u32 = 16400u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_LocalId: u32 = 16388u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_MarkAllItemsRead: u32 = 16418u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_MaxItemCount: u32 = 16422u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Merge: u32 = 16415u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Move: u32 = 16390u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Name: u32 = 16385u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Parent: u32 = 16391u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Password: u32 = 16426u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Path: u32 = 16389u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_PubDate: u32 = 16408u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Rename: u32 = 16386u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_SetCredentials: u32 = 16427u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_SyncSetting: u32 = 16398u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Title: u32 = 16403u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Ttl: u32 = 16409u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_UnreadItemCount: u32 = 16420u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Url: u32 = 16387u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Username: u32 = 16425u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_FEED_Xml: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROMCOLLECTION_BASE: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROMCOLLECTION_COUNT: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROMCOLLECTION_GETBYDRIVESPECIFIER: u32 = 303u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROMCOLLECTION_ITEM: u32 = 302u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROMCOLLECTION_STARTMONITORINGCDROMS: u32 = 304u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROMCOLLECTION_STOPMONITORINGCDROMS: u32 = 305u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROM_BASE: u32 = 250u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROM_DRIVESPECIFIER: u32 = 251u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROM_EJECT: u32 = 253u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCDROM_PLAYLIST: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION2_GETLANGCOUNT: u32 = 955u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION2_GETLANGID: u32 = 957u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION2_GETLANGNAME: u32 = 956u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION2_GETSTYLECOUNT: u32 = 958u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION2_GETSTYLENAME: u32 = 959u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION_BASE: u32 = 950u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION_CAPTIONINGID: u32 = 954u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION_SAMIFILENAME: u32 = 953u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION_SAMILANG: u32 = 952u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCLOSEDCAPTION_SAMISTYLE: u32 = 951u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS2_STEP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS3_AUDIOLANGUAGECOUNT: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS3_CURRENTAUDIOLANGUAGE: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS3_CURRENTAUDIOLANGUAGEINDEX: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS3_CURRENTPOSITIONTIMECODE: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS3_GETAUDIOLANGUAGEDESC: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS3_GETAUDIOLANGUAGEID: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS3_GETLANGUAGENAME: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLSFAKE_TIMECOMPRESSION: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_BASE: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_CURRENTITEM: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_CURRENTMARKER: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_CURRENTPOSITION: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_CURRENTPOSITIONSTRING: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_FASTFORWARD: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_FASTREVERSE: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_ISAVAILABLE: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_NEXT: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_PAUSE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_PLAY: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_PLAYITEM: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_PREVIOUS: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCONTROLS_STOP: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE2_BASE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE2_DVD: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE3_NEWMEDIA: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE3_NEWPLAYLIST: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_AUDIOLANGUAGECHANGE: u32 = 5102u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_BUFFERING: u32 = 5402u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_CDROMMEDIACHANGE: u32 = 5701u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_CURRENTITEMCHANGE: u32 = 5806u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_CURRENTMEDIAITEMAVAILABLE: u32 = 5803u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_CURRENTPLAYLISTCHANGE: u32 = 5804u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_CURRENTPLAYLISTITEMAVAILABLE: u32 = 5805u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_DISCONNECT: u32 = 5401u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_DOMAINCHANGE: u32 = 5822u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_DURATIONUNITCHANGE: u32 = 5204u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_ENDOFSTREAM: u32 = 5201u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_ERROR: u32 = 5501u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MARKERHIT: u32 = 5203u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACHANGE: u32 = 5802u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONATTRIBUTESTRINGADDED: u32 = 5808u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONATTRIBUTESTRINGCHANGED: u32 = 5820u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONATTRIBUTESTRINGREMOVED: u32 = 5809u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONCHANGE: u32 = 5807u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONCONTENTSCANADDEDITEM: u32 = 5813u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONCONTENTSCANPROGRESS: u32 = 5814u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONMEDIAADDED: u32 = 5825u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONMEDIAREMOVED: u32 = 5826u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONSEARCHCOMPLETE: u32 = 5817u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONSEARCHFOUNDITEM: u32 = 5815u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIACOLLECTIONSEARCHPROGRESS: u32 = 5816u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MEDIAERROR: u32 = 5821u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_MODECHANGE: u32 = 5819u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_NEWSTREAM: u32 = 5403u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_OPENPLAYLISTSWITCH: u32 = 5823u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_OPENSTATECHANGE: u32 = 5001u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_PLAYLISTCHANGE: u32 = 5801u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_PLAYLISTCOLLECTIONCHANGE: u32 = 5810u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_PLAYLISTCOLLECTIONPLAYLISTADDED: u32 = 5811u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_PLAYLISTCOLLECTIONPLAYLISTREMOVED: u32 = 5812u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_PLAYLISTCOLLECTIONPLAYLISTSETASDELETED: u32 = 5818u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_PLAYSTATECHANGE: u32 = 5101u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_POSITIONCHANGE: u32 = 5202u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_SCRIPTCOMMAND: u32 = 5301u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_STATUSCHANGE: u32 = 5002u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_STRINGCOLLECTIONCHANGE: u32 = 5824u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCOREEVENT_WARNING: u32 = 5601u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_CDROMCOLLECTION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_CLOSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_CLOSEDCAPTION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_CONTROLS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_CURRENTMEDIA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_CURRENTPLAYLIST: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_ERROR: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_ISONLINE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_LAST: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_LAUNCHURL: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_MAX: u32 = 1454u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_MEDIACOLLECTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_NETWORK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_OPENSTATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_PLAYLISTCOLLECTION: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_PLAYSTATE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_SETTINGS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_STATUS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_URL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPCORE_VERSIONINFO: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADCOLLECTION_BASE: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADCOLLECTION_CLEAR: u32 = 1206u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADCOLLECTION_COUNT: u32 = 1202u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADCOLLECTION_ID: u32 = 1201u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADCOLLECTION_ITEM: u32 = 1203u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADCOLLECTION_REMOVEITEM: u32 = 1205u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADCOLLECTION_STARTDOWNLOAD: u32 = 1204u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM2_BASE: u32 = 1300u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM2_GETITEMINFO: u32 = 1301u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_BASE: u32 = 1250u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_CANCEL: u32 = 1258u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_DOWNLOADSTATE: u32 = 1255u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_PAUSE: u32 = 1256u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_PROGRESS: u32 = 1254u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_RESUME: u32 = 1257u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_SIZE: u32 = 1252u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_SOURCEURL: u32 = 1251u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADITEM_TYPE: u32 = 1253u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADMANAGER_BASE: u32 = 1150u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADMANAGER_CREATEDOWNLOADCOLLECTION: u32 = 1152u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDOWNLOADMANAGER_GETDOWNLOADCOLLECTION: u32 = 1151u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDVD_BACK: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDVD_BASE: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDVD_DOMAIN: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDVD_ISAVAILABLE: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDVD_RESUME: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDVD_TITLEMENU: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPDVD_TOPMENU: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERRORITEM2_CONDITION: u32 = 906u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERRORITEM_BASE: u32 = 900u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERRORITEM_CUSTOMURL: u32 = 905u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERRORITEM_ERRORCODE: u32 = 901u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERRORITEM_ERRORCONTEXT: u32 = 903u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERRORITEM_ERRORDESCRIPTION: u32 = 902u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERRORITEM_REMEDY: u32 = 904u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERROR_BASE: u32 = 850u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERROR_CLEARERRORQUEUE: u32 = 851u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERROR_ERRORCOUNT: u32 = 852u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERROR_ITEM: u32 = 853u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPERROR_WEBHELP: u32 = 854u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA2_ERROR: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA3_GETATTRIBUTECOUNTBYTYPE: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA3_GETITEMINFOBYTYPE: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION2_BASE: u32 = 1400u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION2_CREATEQUERY: u32 = 1401u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION2_GETBYATTRANDMEDIATYPE: u32 = 1404u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION2_GETPLAYLISTBYQUERY: u32 = 1402u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION2_GETSTRINGCOLLBYQUERY: u32 = 1403u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_ADD: u32 = 452u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_BASE: u32 = 450u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_FREEZECOLLECTIONCHANGE: u32 = 474u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETALL: u32 = 453u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETATTRIBUTESTRINGCOLLECTION: u32 = 461u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETBYALBUM: u32 = 457u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETBYATTRIBUTE: u32 = 458u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETBYAUTHOR: u32 = 456u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETBYGENRE: u32 = 455u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETBYNAME: u32 = 454u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETBYQUERYDESCRIPTION: u32 = 473u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_GETMEDIAATOM: u32 = 470u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_ISDELETED: u32 = 472u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_NEWQUERY: u32 = 462u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_POSTCOLLECTIONCHANGE: u32 = 476u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_REMOVE: u32 = 459u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_SETDELETED: u32 = 471u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_STARTCONTENTSCAN: u32 = 465u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_STARTMONITORING: u32 = 463u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_STARTSEARCH: u32 = 467u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_STOPCONTENTSCAN: u32 = 466u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_STOPMONITORING: u32 = 464u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_STOPSEARCH: u32 = 468u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_UNFREEZECOLLECTIONCHANGE: u32 = 475u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIACOLLECTION_UPDATEMETADATA: u32 = 469u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_ATTRIBUTECOUNT: u32 = 759u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_BASE: u32 = 750u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_DURATION: u32 = 757u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_DURATIONSTRING: u32 = 758u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_GETATTRIBUTENAME: u32 = 760u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_GETITEMINFO: u32 = 761u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_GETITEMINFOBYATOM: u32 = 765u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_GETMARKERNAME: u32 = 756u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_GETMARKERTIME: u32 = 755u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_IMAGESOURCEHEIGHT: u32 = 753u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_IMAGESOURCEWIDTH: u32 = 752u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_ISIDENTICAL: u32 = 763u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_ISMEMBEROF: u32 = 766u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_ISREADONLYITEM: u32 = 767u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_MARKERCOUNT: u32 = 754u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_NAME: u32 = 764u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_SETITEMINFO: u32 = 762u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMEDIA_SOURCEURL: u32 = 751u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMETADATA_BASE: u32 = 1050u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMETADATA_PICTURE_DESCRIPTION: u32 = 1053u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMETADATA_PICTURE_MIMETYPE: u32 = 1051u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMETADATA_PICTURE_PICTURETYPE: u32 = 1052u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMETADATA_PICTURE_URL: u32 = 1054u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMETADATA_TEXT_DESCRIPTION: u32 = 1056u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPMETADATA_TEXT_TEXT: u32 = 1055u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_BANDWIDTH: u32 = 801u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_BASE: u32 = 800u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_BITRATE: u32 = 812u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_BUFFERINGCOUNT: u32 = 807u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_BUFFERINGPROGRESS: u32 = 808u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_BUFFERINGTIME: u32 = 809u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_DOWNLOADPROGRESS: u32 = 824u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_ENCODEDFRAMERATE: u32 = 825u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_FRAMERATE: u32 = 810u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_FRAMESSKIPPED: u32 = 826u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_GETPROXYBYPASSFORLOCAL: u32 = 821u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_GETPROXYEXCEPTIONLIST: u32 = 819u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_GETPROXYNAME: u32 = 815u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_GETPROXYPORT: u32 = 817u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_GETPROXYSETTINGS: u32 = 813u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_LOSTPACKETS: u32 = 805u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_MAXBANDWIDTH: u32 = 823u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_MAXBITRATE: u32 = 811u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_RECEIVEDPACKETS: u32 = 804u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_RECEPTIONQUALITY: u32 = 806u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_RECOVEREDPACKETS: u32 = 802u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_SETPROXYBYPASSFORLOCAL: u32 = 822u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_SETPROXYEXCEPTIONLIST: u32 = 820u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_SETPROXYNAME: u32 = 816u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_SETPROXYPORT: u32 = 818u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_SETPROXYSETTINGS: u32 = 814u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPNETWORK_SOURCEPROTOCOL: u32 = 803u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX2_BASE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX2_STRETCHTOFIT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX2_WINDOWLESSVIDEO: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX4_ISREMOTE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX4_OPENPLAYER: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX4_PLAYERAPPLICATION: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_CDROMBURNERROR: u32 = 6523u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_CDROMBURNMEDIAERROR: u32 = 6522u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_CDROMBURNSTATECHANGE: u32 = 6521u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_CDROMRIPMEDIAERROR: u32 = 6520u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_CDROMRIPSTATECHANGE: u32 = 6519u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_CLICK: u32 = 6505u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_CREATEPARTNERSHIPCOMPLETE: u32 = 6518u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_DEVICECONNECT: u32 = 6513u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_DEVICEDISCONNECT: u32 = 6514u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_DEVICEESTIMATION: u32 = 6527u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_DEVICESTATUSCHANGE: u32 = 6515u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_DEVICESYNCERROR: u32 = 6517u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_DEVICESYNCSTATECHANGE: u32 = 6516u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_DOUBLECLICK: u32 = 6506u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_FOLDERSCANSTATECHANGE: u32 = 6526u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_KEYDOWN: u32 = 6507u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_KEYPRESS: u32 = 6508u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_KEYUP: u32 = 6509u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_LIBRARYCONNECT: u32 = 6524u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_LIBRARYDISCONNECT: u32 = 6525u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_MOUSEDOWN: u32 = 6510u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_MOUSEMOVE: u32 = 6511u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_MOUSEUP: u32 = 6512u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_PLAYERDOCKEDSTATECHANGE: u32 = 6503u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_PLAYERRECONNECT: u32 = 6504u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_SWITCHEDTOCONTROL: u32 = 6502u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCXEVENT_SWITCHEDTOPLAYERAPPLICATION: u32 = 6501u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX_BASE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX_ENABLECONTEXTMENU: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX_ENABLED: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX_FULLSCREEN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX_LAST: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX_TRANSPARENTATSTART: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPOCX_UIMODE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYERAPP_BASE: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYERAPP_HASDISPLAY: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYERAPP_PLAYERDOCKED: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYERAPP_REMOTESTATUS: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYERAPP_SWITCHTOCONTROL: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYERAPP_SWITCHTOPLAYERAPPLICATION: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTARRAY_BASE: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTARRAY_COUNT: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTARRAY_ITEM: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_BASE: u32 = 550u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_GETALL: u32 = 553u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_GETBYNAME: u32 = 554u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_GETBYQUERYDESCRIPTION: u32 = 555u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_IMPORTPLAYLIST: u32 = 562u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_ISDELETED: u32 = 561u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_NEWPLAYLIST: u32 = 552u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_NEWQUERY: u32 = 557u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_REMOVE: u32 = 556u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_SETDELETED: u32 = 560u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_STARTMONITORING: u32 = 558u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLISTCOLLECTION_STOPMONITORING: u32 = 559u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_APPENDITEM: u32 = 207u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_ATTRIBUTECOUNT: u32 = 210u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_ATTRIBUTENAME: u32 = 211u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_BASE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_CLEAR: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_COUNT: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_GETITEMINFO: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_INSERTITEM: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_ISIDENTICAL: u32 = 213u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_ITEM: u32 = 212u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_MOVEITEM: u32 = 209u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_NAME: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_REMOVEITEM: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPPLAYLIST_SETITEMINFO: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPQUERY_ADDCONDITION: u32 = 1351u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPQUERY_BASE: u32 = 1350u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPQUERY_BEGINNEXTGROUP: u32 = 1352u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS2_DEFAULTAUDIOLANGUAGE: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS2_LIBRARYACCESSRIGHTS: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS2_REQUESTLIBRARYACCESSRIGHTS: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_AUTOSTART: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_BALANCE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_BASE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_BASEURL: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_DEFAULTFRAME: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_ENABLEERRORDIALOGS: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_GETMODE: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_INVOKEURLS: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_ISAVAILABLE: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_MUTE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_PLAYCOUNT: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_RATE: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_SETMODE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSETTINGS_VOLUME: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSTRINGCOLLECTION2_BASE: u32 = 1450u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSTRINGCOLLECTION2_GETATTRCOUNTBYTYPE: u32 = 1453u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSTRINGCOLLECTION2_GETITEMINFO: u32 = 1452u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSTRINGCOLLECTION2_GETITEMINFOBYTYPE: u32 = 1454u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSTRINGCOLLECTION2_ISIDENTICAL: u32 = 1451u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSTRINGCOLLECTION_BASE: u32 = 400u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSTRINGCOLLECTION_COUNT: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const DISPID_WMPSTRINGCOLLECTION_ITEM: u32 = 402u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const EFFECT2_FULLSCREENEXCLUSIVE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const EFFECT_CANGOFULLSCREEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const EFFECT_HASPROPERTYPAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const EFFECT_VARIABLEFREQSTEP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const EFFECT_WINDOWEDONLY: u32 = 8u32;
pub const FeedFolderWatcher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x281001ed_7765_4cb0_84af_e9b387af01ff);
pub const FeedWatcher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18a6737b_f433_4687_89bc_a1b4dfb9f123);
pub const FeedsManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaeb54c4_f66f_4806_83a0_805299f5e3ad);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const IOCTL_WMP_DEVICE_CAN_SYNC: u32 = 844123479u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const IOCTL_WMP_METADATA_ROUND_TRIP: u32 = 827346263u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_ALL_MEDIASENDTO: ::windows::core::PCWSTR = ::windows::w!("MediaSendTo");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_ALL_PLAYLISTSENDTO: ::windows::core::PCWSTR = ::windows::w!("PlaylistSendTo");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_FLAGS_ACCEPTSMEDIA: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_FLAGS_ACCEPTSPLAYLISTS: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_FLAGS_HASPRESETS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_FLAGS_HASPROPERTYPAGE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_FLAGS_HIDDEN: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_FLAGS_INSTALLAUTORUN: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_FLAGS_LAUNCHPROPERTYPAGE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_INSTALLREGKEY: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\MediaPlayer\\UIPlugins");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_INSTALLREGKEY_CAPABILITIES: ::windows::core::PCWSTR = ::windows::w!("Capabilities");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_INSTALLREGKEY_DESCRIPTION: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_INSTALLREGKEY_FRIENDLYNAME: ::windows::core::PCWSTR = ::windows::w!("FriendlyName");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_INSTALLREGKEY_UNINSTALL: ::windows::core::PCWSTR = ::windows::w!("UninstallPath");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_MISC_CURRENTPRESET: ::windows::core::PCWSTR = ::windows::w!("CurrentPreset");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_MISC_PRESETCOUNT: ::windows::core::PCWSTR = ::windows::w!("PresetCount");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_MISC_PRESETNAMES: ::windows::core::PCWSTR = ::windows::w!("PresetNames");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_MISC_QUERYDESTROY: ::windows::core::PCWSTR = ::windows::w!("QueryDestroy");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_SEPARATEWINDOW_DEFAULTHEIGHT: ::windows::core::PCWSTR = ::windows::w!("DefaultHeight");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_SEPARATEWINDOW_DEFAULTWIDTH: ::windows::core::PCWSTR = ::windows::w!("DefaultWidth");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_SEPARATEWINDOW_MAXHEIGHT: ::windows::core::PCWSTR = ::windows::w!("MaxHeight");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_SEPARATEWINDOW_MAXWIDTH: ::windows::core::PCWSTR = ::windows::w!("MaxWidth");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_SEPARATEWINDOW_MINHEIGHT: ::windows::core::PCWSTR = ::windows::w!("MinHeight");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_SEPARATEWINDOW_MINWIDTH: ::windows::core::PCWSTR = ::windows::w!("MinWidth");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_SEPARATEWINDOW_RESIZABLE: ::windows::core::PCWSTR = ::windows::w!("Resizable");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_TYPE_BACKGROUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_TYPE_DISPLAYAREA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_TYPE_METADATAAREA: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_TYPE_SEPARATEWINDOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const PLUGIN_TYPE_SETTINGSAREA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SA_BUFFER_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_ALLOWCDBURN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_ALLOWPDATRANSFER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_ALLOWPLAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_ALTLOGIN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_BACKGROUNDPROCESSING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_DEVICEAVAILABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_IS_CONTENTPARTNER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_PREPAREFORSYNC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_CAP_UILESSMODE_ALLOWPLAY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const SUBSCRIPTION_V1_CAPS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_BASE: u32 = 5000u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_CDROM_BASE: u32 = 5700u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_CONTENT_BASE: u32 = 5300u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_CONTROL_BASE: u32 = 5100u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_ERROR_BASE: u32 = 5500u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_NETWORK_BASE: u32 = 5400u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_PLAYLIST_BASE: u32 = 5800u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_SEEK_BASE: u32 = 5200u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPCOREEVENT_WARNING_BASE: u32 = 5600u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPGC_FLAGS_ALLOW_PREROLL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPGC_FLAGS_DISABLE_PLUGINS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPGC_FLAGS_IGNORE_AV_SYNC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPGC_FLAGS_SUPPRESS_DIALOGS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPGC_FLAGS_USE_CUSTOM_GRAPH: u32 = 16u32;
pub const WMPLib: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf52a50_394a_11d3_b153_00c04f79faa6);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPOCXEVENT_BASE: u32 = 6500u32;
pub const WMPRemoteMediaServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf333473_2cf7_4be2_907f_9aad5661364f);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPUE_EC_USER: u32 = 33024u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMP_MDRT_FLAGS_UNREPORTED_ADDED_ITEMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMP_MDRT_FLAGS_UNREPORTED_DELETED_ITEMS: u32 = 1u32;
pub const WMP_PLUGINTYPE_DSP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6434baea_4954_498d_abd5_2b07123e1f04);
pub const WMP_PLUGINTYPE_DSP_OUTOFPROC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef29b174_c347_44cc_9a4f_2399118ff38c);
pub const WMP_PLUGINTYPE_RENDERING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8554541_115d_406a_a4c7_51111c330183);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMP_SUBSCR_DL_TYPE_BACKGROUND: ::windows::core::PCWSTR = ::windows::w!("background");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMP_SUBSCR_DL_TYPE_REALTIME: ::windows::core::PCWSTR = ::windows::w!("real time");
pub const WMProfile_V40_100Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f99ddd8_6684_456b_a0a3_33e1316895f0);
pub const WMProfile_V40_128Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93ddbe12_13dc_4e32_a35e_40378e34279a);
pub const WMProfile_V40_16AMRadio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f4be81f_d57d_41e1_b2e3_2fad986bfec2);
pub const WMProfile_V40_1MBVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4482a4c_cc17_4b07_a94e_9818d5e0f13f);
pub const WMProfile_V40_250Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x541841c3_9339_4f7b_9a22_b11540894e42);
pub const WMProfile_V40_2856100MBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a1c2206_dc5e_4186_beb2_4c5a994b132e);
pub const WMProfile_V40_288FMRadioMono: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa57fc8_6ea4_4645_8abf_b6e5a8f814a1);
pub const WMProfile_V40_288FMRadioStereo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22fcf466_aa40_431f_a289_06d0ea1a1e40);
pub const WMProfile_V40_288VideoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac617f2d_6cbe_4e84_8e9a_ce151a12a354);
pub const WMProfile_V40_288VideoVoice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb2bc274_0eb6_4da9_b550_ecf7f2b9948f);
pub const WMProfile_V40_288VideoWebServer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabf2f00d_d555_4815_94ce_8275f3a70bfe);
pub const WMProfile_V40_3MBVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55374ac0_309b_4396_b88f_e6e292113f28);
pub const WMProfile_V40_512Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70440e6d_c4ef_4f84_8cd0_d5c28686e784);
pub const WMProfile_V40_56DialUpStereo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8026f87_e905_4594_a3c7_00d00041d1d9);
pub const WMProfile_V40_56DialUpVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe21713bb_652f_4dab_99de_71e04400270f);
pub const WMProfile_V40_56DialUpVideoWebServer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb756ff10_520f_4749_a399_b780e2fc9250);
pub const WMProfile_V40_64Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4820b3f7_cbec_41dc_9391_78598714c8e5);
pub const WMProfile_V40_6VoiceAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd508978a_11a0_4d15_b0da_acdc99d4f890);
pub const WMProfile_V40_96Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0efa0ee3_9e64_41e2_837f_3c0038f327ba);
pub const WMProfile_V40_DialUpMBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd7f47f1_72a6_45a4_80f0_3aecefc32c07);
pub const WMProfile_V40_IntranetMBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82cd3321_a94a_4ffc_9c2b_092c10ca16e7);
pub const WMProfile_V70_100Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9f3c932_5ea9_4c6d_89b4_2686e515426e);
pub const WMProfile_V70_128Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc64cf5da_df45_40d3_8027_de698d68dc66);
pub const WMProfile_V70_1500FilmContentVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6a5f6df_ee3f_434c_a433_523ce55f516b);
pub const WMProfile_V70_1500Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b89164a_5490_4686_9e37_5a80884e5146);
pub const WMProfile_V70_150VideoPDA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f472967_e3c6_4797_9694_f0304c5e2f17);
pub const WMProfile_V70_2000Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa980124_bf10_4e4f_9afd_4329a7395cff);
pub const WMProfile_V70_225VideoPDA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf55ea573_4c02_42b5_9026_a8260c438a9f);
pub const WMProfile_V70_256Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafe69b3a_403f_4a1b_8007_0e21cfb3df84);
pub const WMProfile_V70_2856100MBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07df7a25_3fe2_4a5b_8b1e_348b0721ca70);
pub const WMProfile_V70_288FMRadioMono: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc012a833_a03b_44a5_96dc_ed95cc65582d);
pub const WMProfile_V70_288FMRadioStereo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe96d67c9_1a39_4dc4_b900_b1184dc83620);
pub const WMProfile_V70_288VideoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58bba0ee_896a_4948_9953_85b736f83947);
pub const WMProfile_V70_288VideoVoice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb952f38e_7dbc_4533_a9ca_b00b1c6e9800);
pub const WMProfile_V70_288VideoWebServer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70a32e2b_e2df_4ebd_9105_d9ca194a2d50);
pub const WMProfile_V70_384Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3d45fbb_8782_44df_97c6_8678e2f9b13d);
pub const WMProfile_V70_56DialUpStereo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674ee767_0949_4fac_875e_f4c9c292013b);
pub const WMProfile_V70_56VideoWebServer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef99e40_57bc_4ab3_b2d1_b6e3caf64257);
pub const WMProfile_V70_64Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb29cffc6_f131_41db_b5e8_99d8b0b945f4);
pub const WMProfile_V70_64AudioISDN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91dea458_9d60_4212_9c59_d40919c939e4);
pub const WMProfile_V70_64VideoISDN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b7a7e9_7b8e_4992_a1a1_068217a3b311);
pub const WMProfile_V70_6VoiceAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeaba9fbf_b64f_49b3_aa0c_73fbdd150ad0);
pub const WMProfile_V70_700FilmContentVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a747920_2449_4d76_99cb_fdb0c90484d4);
pub const WMProfile_V70_768Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0326ebb6_f76e_4964_b0db_e729978d35ee);
pub const WMProfile_V70_96Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d4b819_16cc_4a59_9f37_693dbb0302d6);
pub const WMProfile_V70_DialUpMBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b16e74b_4068_45b5_b80e_7bf8c80d2c2f);
pub const WMProfile_V70_IntranetMBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x045880dc_34b6_4ca9_a326_73557ed143f3);
pub const WMProfile_V80_100768VideoMBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bdb5a0e_979e_47d3_9596_73b386392a55);
pub const WMProfile_V80_100Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2e300b4_c2d4_4fc0_b5dd_ecbd948dc0df);
pub const WMProfile_V80_128StereoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407b9450_8bdc_4ee5_88b8_6f527bd941f2);
pub const WMProfile_V80_1400NTSCVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x931d1bee_617a_4bcd_9905_ccd0786683ee);
pub const WMProfile_V80_150VideoPDA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaee16dfa_2c14_4a2f_ad3f_a3034031784f);
pub const WMProfile_V80_255VideoPDA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeedbcdf_3fac_4c93_ac0d_47941ec72c0b);
pub const WMProfile_V80_256Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbc75500_33d2_4466_b86b_122b201cc9ae);
pub const WMProfile_V80_288100VideoMBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8722c69_2419_4b36_b4e0_6e17b60564e5);
pub const WMProfile_V80_28856VideoMBR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd66920c4_c21f_4ec8_a0b4_95cf2bd57fc4);
pub const WMProfile_V80_288MonoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ea3126d_e1ba_4716_89af_f65cee0c0c67);
pub const WMProfile_V80_288StereoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e4cab5c_35dc_45bb_a7c0_19b28070d0cc);
pub const WMProfile_V80_288Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df678d9_1352_4186_bbf8_74f0c19b6ae2);
pub const WMProfile_V80_288VideoOnly: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c45b4c7_4aeb_4f78_a5ec_88420b9dadef);
pub const WMProfile_V80_32StereoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60907f9f_b352_47e5_b210_0ef1f47e9f9d);
pub const WMProfile_V80_384PALVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9227c692_ae62_4f72_a7ea_736062d0e21e);
pub const WMProfile_V80_384Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29b00c2b_09a9_48bd_ad09_cdae117d1da7);
pub const WMProfile_V80_48StereoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ee06be5_492b_480a_8a8f_12f373ecf9d4);
pub const WMProfile_V80_56Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x254e8a96_2612_405c_8039_f0bf725ced7d);
pub const WMProfile_V80_56VideoOnly: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e2a6955_81df_4943_ba50_68a986a708f6);
pub const WMProfile_V80_64StereoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09bb5bc4_3176_457f_8dd6_3cd919123e2d);
pub const WMProfile_V80_700NTSCVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8c2985f_e5d9_4538_9e23_9b21bf78f745);
pub const WMProfile_V80_700PALVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec298949_639b_45e2_96fd_4ab32d5919c2);
pub const WMProfile_V80_768Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d01102_e71a_4820_8f0d_13d2ec1e4872);
pub const WMProfile_V80_96StereoAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fc81930_61f2_436f_9d33_349f2a1c0f10);
pub const WMProfile_V80_BESTVBRVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x048439ba_309c_440e_9cb4_3dcca3756423);
pub const WMProfile_V80_FAIRVBRVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3510a862_5850_4886_835f_d78ec6a64042);
pub const WMProfile_V80_HIGHVBRVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f10d9d3_3b04_4fb0_a3d3_88d4ac854acc);
pub const WindowsMediaPlayer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf52a52_394a_11d3_b153_00c04f79faa6);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllAuthors: ::windows::core::PCWSTR = ::windows::w!("AllAuthors");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllCPAlbumIDs: ::windows::core::PCWSTR = ::windows::w!("AllCPAlbumIDs");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllCPAlbumSubGenreIDs: ::windows::core::PCWSTR = ::windows::w!("AllCPAlbumSubGenreIDs");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllCPArtistIDs: ::windows::core::PCWSTR = ::windows::w!("AllCPArtistIDs");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllCPGenreIDs: ::windows::core::PCWSTR = ::windows::w!("AllCPGenreIDs");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllCPListIDs: ::windows::core::PCWSTR = ::windows::w!("AllCPListIDs");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllCPRadioIDs: ::windows::core::PCWSTR = ::windows::w!("AllCPRadioIDs");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllCPTrackIDs: ::windows::core::PCWSTR = ::windows::w!("AllCPTrackIDs");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllReleaseDateYears: ::windows::core::PCWSTR = ::windows::w!("AllReleaseDateYears");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllUserEffectiveRatingStarss: ::windows::core::PCWSTR = ::windows::w!("AllUserEffectiveRatingStarss");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAllWMParentalRatings: ::windows::core::PCWSTR = ::windows::w!("AllWMParentalRatings");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szAuthor: ::windows::core::PCWSTR = ::windows::w!("Author");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szCPAlbumID: ::windows::core::PCWSTR = ::windows::w!("CPAlbumID");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szCPAlbumSubGenreID: ::windows::core::PCWSTR = ::windows::w!("CPAlbumSubGenreID");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szCPArtistID: ::windows::core::PCWSTR = ::windows::w!("CPArtistID");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szCPGenreID: ::windows::core::PCWSTR = ::windows::w!("CPGenreID");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szCPListID: ::windows::core::PCWSTR = ::windows::w!("CPListID");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szCPRadioID: ::windows::core::PCWSTR = ::windows::w!("CPRadioID");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szCPTrackID: ::windows::core::PCWSTR = ::windows::w!("CPTrackID");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_AccountBalance: ::windows::core::PCWSTR = ::windows::w!("AccountBalance");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_AccountType: ::windows::core::PCWSTR = ::windows::w!("AccountType");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_HasCachedCredentials: ::windows::core::PCWSTR = ::windows::w!("HasCachedCredentials");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_LicenseRefreshAdvanceWarning: ::windows::core::PCWSTR = ::windows::w!("LicenseRefreshAdvanceWarning");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_LoginState: ::windows::core::PCWSTR = ::windows::w!("LoginState");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_MaximumTrackPurchasePerPurchase: ::windows::core::PCWSTR = ::windows::w!("MaximumNumberOfTracksPerPurchase");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_MediaPlayerAccountType: ::windows::core::PCWSTR = ::windows::w!("MediaPlayerAccountType");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_PurchasedTrackRequiresReDownload: ::windows::core::PCWSTR = ::windows::w!("PurchasedTrackRequiresReDownload");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPartnerInfo_UserName: ::windows::core::PCWSTR = ::windows::w!("UserName");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPrice_CannotBuy: ::windows::core::PCWSTR = ::windows::w!("PriceCannotBuy");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPrice_Free: ::windows::core::PCWSTR = ::windows::w!("PriceFree");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szContentPrice_Unknown: ::windows::core::PCWSTR = ::windows::w!("PriceUnknown");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szFlyoutMenu: ::windows::core::PCWSTR = ::windows::w!("FlyoutMenu");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_ALTLoginCaption: ::windows::core::PCWSTR = ::windows::w!("ALTLoginCaption");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_ALTLoginURL: ::windows::core::PCWSTR = ::windows::w!("ALTLoginURL");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_AlbumArtURL: ::windows::core::PCWSTR = ::windows::w!("AlbumArt");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_ArtistArtURL: ::windows::core::PCWSTR = ::windows::w!("ArtistArt");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_AuthenticationSuccessURL: ::windows::core::PCWSTR = ::windows::w!("AuthenticationSuccessURL");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_CreateAccountURL: ::windows::core::PCWSTR = ::windows::w!("CreateAccount");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_ErrorDescription: ::windows::core::PCWSTR = ::windows::w!("CPErrorDescription");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_ErrorURL: ::windows::core::PCWSTR = ::windows::w!("CPErrorURL");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_ErrorURLLinkText: ::windows::core::PCWSTR = ::windows::w!("CPErrorURLLinkText");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_ForgetPasswordURL: ::windows::core::PCWSTR = ::windows::w!("ForgotPassword");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_GenreArtURL: ::windows::core::PCWSTR = ::windows::w!("GenreArt");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_HTMLViewURL: ::windows::core::PCWSTR = ::windows::w!("HTMLViewURL");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_ListArtURL: ::windows::core::PCWSTR = ::windows::w!("ListArt");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_LoginFailureURL: ::windows::core::PCWSTR = ::windows::w!("LoginFailureURL");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_PopupCaption: ::windows::core::PCWSTR = ::windows::w!("PopupCaption");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_PopupURL: ::windows::core::PCWSTR = ::windows::w!("Popup");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_RadioArtURL: ::windows::core::PCWSTR = ::windows::w!("RadioArt");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_SubGenreArtURL: ::windows::core::PCWSTR = ::windows::w!("SubGenreArt");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szItemInfo_TreeListIconURL: ::windows::core::PCWSTR = ::windows::w!("CPListIDIcon");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szMediaPlayerTask_Browse: ::windows::core::PCWSTR = ::windows::w!("Browse");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szMediaPlayerTask_Burn: ::windows::core::PCWSTR = ::windows::w!("Burn");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szMediaPlayerTask_Sync: ::windows::core::PCWSTR = ::windows::w!("Sync");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szOnlineStore: ::windows::core::PCWSTR = ::windows::w!("OnlineStore");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szRefreshLicenseBurn: ::windows::core::PCWSTR = ::windows::w!("RefreshForBurn");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szRefreshLicensePlay: ::windows::core::PCWSTR = ::windows::w!("RefreshForPlay");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szRefreshLicenseSync: ::windows::core::PCWSTR = ::windows::w!("RefreshForSync");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szReleaseDateYear: ::windows::core::PCWSTR = ::windows::w!("ReleaseDateYear");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szRootLocation: ::windows::core::PCWSTR = ::windows::w!("RootLocation");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szStationEvent_Complete: ::windows::core::PCWSTR = ::windows::w!("TrackComplete");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szStationEvent_Skipped: ::windows::core::PCWSTR = ::windows::w!("TrackSkipped");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szStationEvent_Started: ::windows::core::PCWSTR = ::windows::w!("TrackStarted");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szUnknownLocation: ::windows::core::PCWSTR = ::windows::w!("UnknownLocation");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szUserEffectiveRatingStars: ::windows::core::PCWSTR = ::windows::w!("UserEffectiveRatingStars");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szUserPlaylist: ::windows::core::PCWSTR = ::windows::w!("UserPlaylist");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szVerifyPermissionSync: ::windows::core::PCWSTR = ::windows::w!("VerifyPermissionSync");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szVideoRecent: ::windows::core::PCWSTR = ::windows::w!("VideoRecent");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szVideoRoot: ::windows::core::PCWSTR = ::windows::w!("VideoRoot");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szViewMode_Details: ::windows::core::PCWSTR = ::windows::w!("ViewModeDetails");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szViewMode_Icon: ::windows::core::PCWSTR = ::windows::w!("ViewModeIcon");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szViewMode_OrderedList: ::windows::core::PCWSTR = ::windows::w!("ViewModeOrderedList");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szViewMode_Report: ::windows::core::PCWSTR = ::windows::w!("ViewModeReport");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szViewMode_Tile: ::windows::core::PCWSTR = ::windows::w!("ViewModeTile");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const g_szWMParentalRating: ::windows::core::PCWSTR = ::windows::w!("WMParentalRating");
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const kfltTimedLevelMaximumFrequency: f32 = 22050f32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const kfltTimedLevelMinimumFrequency: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_BACKGROUNDSYNC_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FBSA_DISABLE: FEEDS_BACKGROUNDSYNC_ACTION = FEEDS_BACKGROUNDSYNC_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FBSA_ENABLE: FEEDS_BACKGROUNDSYNC_ACTION = FEEDS_BACKGROUNDSYNC_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FBSA_RUNNOW: FEEDS_BACKGROUNDSYNC_ACTION = FEEDS_BACKGROUNDSYNC_ACTION(2i32);
impl ::core::marker::Copy for FEEDS_BACKGROUNDSYNC_ACTION {}
impl ::core::clone::Clone for FEEDS_BACKGROUNDSYNC_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_BACKGROUNDSYNC_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_BACKGROUNDSYNC_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_BACKGROUNDSYNC_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_BACKGROUNDSYNC_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_BACKGROUNDSYNC_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FBSS_DISABLED: FEEDS_BACKGROUNDSYNC_STATUS = FEEDS_BACKGROUNDSYNC_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FBSS_ENABLED: FEEDS_BACKGROUNDSYNC_STATUS = FEEDS_BACKGROUNDSYNC_STATUS(1i32);
impl ::core::marker::Copy for FEEDS_BACKGROUNDSYNC_STATUS {}
impl ::core::clone::Clone for FEEDS_BACKGROUNDSYNC_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_BACKGROUNDSYNC_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_BACKGROUNDSYNC_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_BACKGROUNDSYNC_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_BACKGROUNDSYNC_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_DOWNLOAD_ERROR(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_NONE: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_DOWNLOAD_FAILED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_INVALID_FEED_FORMAT: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_NORMALIZATION_FAILED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_PERSISTENCE_FAILED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_DOWNLOAD_BLOCKED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(5i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_CANCELED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(6i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_UNSUPPORTED_AUTH: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(7i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_BACKGROUND_DOWNLOAD_DISABLED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(8i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_NOT_EXIST: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(9i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_UNSUPPORTED_MSXML: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(10i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_UNSUPPORTED_DTD: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(11i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_DOWNLOAD_SIZE_LIMIT_EXCEEDED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(12i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_ACCESS_DENIED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(13i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_AUTH_FAILED: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(14i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDE_INVALID_AUTH: FEEDS_DOWNLOAD_ERROR = FEEDS_DOWNLOAD_ERROR(15i32);
impl ::core::marker::Copy for FEEDS_DOWNLOAD_ERROR {}
impl ::core::clone::Clone for FEEDS_DOWNLOAD_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_DOWNLOAD_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_DOWNLOAD_ERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_DOWNLOAD_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_DOWNLOAD_ERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_DOWNLOAD_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDS_NONE: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDS_PENDING: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDS_DOWNLOADING: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDS_DOWNLOADED: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FDS_DOWNLOAD_FAILED: FEEDS_DOWNLOAD_STATUS = FEEDS_DOWNLOAD_STATUS(4i32);
impl ::core::marker::Copy for FEEDS_DOWNLOAD_STATUS {}
impl ::core::clone::Clone for FEEDS_DOWNLOAD_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_DOWNLOAD_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_DOWNLOAD_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_DOWNLOAD_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_DOWNLOAD_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_ERROR_CODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FEC_E_ERRORBASE: FEEDS_ERROR_CODE = FEEDS_ERROR_CODE(-1073479168i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FEC_E_INVALIDMSXMLPROPERTY: FEEDS_ERROR_CODE = FEEDS_ERROR_CODE(-1073479168i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FEC_E_DOWNLOADSIZELIMITEXCEEDED: FEEDS_ERROR_CODE = FEEDS_ERROR_CODE(-1073479167i32);
impl ::core::marker::Copy for FEEDS_ERROR_CODE {}
impl ::core::clone::Clone for FEEDS_ERROR_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_ERROR_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_ERROR_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_ERROR_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_ERROR_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_EVENTS_ITEM_COUNT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FEICF_READ_ITEM_COUNT_CHANGED: FEEDS_EVENTS_ITEM_COUNT_FLAGS = FEEDS_EVENTS_ITEM_COUNT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FEICF_UNREAD_ITEM_COUNT_CHANGED: FEEDS_EVENTS_ITEM_COUNT_FLAGS = FEEDS_EVENTS_ITEM_COUNT_FLAGS(2i32);
impl ::core::marker::Copy for FEEDS_EVENTS_ITEM_COUNT_FLAGS {}
impl ::core::clone::Clone for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_EVENTS_ITEM_COUNT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_ITEM_COUNT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_EVENTS_MASK(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FEM_FOLDEREVENTS: FEEDS_EVENTS_MASK = FEEDS_EVENTS_MASK(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FEM_FEEDEVENTS: FEEDS_EVENTS_MASK = FEEDS_EVENTS_MASK(2i32);
impl ::core::marker::Copy for FEEDS_EVENTS_MASK {}
impl ::core::clone::Clone for FEEDS_EVENTS_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_EVENTS_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_EVENTS_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_EVENTS_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_MASK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_EVENTS_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FES_ALL: FEEDS_EVENTS_SCOPE = FEEDS_EVENTS_SCOPE(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FES_SELF_ONLY: FEEDS_EVENTS_SCOPE = FEEDS_EVENTS_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FES_SELF_AND_CHILDREN_ONLY: FEEDS_EVENTS_SCOPE = FEEDS_EVENTS_SCOPE(2i32);
impl ::core::marker::Copy for FEEDS_EVENTS_SCOPE {}
impl ::core::clone::Clone for FEEDS_EVENTS_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_EVENTS_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_EVENTS_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_EVENTS_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_EVENTS_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_SYNC_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FSS_DEFAULT: FEEDS_SYNC_SETTING = FEEDS_SYNC_SETTING(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FSS_INTERVAL: FEEDS_SYNC_SETTING = FEEDS_SYNC_SETTING(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FSS_MANUAL: FEEDS_SYNC_SETTING = FEEDS_SYNC_SETTING(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FSS_SUGGESTED: FEEDS_SYNC_SETTING = FEEDS_SYNC_SETTING(3i32);
impl ::core::marker::Copy for FEEDS_SYNC_SETTING {}
impl ::core::clone::Clone for FEEDS_SYNC_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_SYNC_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_SYNC_SETTING {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_SYNC_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_SYNC_SETTING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_XML_FILTER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXFF_ALL: FEEDS_XML_FILTER_FLAGS = FEEDS_XML_FILTER_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXFF_UNREAD: FEEDS_XML_FILTER_FLAGS = FEEDS_XML_FILTER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXFF_READ: FEEDS_XML_FILTER_FLAGS = FEEDS_XML_FILTER_FLAGS(2i32);
impl ::core::marker::Copy for FEEDS_XML_FILTER_FLAGS {}
impl ::core::clone::Clone for FEEDS_XML_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_XML_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_XML_FILTER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_XML_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_FILTER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_XML_INCLUDE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXIF_NONE: FEEDS_XML_INCLUDE_FLAGS = FEEDS_XML_INCLUDE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXIF_CF_EXTENSIONS: FEEDS_XML_INCLUDE_FLAGS = FEEDS_XML_INCLUDE_FLAGS(1i32);
impl ::core::marker::Copy for FEEDS_XML_INCLUDE_FLAGS {}
impl ::core::clone::Clone for FEEDS_XML_INCLUDE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_XML_INCLUDE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_XML_INCLUDE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_XML_INCLUDE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_INCLUDE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_XML_SORT_ORDER(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXSO_NONE: FEEDS_XML_SORT_ORDER = FEEDS_XML_SORT_ORDER(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXSO_ASCENDING: FEEDS_XML_SORT_ORDER = FEEDS_XML_SORT_ORDER(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXSO_DESCENDING: FEEDS_XML_SORT_ORDER = FEEDS_XML_SORT_ORDER(2i32);
impl ::core::marker::Copy for FEEDS_XML_SORT_ORDER {}
impl ::core::clone::Clone for FEEDS_XML_SORT_ORDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_XML_SORT_ORDER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_XML_SORT_ORDER {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_XML_SORT_ORDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_SORT_ORDER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDS_XML_SORT_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXSP_NONE: FEEDS_XML_SORT_PROPERTY = FEEDS_XML_SORT_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXSP_PUBDATE: FEEDS_XML_SORT_PROPERTY = FEEDS_XML_SORT_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const FXSP_DOWNLOADTIME: FEEDS_XML_SORT_PROPERTY = FEEDS_XML_SORT_PROPERTY(2i32);
impl ::core::marker::Copy for FEEDS_XML_SORT_PROPERTY {}
impl ::core::clone::Clone for FEEDS_XML_SORT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDS_XML_SORT_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEEDS_XML_SORT_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDS_XML_SORT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDS_XML_SORT_PROPERTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayerState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const stop_state: PlayerState = PlayerState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const pause_state: PlayerState = PlayerState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const play_state: PlayerState = PlayerState(2i32);
impl ::core::marker::Copy for PlayerState {}
impl ::core::clone::Clone for PlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlayerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayerState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPAccountType(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpatBuyOnly: WMPAccountType = WMPAccountType(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpatSubscription: WMPAccountType = WMPAccountType(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpatJanus: WMPAccountType = WMPAccountType(3i32);
impl ::core::marker::Copy for WMPAccountType {}
impl ::core::clone::Clone for WMPAccountType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPAccountType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPAccountType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPAccountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPAccountType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPBurnFormat(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbfAudioCD: WMPBurnFormat = WMPBurnFormat(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbfDataCD: WMPBurnFormat = WMPBurnFormat(1i32);
impl ::core::marker::Copy for WMPBurnFormat {}
impl ::core::clone::Clone for WMPBurnFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPBurnFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPBurnFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPBurnFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPBurnFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPBurnState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsUnknown: WMPBurnState = WMPBurnState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsBusy: WMPBurnState = WMPBurnState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsReady: WMPBurnState = WMPBurnState(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsWaitingForDisc: WMPBurnState = WMPBurnState(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsRefreshStatusPending: WMPBurnState = WMPBurnState(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsPreparingToBurn: WMPBurnState = WMPBurnState(5i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsBurning: WMPBurnState = WMPBurnState(6i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsStopped: WMPBurnState = WMPBurnState(7i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsErasing: WMPBurnState = WMPBurnState(8i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpbsDownloading: WMPBurnState = WMPBurnState(9i32);
impl ::core::marker::Copy for WMPBurnState {}
impl ::core::clone::Clone for WMPBurnState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPBurnState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPBurnState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPBurnState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPBurnState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPCallbackNotification(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpcnLoginStateChange: WMPCallbackNotification = WMPCallbackNotification(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpcnAuthResult: WMPCallbackNotification = WMPCallbackNotification(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpcnLicenseUpdated: WMPCallbackNotification = WMPCallbackNotification(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpcnNewCatalogAvailable: WMPCallbackNotification = WMPCallbackNotification(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpcnNewPluginAvailable: WMPCallbackNotification = WMPCallbackNotification(5i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpcnDisableRadioSkipping: WMPCallbackNotification = WMPCallbackNotification(6i32);
impl ::core::marker::Copy for WMPCallbackNotification {}
impl ::core::clone::Clone for WMPCallbackNotification {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPCallbackNotification {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPCallbackNotification {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPCallbackNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPCallbackNotification").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPDeviceStatus(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpdsUnknown: WMPDeviceStatus = WMPDeviceStatus(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpdsPartnershipExists: WMPDeviceStatus = WMPDeviceStatus(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpdsPartnershipDeclined: WMPDeviceStatus = WMPDeviceStatus(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpdsPartnershipAnother: WMPDeviceStatus = WMPDeviceStatus(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpdsManualDevice: WMPDeviceStatus = WMPDeviceStatus(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpdsNewDevice: WMPDeviceStatus = WMPDeviceStatus(5i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpdsLast: WMPDeviceStatus = WMPDeviceStatus(6i32);
impl ::core::marker::Copy for WMPDeviceStatus {}
impl ::core::clone::Clone for WMPDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPDeviceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPDeviceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPDeviceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPDeviceStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPFolderScanState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpfssUnknown: WMPFolderScanState = WMPFolderScanState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpfssScanning: WMPFolderScanState = WMPFolderScanState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpfssUpdating: WMPFolderScanState = WMPFolderScanState(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpfssStopped: WMPFolderScanState = WMPFolderScanState(3i32);
impl ::core::marker::Copy for WMPFolderScanState {}
impl ::core::clone::Clone for WMPFolderScanState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPFolderScanState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPFolderScanState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPFolderScanState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPFolderScanState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPLibraryType(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpltUnknown: WMPLibraryType = WMPLibraryType(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpltAll: WMPLibraryType = WMPLibraryType(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpltLocal: WMPLibraryType = WMPLibraryType(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpltRemote: WMPLibraryType = WMPLibraryType(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpltDisc: WMPLibraryType = WMPLibraryType(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpltPortableDevice: WMPLibraryType = WMPLibraryType(5i32);
impl ::core::marker::Copy for WMPLibraryType {}
impl ::core::clone::Clone for WMPLibraryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPLibraryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPLibraryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPLibraryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPLibraryType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPOpenState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposUndefined: WMPOpenState = WMPOpenState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposPlaylistChanging: WMPOpenState = WMPOpenState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposPlaylistLocating: WMPOpenState = WMPOpenState(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposPlaylistConnecting: WMPOpenState = WMPOpenState(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposPlaylistLoading: WMPOpenState = WMPOpenState(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposPlaylistOpening: WMPOpenState = WMPOpenState(5i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposPlaylistOpenNoMedia: WMPOpenState = WMPOpenState(6i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposPlaylistChanged: WMPOpenState = WMPOpenState(7i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposMediaChanging: WMPOpenState = WMPOpenState(8i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposMediaLocating: WMPOpenState = WMPOpenState(9i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposMediaConnecting: WMPOpenState = WMPOpenState(10i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposMediaLoading: WMPOpenState = WMPOpenState(11i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposMediaOpening: WMPOpenState = WMPOpenState(12i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposMediaOpen: WMPOpenState = WMPOpenState(13i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposBeginCodecAcquisition: WMPOpenState = WMPOpenState(14i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposEndCodecAcquisition: WMPOpenState = WMPOpenState(15i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposBeginLicenseAcquisition: WMPOpenState = WMPOpenState(16i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposEndLicenseAcquisition: WMPOpenState = WMPOpenState(17i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposBeginIndividualization: WMPOpenState = WMPOpenState(18i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposEndIndividualization: WMPOpenState = WMPOpenState(19i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposMediaWaiting: WMPOpenState = WMPOpenState(20i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmposOpeningUnknownURL: WMPOpenState = WMPOpenState(21i32);
impl ::core::marker::Copy for WMPOpenState {}
impl ::core::clone::Clone for WMPOpenState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPOpenState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPOpenState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPOpenState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPOpenState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPPartnerNotification(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsnBackgroundProcessingBegin: WMPPartnerNotification = WMPPartnerNotification(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsnBackgroundProcessingEnd: WMPPartnerNotification = WMPPartnerNotification(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsnCatalogDownloadFailure: WMPPartnerNotification = WMPPartnerNotification(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsnCatalogDownloadComplete: WMPPartnerNotification = WMPPartnerNotification(4i32);
impl ::core::marker::Copy for WMPPartnerNotification {}
impl ::core::clone::Clone for WMPPartnerNotification {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPPartnerNotification {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPPartnerNotification {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPPartnerNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPartnerNotification").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPPlayState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsUndefined: WMPPlayState = WMPPlayState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsStopped: WMPPlayState = WMPPlayState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsPaused: WMPPlayState = WMPPlayState(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsPlaying: WMPPlayState = WMPPlayState(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsScanForward: WMPPlayState = WMPPlayState(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsScanReverse: WMPPlayState = WMPPlayState(5i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsBuffering: WMPPlayState = WMPPlayState(6i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsWaiting: WMPPlayState = WMPPlayState(7i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsMediaEnded: WMPPlayState = WMPPlayState(8i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsTransitioning: WMPPlayState = WMPPlayState(9i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsReady: WMPPlayState = WMPPlayState(10i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsReconnecting: WMPPlayState = WMPPlayState(11i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmppsLast: WMPPlayState = WMPPlayState(12i32);
impl ::core::marker::Copy for WMPPlayState {}
impl ::core::clone::Clone for WMPPlayState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPPlayState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPPlayState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPPlayState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlayState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPPlaylistChangeEventType(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcUnknown: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcClear: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcInfoChange: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcMove: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcDelete: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcInsert: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(5i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcAppend: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(6i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcPrivate: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(7i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcNameChange: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(8i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcMorph: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(9i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcSort: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(10i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmplcLast: WMPPlaylistChangeEventType = WMPPlaylistChangeEventType(11i32);
impl ::core::marker::Copy for WMPPlaylistChangeEventType {}
impl ::core::clone::Clone for WMPPlaylistChangeEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPPlaylistChangeEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPPlaylistChangeEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPPlaylistChangeEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlaylistChangeEventType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPPlugin_Caps(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPPlugin_Caps_CannotConvertFormats: WMPPlugin_Caps = WMPPlugin_Caps(1i32);
impl ::core::marker::Copy for WMPPlugin_Caps {}
impl ::core::clone::Clone for WMPPlugin_Caps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPPlugin_Caps {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPPlugin_Caps {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPPlugin_Caps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPPlugin_Caps").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPRipState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmprsUnknown: WMPRipState = WMPRipState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmprsRipping: WMPRipState = WMPRipState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmprsStopped: WMPRipState = WMPRipState(2i32);
impl ::core::marker::Copy for WMPRipState {}
impl ::core::clone::Clone for WMPRipState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPRipState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPRipState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPRipState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPRipState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPServices_StreamState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPServices_StreamState_Stop: WMPServices_StreamState = WMPServices_StreamState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPServices_StreamState_Pause: WMPServices_StreamState = WMPServices_StreamState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const WMPServices_StreamState_Play: WMPServices_StreamState = WMPServices_StreamState(2i32);
impl ::core::marker::Copy for WMPServices_StreamState {}
impl ::core::clone::Clone for WMPServices_StreamState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPServices_StreamState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPServices_StreamState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPServices_StreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPServices_StreamState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPStreamingType(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpstUnknown: WMPStreamingType = WMPStreamingType(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpstMusic: WMPStreamingType = WMPStreamingType(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpstVideo: WMPStreamingType = WMPStreamingType(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpstRadio: WMPStreamingType = WMPStreamingType(3i32);
impl ::core::marker::Copy for WMPStreamingType {}
impl ::core::clone::Clone for WMPStreamingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPStreamingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPStreamingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPStreamingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPStreamingType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPStringCollectionChangeEventType(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsccetUnknown: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsccetInsert: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsccetChange: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsccetDelete: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsccetClear: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(4i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsccetBeginUpdates: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(5i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsccetEndUpdates: WMPStringCollectionChangeEventType = WMPStringCollectionChangeEventType(6i32);
impl ::core::marker::Copy for WMPStringCollectionChangeEventType {}
impl ::core::clone::Clone for WMPStringCollectionChangeEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPStringCollectionChangeEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPStringCollectionChangeEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPStringCollectionChangeEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPStringCollectionChangeEventType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPSubscriptionDownloadState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsdlsDownloading: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsdlsPaused: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsdlsProcessing: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsdlsCompleted: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsdlsCancelled: WMPSubscriptionDownloadState = WMPSubscriptionDownloadState(4i32);
impl ::core::marker::Copy for WMPSubscriptionDownloadState {}
impl ::core::clone::Clone for WMPSubscriptionDownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPSubscriptionDownloadState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPSubscriptionDownloadState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPSubscriptionDownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSubscriptionDownloadState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPSubscriptionServiceEvent(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsseCurrentBegin: WMPSubscriptionServiceEvent = WMPSubscriptionServiceEvent(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsseCurrentEnd: WMPSubscriptionServiceEvent = WMPSubscriptionServiceEvent(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsseFullBegin: WMPSubscriptionServiceEvent = WMPSubscriptionServiceEvent(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpsseFullEnd: WMPSubscriptionServiceEvent = WMPSubscriptionServiceEvent(4i32);
impl ::core::marker::Copy for WMPSubscriptionServiceEvent {}
impl ::core::clone::Clone for WMPSubscriptionServiceEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPSubscriptionServiceEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPSubscriptionServiceEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPSubscriptionServiceEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSubscriptionServiceEvent").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPSyncState(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpssUnknown: WMPSyncState = WMPSyncState(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpssSynchronizing: WMPSyncState = WMPSyncState(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpssStopped: WMPSyncState = WMPSyncState(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpssEstimating: WMPSyncState = WMPSyncState(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpssLast: WMPSyncState = WMPSyncState(4i32);
impl ::core::marker::Copy for WMPSyncState {}
impl ::core::clone::Clone for WMPSyncState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPSyncState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPSyncState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPSyncState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPSyncState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPTaskType(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpttBrowse: WMPTaskType = WMPTaskType(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpttSync: WMPTaskType = WMPTaskType(2i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpttBurn: WMPTaskType = WMPTaskType(3i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpttCurrent: WMPTaskType = WMPTaskType(4i32);
impl ::core::marker::Copy for WMPTaskType {}
impl ::core::clone::Clone for WMPTaskType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPTaskType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPTaskType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPTaskType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTaskType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPTemplateSize(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmptsSmall: WMPTemplateSize = WMPTemplateSize(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmptsMedium: WMPTemplateSize = WMPTemplateSize(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmptsLarge: WMPTemplateSize = WMPTemplateSize(2i32);
impl ::core::marker::Copy for WMPTemplateSize {}
impl ::core::clone::Clone for WMPTemplateSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPTemplateSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPTemplateSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPTemplateSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTemplateSize").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMPTransactionType(pub i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpttNoTransaction: WMPTransactionType = WMPTransactionType(0i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpttDownload: WMPTransactionType = WMPTransactionType(1i32);
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub const wmpttBuy: WMPTransactionType = WMPTransactionType(2i32);
impl ::core::marker::Copy for WMPTransactionType {}
impl ::core::clone::Clone for WMPTransactionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMPTransactionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMPTransactionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMPTransactionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMPTransactionType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub struct TimedLevel {
    pub frequency: [u8; 2048],
    pub waveform: [u8; 2048],
    pub state: i32,
    pub timeStamp: i64,
}
impl ::core::marker::Copy for TimedLevel {}
impl ::core::clone::Clone for TimedLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedLevel").field("frequency", &self.frequency).field("waveform", &self.waveform).field("state", &self.state).field("timeStamp", &self.timeStamp).finish()
    }
}
unsafe impl ::windows::core::Abi for TimedLevel {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedLevel {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedLevel>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedLevel {}
impl ::core::default::Default for TimedLevel {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub struct WMPContextMenuInfo {
    pub dwID: u32,
    pub bstrMenuText: ::windows::core::BSTR,
    pub bstrHelpText: ::windows::core::BSTR,
}
impl ::core::clone::Clone for WMPContextMenuInfo {
    fn clone(&self) -> Self {
        Self { dwID: self.dwID, bstrMenuText: self.bstrMenuText.clone(), bstrHelpText: self.bstrHelpText.clone() }
    }
}
impl ::core::fmt::Debug for WMPContextMenuInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMPContextMenuInfo").field("dwID", &self.dwID).field("bstrMenuText", &self.bstrMenuText).field("bstrHelpText", &self.bstrHelpText).finish()
    }
}
unsafe impl ::windows::core::Abi for WMPContextMenuInfo {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMPContextMenuInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwID == other.dwID && self.bstrMenuText == other.bstrMenuText && self.bstrHelpText == other.bstrHelpText
    }
}
impl ::core::cmp::Eq for WMPContextMenuInfo {}
impl ::core::default::Default for WMPContextMenuInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub struct WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    pub dwCurrentTransactionID: u32,
    pub dwReturnedObjectCount: u32,
    pub dwUnretrievedObjectCount: u32,
    pub dwDeletedObjectStartingOffset: u32,
    pub dwFlags: u32,
    pub wsObjectPathnameList: [u16; 1],
}
impl ::core::marker::Copy for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {}
impl ::core::clone::Clone for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {}
impl ::core::default::Default for WMP_WMDM_METADATA_ROUND_TRIP_DEVICE2PC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`*"]
pub struct WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    pub dwChangesSinceTransactionID: u32,
    pub dwResultSetStartingIndex: u32,
}
impl ::core::marker::Copy for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {}
impl ::core::clone::Clone for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {}
impl ::core::default::Default for WMP_WMDM_METADATA_ROUND_TRIP_PC2DEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
