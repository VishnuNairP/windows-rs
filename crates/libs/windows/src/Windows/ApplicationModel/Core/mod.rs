#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppListEntry {
    type Vtable = IAppListEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppListEntry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef00f07f_2108_490a_877a_8a9f17c25fad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppListEntry2 {
    type Vtable = IAppListEntry2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppListEntry2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0a618ad_bf35_42ac_ac06_86eeeb41d04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppListEntry3 {
    type Vtable = IAppListEntry3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppListEntry3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6099f28d_fc32_470a_bc69_4b061a76ef2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub LaunchForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    LaunchForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppListEntry4 {
    type Vtable = IAppListEntry4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppListEntry4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a131ed2_56f5_487c_8697_5166f3b33da0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplication {
    type Vtable = ICoreApplication_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aacf7a4_5e1d_49df_8034_fb6a68bc5ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Suspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Suspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub Resuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Resuming: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResuming: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub GetCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RunWithActivationFactories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationfactorycallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunWithActivationFactories: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplication2 {
    type Vtable = ICoreApplication2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplication2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x998681fb_1ab6_4b7f_be4a_9a0645224c04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub LeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub EnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnteredBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnteredBackground: usize,
    pub EnablePrelaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplication3 {
    type Vtable = ICoreApplication3_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplication3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeec0d39_598b_4507_8a67_772632580a57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestRestartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRestartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    RequestRestartForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationExit(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplicationExit {
    type Vtable = ICoreApplicationExit_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationExit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf86461d_261e_4b72_9acd_44ed2ace6a29);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationExit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Exit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Exiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Exiting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExiting: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct ICoreApplicationUnhandledError(::windows::core::IUnknown);
impl ICoreApplicationUnhandledError {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnhandledErrorDetected(&self, handler: &super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnhandledErrorDetected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnhandledErrorDetected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUnhandledErrorDetected)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
::windows::core::interface_hierarchy!(ICoreApplicationUnhandledError, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ICoreApplicationUnhandledError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreApplicationUnhandledError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreApplicationUnhandledError {}
impl ::core::fmt::Debug for ICoreApplicationUnhandledError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreApplicationUnhandledError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreApplicationUnhandledError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f0e24ab0-dd09-42e1-b0bc-e0e131f78d7e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICoreApplicationUnhandledError {
    type Vtable = ICoreApplicationUnhandledError_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationUnhandledError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0e24ab0_dd09_42e1_b0bc_e0e131f78d7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUnhandledError_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub UnhandledErrorDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnhandledErrorDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnhandledErrorDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnhandledErrorDetected: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationUseCount(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplicationUseCount {
    type Vtable = ICoreApplicationUseCount_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationUseCount {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x518dc408_c077_475b_809e_0bc0c57e4b74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUseCount_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IncrementApplicationUseCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DecrementApplicationUseCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplicationView {
    type Vtable = ICoreApplicationView_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x638bb2db_451d_4661_b099_414f34ffb9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub CoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CoreWindow: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    pub IsMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHosted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplicationView2 {
    type Vtable = ICoreApplicationView2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationView2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68eb7adf_917f_48eb_9aeb_7de53e086ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplicationView3 {
    type Vtable = ICoreApplicationView3_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationView3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07ebe1b3_a4cf_4550_ab70_b07e85330bc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HostedViewClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HostedViewClosing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHostedViewClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHostedViewClosing: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplicationView5 {
    type Vtable = ICoreApplicationView5_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationView5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc095a8_8ef0_446d_9e60_3a3e0428c671);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView6(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplicationView6 {
    type Vtable = ICoreApplicationView6_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationView6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc119d49a_0679_49ba_803f_b79c5cf34cca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationViewTitleBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreApplicationViewTitleBar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x006d35e3_e1f1_431b_9508_29b96926ac53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationViewTitleBar_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetExtendViewIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ExtendViewIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SystemOverlayLeftInset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SystemOverlayRightInset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LayoutMetricsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutMetricsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLayoutMetricsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLayoutMetricsChanged: usize,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsVisibleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsVisibleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsVisibleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsVisibleChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreImmersiveApplication {
    type Vtable = ICoreImmersiveApplication_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ada0e3e_e4a2_4123_b451_dc96bf800419);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Views: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Views: usize,
    pub CreateNewView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runtimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MainView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreImmersiveApplication2 {
    type Vtable = ICoreImmersiveApplication2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x828e1e36_e9e3_4cfc_9b66_48b78ea9bb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateNewViewFromMainView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreImmersiveApplication3 {
    type Vtable = ICoreImmersiveApplication3_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a05b2f_ee0d_41e5_8314_cf10c91bf0af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateNewViewWithViewSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewsource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct IFrameworkView(::windows::core::IUnknown);
impl IFrameworkView {
    pub fn Initialize(&self, applicationview: &CoreApplicationView) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Initialize)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationview)).ok() }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn SetWindow(&self, window: &super::super::UI::Core::CoreWindow) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetWindow)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(window)).ok() }
    }
    pub fn Load(&self, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Load)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(entrypoint)).ok() }
    }
    pub fn Run(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Run)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Uninitialize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Uninitialize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IFrameworkView, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IFrameworkView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFrameworkView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFrameworkView {}
impl ::core::fmt::Debug for IFrameworkView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFrameworkView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFrameworkView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{faab5cd0-8924-45ac-ad0f-a08fae5d0324}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IFrameworkView {
    type Vtable = IFrameworkView_Vtbl;
}
unsafe impl ::windows::core::Interface for IFrameworkView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaab5cd0_8924_45ac_ad0f_a08fae5d0324);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkView_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationview: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub SetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetWindow: usize,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct IFrameworkViewSource(::windows::core::IUnknown);
impl IFrameworkViewSource {
    pub fn CreateView(&self) -> ::windows::core::Result<IFrameworkView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IFrameworkView>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IFrameworkViewSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IFrameworkViewSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFrameworkViewSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFrameworkViewSource {}
impl ::core::fmt::Debug for IFrameworkViewSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFrameworkViewSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFrameworkViewSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cd770614-65c4-426c-9494-34fc43554862}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IFrameworkViewSource {
    type Vtable = IFrameworkViewSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IFrameworkViewSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd770614_65c4_426c_9494_34fc43554862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkViewSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostedViewClosingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IHostedViewClosingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd238943c_b24e_4790_acb5_3e4243c4ff87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostedViewClosingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnhandledError(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUnhandledError {
    type Vtable = IUnhandledError_Vtbl;
}
unsafe impl ::windows::core::Interface for IUnhandledError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9459b726_53b5_4686_9eaf_fa8162dc3980);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledError_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Propagate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnhandledErrorDetectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUnhandledErrorDetectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x679ab78b_b336_4822_ac40_0d750f0b7a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledErrorDetectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnhandledError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct AppListEntry(::windows::core::IUnknown);
impl AppListEntry {
    pub fn DisplayInfo(&self) -> ::windows::core::Result<super::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AppDisplayInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppUserModelId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn LaunchForUserAsync(&self, user: &super::super::System::User) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppListEntry3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn AppInfo(&self) -> ::windows::core::Result<super::AppInfo> {
        let this = &::windows::core::Interface::cast::<IAppListEntry4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AppInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppListEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppListEntry {}
impl ::core::fmt::Debug for AppListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppListEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppListEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.AppListEntry;{ef00f07f-2108-490a-877a-8a9f17c25fad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppListEntry {
    type Vtable = IAppListEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for AppListEntry {
    const IID: ::windows::core::GUID = <IAppListEntry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppListEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Core.AppListEntry";
}
::windows::core::interface_hierarchy!(AppListEntry, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppListEntry {}
unsafe impl ::core::marker::Sync for AppListEntry {}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
pub struct CoreApplication;
impl CoreApplication {
    pub fn Id() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Suspending(handler: &super::super::Foundation::EventHandler<super::SuspendingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Suspending)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSuspending(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveSuspending)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Resuming(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Resuming)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResuming(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveResuming)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties() -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        })
    }
    pub fn GetCurrentView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn Run<'a, P0, E0>(viewsource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IFrameworkViewSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Vtable::vtable(this).Run)(::windows::core::Vtable::as_raw(this), viewsource.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunWithActivationFactories<'a, P0, E0>(activationfactorycallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IGetActivationFactory>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Vtable::vtable(this).RunWithActivationFactories)(::windows::core::Vtable::as_raw(this), activationfactorycallback.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated(handler: &super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundActivated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveBackgroundActivated)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LeavingBackground(handler: &super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LeavingBackground)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLeavingBackground(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveLeavingBackground)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnteredBackground(handler: &super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnteredBackground)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnteredBackground(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveEnteredBackground)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    pub fn EnablePrelaunch(value: bool) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Vtable::vtable(this).EnablePrelaunch)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestRestartAsync(launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestRestartAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(launcharguments), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn RequestRestartForUserAsync(user: &super::super::System::User, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestRestartForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(launcharguments), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    pub fn Exit() -> ::windows::core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows::core::Vtable::vtable(this).Exit)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Exiting(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplicationExit(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Exiting)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveExiting(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveExiting)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnhandledErrorDetected(handler: &super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplicationUnhandledError(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnhandledErrorDetected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnhandledErrorDetected(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::ICoreApplicationUnhandledError(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveUnhandledErrorDetected)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    pub fn IncrementApplicationUseCount() -> ::windows::core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows::core::Vtable::vtable(this).IncrementApplicationUseCount)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    pub fn DecrementApplicationUseCount() -> ::windows::core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows::core::Vtable::vtable(this).DecrementApplicationUseCount)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Views() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CoreApplicationView>> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Views)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<CoreApplicationView>>(result__)
        })
    }
    pub fn CreateNewView(runtimetype: &::windows::core::HSTRING, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNewView)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(runtimetype), ::core::mem::transmute_copy(entrypoint), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn MainView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MainView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn CreateNewViewFromMainView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNewViewFromMainView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn CreateNewViewWithViewSource<'a, P0, E0>(viewsource: P0) -> ::windows::core::Result<CoreApplicationView>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IFrameworkViewSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICoreImmersiveApplication3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNewViewWithViewSource)(::windows::core::Vtable::as_raw(this), viewsource.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreApplication<R, F: FnOnce(&ICoreApplication) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplication2<R, F: FnOnce(&ICoreApplication2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplication3<R, F: FnOnce(&ICoreApplication3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplicationExit<R, F: FnOnce(&ICoreApplicationExit) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationExit> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplicationUnhandledError<R, F: FnOnce(&ICoreApplicationUnhandledError) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationUnhandledError> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplicationUseCount<R, F: FnOnce(&ICoreApplicationUseCount) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationUseCount> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication<R, F: FnOnce(&ICoreImmersiveApplication) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication2<R, F: FnOnce(&ICoreImmersiveApplication2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication3<R, F: FnOnce(&ICoreImmersiveApplication3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CoreApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplication";
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct CoreApplicationView(::windows::core::IUnknown);
impl CoreApplicationView {
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn CoreWindow(&self) -> ::windows::core::Result<super::super::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CoreWindow)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Core::CoreWindow>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated(&self, handler: &super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Activated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveActivated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn IsMain(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMain)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHosted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHosted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Core::CoreDispatcher>(result__)
        }
    }
    pub fn IsComponent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsComponent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TitleBar(&self) -> ::windows::core::Result<CoreApplicationViewTitleBar> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TitleBar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreApplicationViewTitleBar>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HostedViewClosing(&self, handler: &super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HostedViewClosing)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHostedViewClosing(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveHostedViewClosing)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreApplicationView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreApplicationView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreApplicationView {}
impl ::core::fmt::Debug for CoreApplicationView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreApplicationView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreApplicationView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationView;{638bb2db-451d-4661-b099-414f34ffb9f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreApplicationView {
    type Vtable = ICoreApplicationView_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreApplicationView {
    const IID: ::windows::core::GUID = <ICoreApplicationView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreApplicationView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationView";
}
::windows::core::interface_hierarchy!(CoreApplicationView, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct CoreApplicationViewTitleBar(::windows::core::IUnknown);
impl CoreApplicationViewTitleBar {
    pub fn SetExtendViewIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExtendViewIntoTitleBar)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ExtendViewIntoTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendViewIntoTitleBar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SystemOverlayLeftInset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemOverlayLeftInset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SystemOverlayRightInset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemOverlayRightInset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LayoutMetricsChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LayoutMetricsChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLayoutMetricsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveLayoutMetricsChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsVisibleChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVisibleChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsVisibleChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveIsVisibleChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for CoreApplicationViewTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreApplicationViewTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreApplicationViewTitleBar {}
impl ::core::fmt::Debug for CoreApplicationViewTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreApplicationViewTitleBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreApplicationViewTitleBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationViewTitleBar;{006d35e3-e1f1-431b-9508-29b96926ac53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreApplicationViewTitleBar {
    const IID: ::windows::core::GUID = <ICoreApplicationViewTitleBar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreApplicationViewTitleBar {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationViewTitleBar";
}
::windows::core::interface_hierarchy!(CoreApplicationViewTitleBar, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct HostedViewClosingEventArgs(::windows::core::IUnknown);
impl HostedViewClosingEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for HostedViewClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HostedViewClosingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HostedViewClosingEventArgs {}
impl ::core::fmt::Debug for HostedViewClosingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostedViewClosingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HostedViewClosingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.HostedViewClosingEventArgs;{d238943c-b24e-4790-acb5-3e4243c4ff87})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for HostedViewClosingEventArgs {
    const IID: ::windows::core::GUID = <IHostedViewClosingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HostedViewClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.HostedViewClosingEventArgs";
}
::windows::core::interface_hierarchy!(HostedViewClosingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HostedViewClosingEventArgs {}
unsafe impl ::core::marker::Sync for HostedViewClosingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct UnhandledError(::windows::core::IUnknown);
impl UnhandledError {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Propagate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Propagate)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for UnhandledError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnhandledError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnhandledError {}
impl ::core::fmt::Debug for UnhandledError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnhandledError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnhandledError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledError;{9459b726-53b5-4686-9eaf-fa8162dc3980})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UnhandledError {
    type Vtable = IUnhandledError_Vtbl;
}
unsafe impl ::windows::core::Interface for UnhandledError {
    const IID: ::windows::core::GUID = <IUnhandledError as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledError";
}
::windows::core::interface_hierarchy!(UnhandledError, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UnhandledError {}
unsafe impl ::core::marker::Sync for UnhandledError {}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct UnhandledErrorDetectedEventArgs(::windows::core::IUnknown);
impl UnhandledErrorDetectedEventArgs {
    pub fn UnhandledError(&self) -> ::windows::core::Result<UnhandledError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnhandledError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<UnhandledError>(result__)
        }
    }
}
impl ::core::clone::Clone for UnhandledErrorDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnhandledErrorDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnhandledErrorDetectedEventArgs {}
impl ::core::fmt::Debug for UnhandledErrorDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnhandledErrorDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnhandledErrorDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs;{679ab78b-b336-4822-ac40-0d750f0b7a2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UnhandledErrorDetectedEventArgs {
    const IID: ::windows::core::GUID = <IUnhandledErrorDetectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UnhandledErrorDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs";
}
::windows::core::interface_hierarchy!(UnhandledErrorDetectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UnhandledErrorDetectedEventArgs {}
unsafe impl ::core::marker::Sync for UnhandledErrorDetectedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppRestartFailureReason(pub i32);
impl AppRestartFailureReason {
    pub const RestartPending: Self = Self(0i32);
    pub const NotInForeground: Self = Self(1i32);
    pub const InvalidUser: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for AppRestartFailureReason {}
impl ::core::clone::Clone for AppRestartFailureReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppRestartFailureReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppRestartFailureReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppRestartFailureReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRestartFailureReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRestartFailureReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Core.AppRestartFailureReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
