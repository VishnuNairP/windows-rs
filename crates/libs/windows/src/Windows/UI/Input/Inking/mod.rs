#[cfg(feature = "UI_Input_Inking_Analysis")]
pub mod Analysis;
#[cfg(feature = "UI_Input_Inking_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking_Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkDrawingAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97a2176c_6774_48ad_84f0_48f5a9be74f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    pub PenTip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PenTipShape) -> ::windows::core::HRESULT,
    pub SetPenTip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PenTipShape) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSize: usize,
    pub IgnorePressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIgnorePressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub FitToCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetFitToCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkDrawingAttributes2 {
    type Vtable = IInkDrawingAttributes2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkDrawingAttributes2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cab6508_8ec4_42fd_a5a5_e4b7d1d5316d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub PenTipTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PenTipTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPenTipTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPenTipTransform: usize,
    pub DrawAsHighlighter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDrawAsHighlighter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkDrawingAttributes3 {
    type Vtable = IInkDrawingAttributes3_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkDrawingAttributes3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72020002_7d5b_4690_8af4_e664cbe2b74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkDrawingAttributesKind) -> ::windows::core::HRESULT,
    pub PencilProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkDrawingAttributes4 {
    type Vtable = IInkDrawingAttributes4_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkDrawingAttributes4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef65dc25_9f19_456d_91a3_bc3a3d91c5fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IgnoreTilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIgnoreTilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkDrawingAttributes5 {
    type Vtable = IInkDrawingAttributes5_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkDrawingAttributes5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd11aa0bb_0775_4852_ae64_41143a7ae6c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ModelerAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributesPencilProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkDrawingAttributesPencilProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f2534cb_2d86_41bb_b0e8_e4c2a0253c52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesPencilProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Opacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkDrawingAttributesStatics {
    type Vtable = IInkDrawingAttributesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkDrawingAttributesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf731e03f_1a65_4862_96cb_6e1665e17f6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateForPencil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkInputConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkInputConfiguration {
    type Vtable = IInkInputConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkInputConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a68dc4_0b7b_49d7_b34f_9901e524dcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPrimaryBarrelButtonInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPrimaryBarrelButtonInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsEraserInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEraserInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkInputConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkInputConfiguration2 {
    type Vtable = IInkInputConfiguration2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkInputConfiguration2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac2272e_81b4_5cc4_a36d_d057c387dfda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputConfiguration2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPenHapticFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPenHapticFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkInputProcessingConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkInputProcessingConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2778d85e_33ca_4b06_a6d3_ac3945116d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputProcessingConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkInputProcessingMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkInputProcessingMode) -> ::windows::core::HRESULT,
    pub RightDragAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkInputRightDragAction) -> ::windows::core::HRESULT,
    pub SetRightDragAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkInputRightDragAction) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkManager {
    type Vtable = IInkManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4744737d_671b_4163_9c95_4e8d7a035fe1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkManipulationMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkManipulationMode) -> ::windows::core::HRESULT,
    pub ProcessPointerDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessPointerUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProcessPointerUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProcessPointerUp: usize,
    pub SetDefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecognizeAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognitiontarget: InkRecognitionTarget, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecognizeAsync2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkModelerAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkModelerAttributes {
    type Vtable = IInkModelerAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkModelerAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad31f27_0cd9_4bfd_b6f3_9e03ba8d7454);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkModelerAttributes_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PredictionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PredictionTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetPredictionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPredictionTime: usize,
    pub ScalingFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetScalingFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkModelerAttributes2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkModelerAttributes2 {
    type Vtable = IInkModelerAttributes2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkModelerAttributes2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86d1d09a_4ef8_5e25_b7bc_b65424f16bb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkModelerAttributes2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UseVelocityBasedPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUseVelocityBasedPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPoint(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPoint {
    type Vtable = IInkPoint_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPoint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f87272b_858c_46a5_9b41_d195970459fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPoint_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPoint2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPoint2 {
    type Vtable = IInkPoint2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPoint2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfba9c3f7_ae56_4d5c_bd2f_0ac45f5e4af9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPoint2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TiltX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub TiltY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct IInkPointFactory(::windows::core::IUnknown);
impl IInkPointFactory {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPoint(&self, position: super::super::super::Foundation::Point, pressure: f32) -> ::windows::core::Result<InkPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInkPoint)(::windows::core::Vtable::as_raw(this), position, pressure, result__.as_mut_ptr()).from_abi::<InkPoint>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IInkPointFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IInkPointFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkPointFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkPointFactory {}
impl ::core::fmt::Debug for IInkPointFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPointFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPointFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{29e5d51c-c98f-405d-9f3b-e53e31068d4d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IInkPointFactory {
    type Vtable = IInkPointFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPointFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29e5d51c_c98f_405d_9f3b_e53e31068d4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInkPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInkPoint: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPointFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPointFactory2 {
    type Vtable = IInkPointFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPointFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0145e85_daff_45f2_ad69_050d8256a209);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInkPointWithTiltAndTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInkPointWithTiltAndTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPresenter {
    type Vtable = IInkPresenter_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa69b70e2_887b_458f_b173_4fe4438930a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub InputDeviceTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    InputDeviceTypes: usize,
    #[cfg(feature = "UI_Core")]
    pub SetInputDeviceTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetInputDeviceTypes: usize,
    pub UnprocessedInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StrokeInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InputProcessingConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StrokeContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStrokeContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyDefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateDefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ActivateCustomDrying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPredefinedConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkPresenterPredefinedConfiguration) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StrokesCollected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StrokesCollected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokesCollected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokesCollected: usize,
    #[cfg(feature = "Foundation")]
    pub StrokesErased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StrokesErased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokesErased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokesErased: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPresenter2 {
    type Vtable = IInkPresenter2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf53e612_9a34_11e6_9f33_a24fc0d9649c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HighContrastAdjustment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkHighContrastAdjustment) -> ::windows::core::HRESULT,
    pub SetHighContrastAdjustment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkHighContrastAdjustment) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenter3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPresenter3 {
    type Vtable = IInkPresenter3_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenter3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e1ce89_d37d_4a90_83fc_7f5e9dfbf217);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InputConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenterProtractor(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPresenterProtractor {
    type Vtable = IInkPresenterProtractor_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenterProtractor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de3f2aa_ef6c_4e91_a73b_5b70d56fbd17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterProtractor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AreTickMarksVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAreTickMarksVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AreRaysVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAreRaysVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCenterMarkerVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCenterMarkerVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAngleReadoutVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAngleReadoutVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsResizable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsResizable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Radius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub AccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub SetAccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenterProtractorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPresenterProtractorFactory {
    type Vtable = IInkPresenterProtractorFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenterProtractorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320103c9_68fa_47e9_8127_8370711fc46c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterProtractorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpresenter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenterRuler(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPresenterRuler {
    type Vtable = IInkPresenterRuler_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenterRuler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cda7d5a_dec7_4dd7_877a_2133f183d48a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRuler_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenterRuler2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkPresenterRuler2 {
    type Vtable = IInkPresenterRuler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenterRuler2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45130dc1_bc61_44d4_a423_54712ae671c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRuler2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AreTickMarksVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAreTickMarksVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCompassVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCompassVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct IInkPresenterRulerFactory(::windows::core::IUnknown);
impl IInkPresenterRulerFactory {
    pub fn Create(&self, inkpresenter: &InkPresenter) -> ::windows::core::Result<InkPresenterRuler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(inkpresenter), result__.as_mut_ptr()).from_abi::<InkPresenterRuler>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IInkPresenterRulerFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IInkPresenterRulerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkPresenterRulerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkPresenterRulerFactory {}
impl ::core::fmt::Debug for IInkPresenterRulerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPresenterRulerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPresenterRulerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{34361beb-9001-4a4b-a690-69dbaf63e501}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IInkPresenterRulerFactory {
    type Vtable = IInkPresenterRulerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenterRulerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34361beb_9001_4a4b_a690_69dbaf63e501);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRulerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpresenter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct IInkPresenterStencil(::windows::core::IUnknown);
impl IInkPresenterStencil {
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForegroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetForegroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Transform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransform)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
::windows::core::interface_hierarchy!(IInkPresenterStencil, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IInkPresenterStencil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkPresenterStencil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkPresenterStencil {}
impl ::core::fmt::Debug for IInkPresenterStencil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPresenterStencil").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPresenterStencil {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{30d12d6d-3e06-4d02-b116-277fb5d8addc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IInkPresenterStencil {
    type Vtable = IInkPresenterStencil_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenterStencil {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d12d6d_3e06_4d02_b116_277fb5d8addc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterStencil_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkPresenterStencilKind) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Transform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkRecognitionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkRecognitionResult {
    type Vtable = IInkRecognitionResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkRecognitionResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36461a94_5068_40ef_8a05_2c2fb60908a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextCandidates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextCandidates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkRecognizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkRecognizer {
    type Vtable = IInkRecognizer_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkRecognizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077ccea3_904d_442a_b151_aaca3631c43b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct IInkRecognizerContainer(::windows::core::IUnknown);
impl IInkRecognizerContainer {
    pub fn SetDefaultRecognizer(&self, recognizer: &InkRecognizer) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDefaultRecognizer)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(recognizer)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecognizeAsync(&self, strokecollection: &InkStrokeContainer, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecognizeAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(strokecollection), recognitiontarget, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRecognizers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IInkRecognizerContainer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IInkRecognizerContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkRecognizerContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkRecognizerContainer {}
impl ::core::fmt::Debug for IInkRecognizerContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizerContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkRecognizerContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a74d9a31-8047-4698-a912-f82a5085012f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IInkRecognizerContainer {
    type Vtable = IInkRecognizerContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkRecognizerContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74d9a31_8047_4698_a912_f82a5085012f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerContainer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetDefaultRecognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecognizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokecollection: *mut ::core::ffi::c_void, recognitiontarget: InkRecognitionTarget, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecognizeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecognizers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecognizers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStroke(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStroke {
    type Vtable = IInkStroke_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStroke {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15144d60_cce3_4fcf_9d52_11518ab6afd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    pub Selected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Recognized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRenderingSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRenderingSegments: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStroke2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStroke2 {
    type Vtable = IInkStroke2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStroke2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5db9e4f4_bafa_4de1_89d3_201b1ed7d89b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub PointTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PointTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPointTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPointTransform: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInkPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInkPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStroke3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStroke3 {
    type Vtable = IInkStroke3_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStroke3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a807374_9499_411d_a1c4_68855d03d65f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StrokeStartedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StrokeStartedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStrokeStartedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStrokeStartedTime: usize,
    #[cfg(feature = "Foundation")]
    pub StrokeDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StrokeDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetStrokeDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStrokeDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStroke4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStroke4 {
    type Vtable = IInkStroke4_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStroke4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd5b62e5_b6e9_5b91_a577_1921d2348690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeBuilder(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokeBuilder {
    type Vtable = IInkStrokeBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokeBuilder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82bbd1dc_1c63_41dc_9e07_4b4a70ced801);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BeginStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppendToStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateStroke: usize,
    pub SetDefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeBuilder2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokeBuilder2 {
    type Vtable = IInkStrokeBuilder2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokeBuilder2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd82bc27_731f_4cbc_bbbf_6d468044f1e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub CreateStrokeFromInkPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *mut ::core::ffi::c_void, transform: super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))]
    CreateStrokeFromInkPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeBuilder3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokeBuilder3 {
    type Vtable = IInkStrokeBuilder3_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokeBuilder3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2c71fcd_5472_46b1_a81d_c37a3d169441);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub CreateStrokeFromInkPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *mut ::core::ffi::c_void, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: *mut ::core::ffi::c_void, strokeduration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))]
    CreateStrokeFromInkPoints: usize,
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct IInkStrokeContainer(::windows::core::IUnknown);
impl IInkStrokeContainer {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BoundingRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn AddStroke(&self, stroke: &InkStroke) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddStroke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(stroke)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteSelected)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected(&self, translation: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveSelected)(::windows::core::Vtable::as_raw(this), translation, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SelectWithPolyLine<'a, P0, E0>(&self, polyline: P0) -> ::windows::core::Result<super::super::super::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectWithPolyLine)(::windows::core::Vtable::as_raw(this), polyline.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine(&self, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectWithLine)(::windows::core::Vtable::as_raw(this), from, to, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CopySelectedToClipboard)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard(&self, position: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PasteFromClipboard)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanPasteFromClipboard)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, P0, E0>(&self, inputstream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadAsync)(::windows::core::Vtable::as_raw(this), inputstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, P0, E0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), outputstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, P0, E0>(&self, recognitionresults: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateRecognitionResults)(::windows::core::Vtable::as_raw(this), recognitionresults.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStrokes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRecognitionResults)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IInkStrokeContainer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IInkStrokeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkStrokeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkStrokeContainer {}
impl ::core::fmt::Debug for IInkStrokeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkStrokeContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkStrokeContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{22accbc6-faa9-4f14-b68c-f6cee670ae16}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IInkStrokeContainer {
    type Vtable = IInkStrokeContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokeContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22accbc6_faa9_4f14_b68c_f6cee670ae16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    pub AddStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stroke: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteSelected: usize,
    #[cfg(feature = "Foundation")]
    pub MoveSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveSelected: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SelectWithPolyLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, polyline: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SelectWithPolyLine: usize,
    #[cfg(feature = "Foundation")]
    pub SelectWithLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectWithLine: usize,
    pub CopySelectedToClipboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PasteFromClipboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PasteFromClipboard: usize,
    pub CanPasteFromClipboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateRecognitionResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognitionresults: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateRecognitionResults: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecognitionResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecognitionResults: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeContainer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokeContainer2 {
    type Vtable = IInkStrokeContainer2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokeContainer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8901d364_da36_4bcf_9e5c_d195825995b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddStrokes: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeContainer3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokeContainer3 {
    type Vtable = IInkStrokeContainer3_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokeContainer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d07bea5_baea_4c82_a719_7b83da1067d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, inkpersistenceformat: InkPersistenceFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveWithFormatAsync: usize,
    pub GetStrokeById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeInput(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokeInput {
    type Vtable = IInkStrokeInput_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokeInput {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2ffe7b_5e10_43c6_a080_88f26e1dc67d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeInput_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub StrokeStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    StrokeStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokeStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokeStarted: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub StrokeContinued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    StrokeContinued: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokeContinued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokeContinued: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub StrokeEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    StrokeEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokeEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokeEnded: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub StrokeCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    StrokeCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokeCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokeCanceled: usize,
    pub InkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeRenderingSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegment_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokeRenderingSegment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68510f1f_88e3_477a_a2fa_569f5f1f9bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeRenderingSegment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub BezierControlPoint1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BezierControlPoint1: usize,
    #[cfg(feature = "Foundation")]
    pub BezierControlPoint2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BezierControlPoint2: usize,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub TiltX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub TiltY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Twist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokesCollectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokesCollectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4f3f229_1938_495c_b4d9_6de4b08d4811);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokesCollectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strokes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokesErasedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkStrokesErasedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4216a22_1503_4ebf_8ff5_2de84584a8aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokesErasedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strokes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkSynchronizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkSynchronizer {
    type Vtable = IInkSynchronizer_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkSynchronizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b9ea160_ae9b_45f9_8407_4b493b163661);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkSynchronizer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub BeginDry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BeginDry: usize,
    pub EndDry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkUnprocessedInput(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInkUnprocessedInput {
    type Vtable = IInkUnprocessedInput_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkUnprocessedInput {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb4445e0_8398_4921_ac3b_ab978c5ba256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkUnprocessedInput_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerHovered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerHovered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerHovered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerHovered: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerExited: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerLost: usize,
    pub InkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenAndInkSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPenAndInkSettings {
    type Vtable = IPenAndInkSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IPenAndInkSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2ceb8f_0066_44a8_bb7a_b839b3deb8f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsHandwritingDirectlyIntoTextFieldEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PenHandedness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PenHandedness) -> ::windows::core::HRESULT,
    pub HandwritingLineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HandwritingLineHeight) -> ::windows::core::HRESULT,
    pub FontFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserConsentsToHandwritingTelemetryCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTouchHandwritingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenAndInkSettings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPenAndInkSettings2 {
    type Vtable = IPenAndInkSettings2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPenAndInkSettings2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3262da53_1f44_55e2_9929_ebf77e5481b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettings2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetPenHandedness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PenHandedness) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenAndInkSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPenAndInkSettingsStatics {
    type Vtable = IPenAndInkSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPenAndInkSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed6dd036_5708_5c3c_96db_f2f552eab641);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkDrawingAttributes(::windows::core::IUnknown);
impl InkDrawingAttributes {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkDrawingAttributes, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Color(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Color)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PenTip(&self) -> ::windows::core::Result<PenTipShape> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PenTip)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PenTipShape>(result__)
        }
    }
    pub fn SetPenTip(&self, value: PenTipShape) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPenTip)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSize(&self, value: super::super::super::Foundation::Size) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IgnorePressure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IgnorePressure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnorePressure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIgnorePressure)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn FitToCurve(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FitToCurve)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetFitToCurve(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFitToCurve)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PenTipTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PenTipTransform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPenTipTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPenTipTransform)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DrawAsHighlighter(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DrawAsHighlighter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDrawAsHighlighter(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDrawAsHighlighter)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkDrawingAttributesKind> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkDrawingAttributesKind>(result__)
        }
    }
    pub fn PencilProperties(&self) -> ::windows::core::Result<InkDrawingAttributesPencilProperties> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PencilProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkDrawingAttributesPencilProperties>(result__)
        }
    }
    pub fn IgnoreTilt(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IgnoreTilt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreTilt(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIgnoreTilt)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ModelerAttributes(&self) -> ::windows::core::Result<InkModelerAttributes> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelerAttributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkModelerAttributes>(result__)
        }
    }
    pub fn CreateForPencil() -> ::windows::core::Result<InkDrawingAttributes> {
        Self::IInkDrawingAttributesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForPencil)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkDrawingAttributes>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInkDrawingAttributesStatics<R, F: FnOnce(&IInkDrawingAttributesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkDrawingAttributes, IInkDrawingAttributesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InkDrawingAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkDrawingAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkDrawingAttributes {}
impl ::core::fmt::Debug for InkDrawingAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDrawingAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkDrawingAttributes;{97a2176c-6774-48ad-84f0-48f5a9be74f9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for InkDrawingAttributes {
    const IID: ::windows::core::GUID = <IInkDrawingAttributes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkDrawingAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributes";
}
::windows::core::interface_hierarchy!(InkDrawingAttributes, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkDrawingAttributes {}
unsafe impl ::core::marker::Sync for InkDrawingAttributes {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkDrawingAttributesPencilProperties(::windows::core::IUnknown);
impl InkDrawingAttributesPencilProperties {
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Opacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOpacity)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InkDrawingAttributesPencilProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkDrawingAttributesPencilProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkDrawingAttributesPencilProperties {}
impl ::core::fmt::Debug for InkDrawingAttributesPencilProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDrawingAttributesPencilProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributesPencilProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties;{4f2534cb-2d86-41bb-b0e8-e4c2a0253c52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for InkDrawingAttributesPencilProperties {
    const IID: ::windows::core::GUID = <IInkDrawingAttributesPencilProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkDrawingAttributesPencilProperties {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties";
}
::windows::core::interface_hierarchy!(InkDrawingAttributesPencilProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkDrawingAttributesPencilProperties {}
unsafe impl ::core::marker::Sync for InkDrawingAttributesPencilProperties {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkInputConfiguration(::windows::core::IUnknown);
impl InkInputConfiguration {
    pub fn IsPrimaryBarrelButtonInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrimaryBarrelButtonInputEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPrimaryBarrelButtonInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsPrimaryBarrelButtonInputEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsEraserInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEraserInputEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEraserInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsEraserInputEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsPenHapticFeedbackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPenHapticFeedbackEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPenHapticFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsPenHapticFeedbackEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InkInputConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkInputConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkInputConfiguration {}
impl ::core::fmt::Debug for InkInputConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInputConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkInputConfiguration;{93a68dc4-0b7b-49d7-b34f-9901e524dcf2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkInputConfiguration {
    type Vtable = IInkInputConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for InkInputConfiguration {
    const IID: ::windows::core::GUID = <IInkInputConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkInputConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputConfiguration";
}
::windows::core::interface_hierarchy!(InkInputConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkInputConfiguration {}
unsafe impl ::core::marker::Sync for InkInputConfiguration {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkInputProcessingConfiguration(::windows::core::IUnknown);
impl InkInputProcessingConfiguration {
    pub fn Mode(&self) -> ::windows::core::Result<InkInputProcessingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkInputProcessingMode>(result__)
        }
    }
    pub fn SetMode(&self, value: InkInputProcessingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RightDragAction(&self) -> ::windows::core::Result<InkInputRightDragAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RightDragAction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkInputRightDragAction>(result__)
        }
    }
    pub fn SetRightDragAction(&self, value: InkInputRightDragAction) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRightDragAction)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InkInputProcessingConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkInputProcessingConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkInputProcessingConfiguration {}
impl ::core::fmt::Debug for InkInputProcessingConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInputProcessingConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputProcessingConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkInputProcessingConfiguration;{2778d85e-33ca-4b06-a6d3-ac3945116d37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for InkInputProcessingConfiguration {
    const IID: ::windows::core::GUID = <IInkInputProcessingConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkInputProcessingConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputProcessingConfiguration";
}
::windows::core::interface_hierarchy!(InkInputProcessingConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkInputProcessingConfiguration {}
unsafe impl ::core::marker::Sync for InkInputProcessingConfiguration {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkManager(::windows::core::IUnknown);
impl InkManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkManager, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> ::windows::core::Result<InkManipulationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkManipulationMode>(result__)
        }
    }
    pub fn SetMode(&self, value: InkManipulationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ProcessPointerDown(&self, pointerpoint: &super::PointerPoint) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ProcessPointerDown)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pointerpoint)).ok() }
    }
    pub fn ProcessPointerUpdate(&self, pointerpoint: &super::PointerPoint) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessPointerUpdate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pointerpoint), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProcessPointerUp(&self, pointerpoint: &super::PointerPoint) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessPointerUp)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pointerpoint), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn SetDefaultDrawingAttributes(&self, drawingattributes: &InkDrawingAttributes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDefaultDrawingAttributes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(drawingattributes)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecognizeAsync2(&self, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecognizeAsync2)(::windows::core::Vtable::as_raw(this), recognitiontarget, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    pub fn SetDefaultRecognizer(&self, recognizer: &InkRecognizer) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDefaultRecognizer)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(recognizer)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecognizeAsync(&self, strokecollection: &InkStrokeContainer, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecognizeAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(strokecollection), recognitiontarget, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRecognizers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BoundingRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn AddStroke(&self, stroke: &InkStroke) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AddStroke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(stroke)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteSelected)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected(&self, translation: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveSelected)(::windows::core::Vtable::as_raw(this), translation, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SelectWithPolyLine<'a, P0, E0>(&self, polyline: P0) -> ::windows::core::Result<super::super::super::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectWithPolyLine)(::windows::core::Vtable::as_raw(this), polyline.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine(&self, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectWithLine)(::windows::core::Vtable::as_raw(this), from, to, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).CopySelectedToClipboard)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard(&self, position: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PasteFromClipboard)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanPasteFromClipboard)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, P0, E0>(&self, inputstream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadAsync)(::windows::core::Vtable::as_raw(this), inputstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, P0, E0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), outputstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, P0, E0>(&self, recognitionresults: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateRecognitionResults)(::windows::core::Vtable::as_raw(this), recognitionresults.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStrokes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRecognitionResults)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkManager {}
impl ::core::fmt::Debug for InkManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkManager;{4744737d-671b-4163-9c95-4e8d7a035fe1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkManager {
    type Vtable = IInkManager_Vtbl;
}
unsafe impl ::windows::core::Interface for InkManager {
    const IID: ::windows::core::GUID = <IInkManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkManager {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkManager";
}
::windows::core::interface_hierarchy!(InkManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InkManager> for IInkRecognizerContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: InkManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkManager> for IInkRecognizerContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InkManager> for ::windows::core::InParam<'a, IInkRecognizerContainer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkManager) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<InkManager> for IInkStrokeContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: InkManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkManager> for IInkStrokeContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InkManager> for ::windows::core::InParam<'a, IInkStrokeContainer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkManager) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkModelerAttributes(::windows::core::IUnknown);
impl InkModelerAttributes {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PredictionTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PredictionTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPredictionTime(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPredictionTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ScalingFactor(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScalingFactor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetScalingFactor(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScalingFactor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn UseVelocityBasedPressure(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UseVelocityBasedPressure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetUseVelocityBasedPressure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUseVelocityBasedPressure)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InkModelerAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkModelerAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkModelerAttributes {}
impl ::core::fmt::Debug for InkModelerAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkModelerAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkModelerAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkModelerAttributes;{bad31f27-0cd9-4bfd-b6f3-9e03ba8d7454})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkModelerAttributes {
    type Vtable = IInkModelerAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for InkModelerAttributes {
    const IID: ::windows::core::GUID = <IInkModelerAttributes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkModelerAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkModelerAttributes";
}
::windows::core::interface_hierarchy!(InkModelerAttributes, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkModelerAttributes {}
unsafe impl ::core::marker::Sync for InkModelerAttributes {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkPoint(::windows::core::IUnknown);
impl InkPoint {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pressure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn TiltX(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TiltX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn TiltY(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TiltY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPoint(position: super::super::super::Foundation::Point, pressure: f32) -> ::windows::core::Result<InkPoint> {
        Self::IInkPointFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInkPoint)(::windows::core::Vtable::as_raw(this), position, pressure, result__.as_mut_ptr()).from_abi::<InkPoint>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPointWithTiltAndTimestamp(position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64) -> ::windows::core::Result<InkPoint> {
        Self::IInkPointFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInkPointWithTiltAndTimestamp)(::windows::core::Vtable::as_raw(this), position, pressure, tiltx, tilty, timestamp, result__.as_mut_ptr()).from_abi::<InkPoint>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInkPointFactory<R, F: FnOnce(&IInkPointFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkPoint, IInkPointFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInkPointFactory2<R, F: FnOnce(&IInkPointFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkPoint, IInkPointFactory2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InkPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkPoint {}
impl ::core::fmt::Debug for InkPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPoint;{9f87272b-858c-46a5-9b41-d195970459fd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkPoint {
    type Vtable = IInkPoint_Vtbl;
}
unsafe impl ::windows::core::Interface for InkPoint {
    const IID: ::windows::core::GUID = <IInkPoint as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkPoint {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPoint";
}
::windows::core::interface_hierarchy!(InkPoint, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkPoint {}
unsafe impl ::core::marker::Sync for InkPoint {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkPresenter(::windows::core::IUnknown);
impl InkPresenter {
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInputEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsInputEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn InputDeviceTypes(&self) -> ::windows::core::Result<super::super::Core::CoreInputDeviceTypes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputDeviceTypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreInputDeviceTypes>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn SetInputDeviceTypes(&self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInputDeviceTypes)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn UnprocessedInput(&self) -> ::windows::core::Result<InkUnprocessedInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnprocessedInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkUnprocessedInput>(result__)
        }
    }
    pub fn StrokeInput(&self) -> ::windows::core::Result<InkStrokeInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokeInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkStrokeInput>(result__)
        }
    }
    pub fn InputProcessingConfiguration(&self) -> ::windows::core::Result<InkInputProcessingConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputProcessingConfiguration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkInputProcessingConfiguration>(result__)
        }
    }
    pub fn StrokeContainer(&self) -> ::windows::core::Result<InkStrokeContainer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokeContainer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkStrokeContainer>(result__)
        }
    }
    pub fn SetStrokeContainer(&self, value: &InkStrokeContainer) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStrokeContainer)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CopyDefaultDrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CopyDefaultDrawingAttributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkDrawingAttributes>(result__)
        }
    }
    pub fn UpdateDefaultDrawingAttributes(&self, value: &InkDrawingAttributes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateDefaultDrawingAttributes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActivateCustomDrying(&self) -> ::windows::core::Result<InkSynchronizer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivateCustomDrying)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkSynchronizer>(result__)
        }
    }
    pub fn SetPredefinedConfiguration(&self, value: InkPresenterPredefinedConfiguration) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPredefinedConfiguration)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StrokesCollected(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokesCollected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokesCollected(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStrokesCollected)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StrokesErased(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokesErased)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokesErased(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStrokesErased)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    pub fn HighContrastAdjustment(&self) -> ::windows::core::Result<InkHighContrastAdjustment> {
        let this = &::windows::core::Interface::cast::<IInkPresenter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HighContrastAdjustment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkHighContrastAdjustment>(result__)
        }
    }
    pub fn SetHighContrastAdjustment(&self, value: InkHighContrastAdjustment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenter2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetHighContrastAdjustment)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InputConfiguration(&self) -> ::windows::core::Result<InkInputConfiguration> {
        let this = &::windows::core::Interface::cast::<IInkPresenter3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputConfiguration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkInputConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for InkPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkPresenter {}
impl ::core::fmt::Debug for InkPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenter;{a69b70e2-887b-458f-b173-4fe4438930a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkPresenter {
    type Vtable = IInkPresenter_Vtbl;
}
unsafe impl ::windows::core::Interface for InkPresenter {
    const IID: ::windows::core::GUID = <IInkPresenter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkPresenter {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenter";
}
::windows::core::interface_hierarchy!(InkPresenter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkPresenter {}
unsafe impl ::core::marker::Sync for InkPresenter {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkPresenterProtractor(::windows::core::IUnknown);
impl InkPresenterProtractor {
    pub fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AreTickMarksVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAreTickMarksVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AreRaysVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AreRaysVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAreRaysVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAreRaysVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCenterMarkerVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCenterMarkerVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCenterMarkerVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsCenterMarkerVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsAngleReadoutVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAngleReadoutVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAngleReadoutVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsAngleReadoutVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsResizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsResizable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsResizable)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Radius(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Radius)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadius(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRadius)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AccentColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccentColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetAccentColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAccentColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Create(inkpresenter: &InkPresenter) -> ::windows::core::Result<InkPresenterProtractor> {
        Self::IInkPresenterProtractorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(inkpresenter), result__.as_mut_ptr()).from_abi::<InkPresenterProtractor>(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForegroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetForegroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Transform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransform)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IInkPresenterProtractorFactory<R, F: FnOnce(&IInkPresenterProtractorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkPresenterProtractor, IInkPresenterProtractorFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InkPresenterProtractor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkPresenterProtractor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkPresenterProtractor {}
impl ::core::fmt::Debug for InkPresenterProtractor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenterProtractor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterProtractor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenterProtractor;{7de3f2aa-ef6c-4e91-a73b-5b70d56fbd17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkPresenterProtractor {
    type Vtable = IInkPresenterProtractor_Vtbl;
}
unsafe impl ::windows::core::Interface for InkPresenterProtractor {
    const IID: ::windows::core::GUID = <IInkPresenterProtractor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkPresenterProtractor {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterProtractor";
}
::windows::core::interface_hierarchy!(InkPresenterProtractor, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InkPresenterProtractor> for IInkPresenterStencil {
    type Error = ::windows::core::Error;
    fn try_from(value: InkPresenterProtractor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkPresenterProtractor> for IInkPresenterStencil {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkPresenterProtractor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InkPresenterProtractor> for ::windows::core::InParam<'a, IInkPresenterStencil> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkPresenterProtractor) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for InkPresenterProtractor {}
unsafe impl ::core::marker::Sync for InkPresenterProtractor {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkPresenterRuler(::windows::core::IUnknown);
impl InkPresenterRuler {
    pub fn Length(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLength)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Width(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Width)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetWidth)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AreTickMarksVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAreTickMarksVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCompassVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCompassVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCompassVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsCompassVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Create(inkpresenter: &InkPresenter) -> ::windows::core::Result<InkPresenterRuler> {
        Self::IInkPresenterRulerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(inkpresenter), result__.as_mut_ptr()).from_abi::<InkPresenterRuler>(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForegroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetForegroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Transform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransform)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IInkPresenterRulerFactory<R, F: FnOnce(&IInkPresenterRulerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkPresenterRuler, IInkPresenterRulerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InkPresenterRuler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkPresenterRuler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkPresenterRuler {}
impl ::core::fmt::Debug for InkPresenterRuler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenterRuler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterRuler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenterRuler;{6cda7d5a-dec7-4dd7-877a-2133f183d48a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkPresenterRuler {
    type Vtable = IInkPresenterRuler_Vtbl;
}
unsafe impl ::windows::core::Interface for InkPresenterRuler {
    const IID: ::windows::core::GUID = <IInkPresenterRuler as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkPresenterRuler {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterRuler";
}
::windows::core::interface_hierarchy!(InkPresenterRuler, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InkPresenterRuler> for IInkPresenterStencil {
    type Error = ::windows::core::Error;
    fn try_from(value: InkPresenterRuler) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkPresenterRuler> for IInkPresenterStencil {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkPresenterRuler) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InkPresenterRuler> for ::windows::core::InParam<'a, IInkPresenterStencil> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkPresenterRuler) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for InkPresenterRuler {}
unsafe impl ::core::marker::Sync for InkPresenterRuler {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkRecognitionResult(::windows::core::IUnknown);
impl InkRecognitionResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BoundingRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextCandidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTextCandidates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStrokes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkRecognitionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkRecognitionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkRecognitionResult {}
impl ::core::fmt::Debug for InkRecognitionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognitionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognitionResult;{36461a94-5068-40ef-8a05-2c2fb60908a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkRecognitionResult {
    type Vtable = IInkRecognitionResult_Vtbl;
}
unsafe impl ::windows::core::Interface for InkRecognitionResult {
    const IID: ::windows::core::GUID = <IInkRecognitionResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkRecognitionResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognitionResult";
}
::windows::core::interface_hierarchy!(InkRecognitionResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkRecognitionResult {}
unsafe impl ::core::marker::Sync for InkRecognitionResult {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkRecognizer(::windows::core::IUnknown);
impl InkRecognizer {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for InkRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkRecognizer {}
impl ::core::fmt::Debug for InkRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognizer;{077ccea3-904d-442a-b151-aaca3631c43b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkRecognizer {
    type Vtable = IInkRecognizer_Vtbl;
}
unsafe impl ::windows::core::Interface for InkRecognizer {
    const IID: ::windows::core::GUID = <IInkRecognizer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizer";
}
::windows::core::interface_hierarchy!(InkRecognizer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkRecognizerContainer(::windows::core::IUnknown);
impl InkRecognizerContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkRecognizerContainer, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDefaultRecognizer(&self, recognizer: &InkRecognizer) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDefaultRecognizer)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(recognizer)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecognizeAsync(&self, strokecollection: &InkStrokeContainer, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecognizeAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(strokecollection), recognitiontarget, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRecognizers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkRecognizerContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkRecognizerContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkRecognizerContainer {}
impl ::core::fmt::Debug for InkRecognizerContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognizerContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognizerContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognizerContainer;{a74d9a31-8047-4698-a912-f82a5085012f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkRecognizerContainer {
    type Vtable = IInkRecognizerContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for InkRecognizerContainer {
    const IID: ::windows::core::GUID = <IInkRecognizerContainer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkRecognizerContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizerContainer";
}
::windows::core::interface_hierarchy!(InkRecognizerContainer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InkRecognizerContainer> for IInkRecognizerContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: InkRecognizerContainer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkRecognizerContainer> for IInkRecognizerContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkRecognizerContainer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InkRecognizerContainer> for ::windows::core::InParam<'a, IInkRecognizerContainer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkRecognizerContainer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkStroke(::windows::core::IUnknown);
impl InkStroke {
    pub fn DrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DrawingAttributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkDrawingAttributes>(result__)
        }
    }
    pub fn SetDrawingAttributes(&self, value: &InkDrawingAttributes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDrawingAttributes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BoundingRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn Selected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Selected)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelected(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSelected)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Recognized(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Recognized)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRenderingSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRenderingSegments)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>>(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkStroke>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PointTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointTransform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPointTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPointTransform)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInkPoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkPoint>> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInkPoints)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkPoint>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StrokeStartedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokeStartedTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStrokeStartedTime<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStrokeStartedTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StrokeDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokeDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStrokeDuration<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStrokeDuration)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkStroke4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStroke {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStroke {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStroke {}
impl ::core::fmt::Debug for InkStroke {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStroke").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStroke {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStroke;{15144d60-cce3-4fcf-9d52-11518ab6afd4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkStroke {
    type Vtable = IInkStroke_Vtbl;
}
unsafe impl ::windows::core::Interface for InkStroke {
    const IID: ::windows::core::GUID = <IInkStroke as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStroke";
}
::windows::core::interface_hierarchy!(InkStroke, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkStroke {}
unsafe impl ::core::marker::Sync for InkStroke {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkStrokeBuilder(::windows::core::IUnknown);
impl InkStrokeBuilder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkStrokeBuilder, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn BeginStroke(&self, pointerpoint: &super::PointerPoint) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).BeginStroke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pointerpoint)).ok() }
    }
    pub fn AppendToStroke(&self, pointerpoint: &super::PointerPoint) -> ::windows::core::Result<super::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendToStroke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pointerpoint), result__.as_mut_ptr()).from_abi::<super::PointerPoint>(result__)
        }
    }
    pub fn EndStroke(&self, pointerpoint: &super::PointerPoint) -> ::windows::core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EndStroke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pointerpoint), result__.as_mut_ptr()).from_abi::<InkStroke>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateStroke<'a, P0, E0>(&self, points: P0) -> ::windows::core::Result<InkStroke>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateStroke)(::windows::core::Vtable::as_raw(this), points.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<InkStroke>(result__)
        }
    }
    pub fn SetDefaultDrawingAttributes(&self, drawingattributes: &InkDrawingAttributes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDefaultDrawingAttributes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(drawingattributes)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub fn CreateStrokeFromInkPoints<'a, P0, E0>(&self, inkpoints: P0, transform: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<InkStroke>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<InkPoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStrokeBuilder2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateStrokeFromInkPoints)(::windows::core::Vtable::as_raw(this), inkpoints.try_into().map_err(|e| e.into())?.abi(), transform, result__.as_mut_ptr()).from_abi::<InkStroke>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub fn CreateStrokeFromInkPoints2<'a, P0, E0, P1, E1, P2, E2>(&self, inkpoints: P0, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: P1, strokeduration: P2) -> ::windows::core::Result<InkStroke>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<InkPoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStrokeBuilder3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateStrokeFromInkPoints)(::windows::core::Vtable::as_raw(this), inkpoints.try_into().map_err(|e| e.into())?.abi(), transform, strokestartedtime.try_into().map_err(|e| e.into())?.abi(), strokeduration.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<InkStroke>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokeBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokeBuilder {}
impl ::core::fmt::Debug for InkStrokeBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokeBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeBuilder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeBuilder;{82bbd1dc-1c63-41dc-9e07-4b4a70ced801})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkStrokeBuilder {
    type Vtable = IInkStrokeBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for InkStrokeBuilder {
    const IID: ::windows::core::GUID = <IInkStrokeBuilder as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkStrokeBuilder {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeBuilder";
}
::windows::core::interface_hierarchy!(InkStrokeBuilder, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkStrokeContainer(::windows::core::IUnknown);
impl InkStrokeContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InkStrokeContainer, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BoundingRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn AddStroke(&self, stroke: &InkStroke) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddStroke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(stroke)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteSelected)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected(&self, translation: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveSelected)(::windows::core::Vtable::as_raw(this), translation, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SelectWithPolyLine<'a, P0, E0>(&self, polyline: P0) -> ::windows::core::Result<super::super::super::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectWithPolyLine)(::windows::core::Vtable::as_raw(this), polyline.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine(&self, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectWithLine)(::windows::core::Vtable::as_raw(this), from, to, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CopySelectedToClipboard)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard(&self, position: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PasteFromClipboard)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanPasteFromClipboard)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, P0, E0>(&self, inputstream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadAsync)(::windows::core::Vtable::as_raw(this), inputstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, P0, E0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), outputstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, P0, E0>(&self, recognitionresults: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateRecognitionResults)(::windows::core::Vtable::as_raw(this), recognitionresults.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStrokes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRecognitionResults)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddStrokes<'a, P0, E0>(&self, strokes: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<InkStroke>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AddStrokes)(::windows::core::Vtable::as_raw(this), strokes.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveWithFormatAsync<'a, P0, E0>(&self, outputstream: P0, inkpersistenceformat: InkPersistenceFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveWithFormatAsync)(::windows::core::Vtable::as_raw(this), outputstream.try_into().map_err(|e| e.into())?.abi(), inkpersistenceformat, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    pub fn GetStrokeById(&self, id: u32) -> ::windows::core::Result<InkStroke> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStrokeById)(::windows::core::Vtable::as_raw(this), id, result__.as_mut_ptr()).from_abi::<InkStroke>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokeContainer {}
impl ::core::fmt::Debug for InkStrokeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokeContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeContainer;{22accbc6-faa9-4f14-b68c-f6cee670ae16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkStrokeContainer {
    type Vtable = IInkStrokeContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for InkStrokeContainer {
    const IID: ::windows::core::GUID = <IInkStrokeContainer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkStrokeContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeContainer";
}
::windows::core::interface_hierarchy!(InkStrokeContainer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InkStrokeContainer> for IInkStrokeContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: InkStrokeContainer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkStrokeContainer> for IInkStrokeContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkStrokeContainer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InkStrokeContainer> for ::windows::core::InParam<'a, IInkStrokeContainer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkStrokeContainer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkStrokeInput(::windows::core::IUnknown);
impl InkStrokeInput {
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeStarted(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokeStarted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeStarted(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStrokeStarted)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeContinued(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokeContinued)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeContinued(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStrokeContinued)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeEnded(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokeEnded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeEnded(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStrokeEnded)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeCanceled(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StrokeCanceled)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeCanceled(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStrokeCanceled)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    pub fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InkPresenter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkPresenter>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokeInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokeInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokeInput {}
impl ::core::fmt::Debug for InkStrokeInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokeInput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeInput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeInput;{cf2ffe7b-5e10-43c6-a080-88f26e1dc67d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkStrokeInput {
    type Vtable = IInkStrokeInput_Vtbl;
}
unsafe impl ::windows::core::Interface for InkStrokeInput {
    const IID: ::windows::core::GUID = <IInkStrokeInput as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkStrokeInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeInput";
}
::windows::core::interface_hierarchy!(InkStrokeInput, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkStrokeInput {}
unsafe impl ::core::marker::Sync for InkStrokeInput {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkStrokeRenderingSegment(::windows::core::IUnknown);
impl InkStrokeRenderingSegment {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BezierControlPoint1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BezierControlPoint1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BezierControlPoint2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BezierControlPoint2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pressure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn TiltX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TiltX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn TiltY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TiltY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Twist(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Twist)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokeRenderingSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokeRenderingSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokeRenderingSegment {}
impl ::core::fmt::Debug for InkStrokeRenderingSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokeRenderingSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeRenderingSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeRenderingSegment;{68510f1f-88e3-477a-a2fa-569f5f1f9bd5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegment_Vtbl;
}
unsafe impl ::windows::core::Interface for InkStrokeRenderingSegment {
    const IID: ::windows::core::GUID = <IInkStrokeRenderingSegment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkStrokeRenderingSegment {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeRenderingSegment";
}
::windows::core::interface_hierarchy!(InkStrokeRenderingSegment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkStrokeRenderingSegment {}
unsafe impl ::core::marker::Sync for InkStrokeRenderingSegment {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkStrokesCollectedEventArgs(::windows::core::IUnknown);
impl InkStrokesCollectedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Strokes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokesCollectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokesCollectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokesCollectedEventArgs {}
impl ::core::fmt::Debug for InkStrokesCollectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokesCollectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokesCollectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokesCollectedEventArgs;{c4f3f229-1938-495c-b4d9-6de4b08d4811})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for InkStrokesCollectedEventArgs {
    const IID: ::windows::core::GUID = <IInkStrokesCollectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkStrokesCollectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesCollectedEventArgs";
}
::windows::core::interface_hierarchy!(InkStrokesCollectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkStrokesErasedEventArgs(::windows::core::IUnknown);
impl InkStrokesErasedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Strokes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokesErasedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokesErasedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokesErasedEventArgs {}
impl ::core::fmt::Debug for InkStrokesErasedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokesErasedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokesErasedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokesErasedEventArgs;{a4216a22-1503-4ebf-8ff5-2de84584a8aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for InkStrokesErasedEventArgs {
    const IID: ::windows::core::GUID = <IInkStrokesErasedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkStrokesErasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesErasedEventArgs";
}
::windows::core::interface_hierarchy!(InkStrokesErasedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkSynchronizer(::windows::core::IUnknown);
impl InkSynchronizer {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BeginDry(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginDry)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    pub fn EndDry(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).EndDry)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for InkSynchronizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkSynchronizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkSynchronizer {}
impl ::core::fmt::Debug for InkSynchronizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkSynchronizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkSynchronizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkSynchronizer;{9b9ea160-ae9b-45f9-8407-4b493b163661})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkSynchronizer {
    type Vtable = IInkSynchronizer_Vtbl;
}
unsafe impl ::windows::core::Interface for InkSynchronizer {
    const IID: ::windows::core::GUID = <IInkSynchronizer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkSynchronizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkSynchronizer";
}
::windows::core::interface_hierarchy!(InkSynchronizer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkUnprocessedInput(::windows::core::IUnknown);
impl InkUnprocessedInput {
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerEntered(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerEntered)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePointerEntered)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerHovered(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerHovered)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerHovered(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePointerHovered)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerExited(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerExited)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePointerExited)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerPressed(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerPressed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePointerPressed)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerMoved(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerMoved)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePointerMoved)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerReleased(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerReleased)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePointerReleased)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerLost(&self, handler: &super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerLost)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerLost(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePointerLost)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    pub fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InkPresenter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InkPresenter>(result__)
        }
    }
}
impl ::core::clone::Clone for InkUnprocessedInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkUnprocessedInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkUnprocessedInput {}
impl ::core::fmt::Debug for InkUnprocessedInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkUnprocessedInput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkUnprocessedInput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkUnprocessedInput;{db4445e0-8398-4921-ac3b-ab978c5ba256})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InkUnprocessedInput {
    type Vtable = IInkUnprocessedInput_Vtbl;
}
unsafe impl ::windows::core::Interface for InkUnprocessedInput {
    const IID: ::windows::core::GUID = <IInkUnprocessedInput as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkUnprocessedInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkUnprocessedInput";
}
::windows::core::interface_hierarchy!(InkUnprocessedInput, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkUnprocessedInput {}
unsafe impl ::core::marker::Sync for InkUnprocessedInput {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct PenAndInkSettings(::windows::core::IUnknown);
impl PenAndInkSettings {
    pub fn IsHandwritingDirectlyIntoTextFieldEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHandwritingDirectlyIntoTextFieldEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PenHandedness(&self) -> ::windows::core::Result<PenHandedness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PenHandedness)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PenHandedness>(result__)
        }
    }
    pub fn HandwritingLineHeight(&self) -> ::windows::core::Result<HandwritingLineHeight> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HandwritingLineHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HandwritingLineHeight>(result__)
        }
    }
    pub fn FontFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn UserConsentsToHandwritingTelemetryCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserConsentsToHandwritingTelemetryCollection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTouchHandwritingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTouchHandwritingEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPenHandedness(&self, value: PenHandedness) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPenAndInkSettings2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPenHandedness)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<PenAndInkSettings> {
        Self::IPenAndInkSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PenAndInkSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenAndInkSettingsStatics<R, F: FnOnce(&IPenAndInkSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PenAndInkSettings, IPenAndInkSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PenAndInkSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenAndInkSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenAndInkSettings {}
impl ::core::fmt::Debug for PenAndInkSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenAndInkSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PenAndInkSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.PenAndInkSettings;{bc2ceb8f-0066-44a8-bb7a-b839b3deb8f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PenAndInkSettings {
    type Vtable = IPenAndInkSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for PenAndInkSettings {
    const IID: ::windows::core::GUID = <IPenAndInkSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PenAndInkSettings {
    const NAME: &'static str = "Windows.UI.Input.Inking.PenAndInkSettings";
}
::windows::core::interface_hierarchy!(PenAndInkSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenAndInkSettings {}
unsafe impl ::core::marker::Sync for PenAndInkSettings {}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HandwritingLineHeight(pub i32);
impl HandwritingLineHeight {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for HandwritingLineHeight {}
impl ::core::clone::Clone for HandwritingLineHeight {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HandwritingLineHeight {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HandwritingLineHeight {
    type Abi = Self;
}
impl ::core::fmt::Debug for HandwritingLineHeight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandwritingLineHeight").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HandwritingLineHeight {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.HandwritingLineHeight;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkDrawingAttributesKind(pub i32);
impl InkDrawingAttributesKind {
    pub const Default: Self = Self(0i32);
    pub const Pencil: Self = Self(1i32);
}
impl ::core::marker::Copy for InkDrawingAttributesKind {}
impl ::core::clone::Clone for InkDrawingAttributesKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkDrawingAttributesKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkDrawingAttributesKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkDrawingAttributesKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDrawingAttributesKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributesKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkDrawingAttributesKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkHighContrastAdjustment(pub i32);
impl InkHighContrastAdjustment {
    pub const UseSystemColorsWhenNecessary: Self = Self(0i32);
    pub const UseSystemColors: Self = Self(1i32);
    pub const UseOriginalColors: Self = Self(2i32);
}
impl ::core::marker::Copy for InkHighContrastAdjustment {}
impl ::core::clone::Clone for InkHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkHighContrastAdjustment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkHighContrastAdjustment {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkHighContrastAdjustment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkHighContrastAdjustment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkHighContrastAdjustment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkHighContrastAdjustment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkInputProcessingMode(pub i32);
impl InkInputProcessingMode {
    pub const None: Self = Self(0i32);
    pub const Inking: Self = Self(1i32);
    pub const Erasing: Self = Self(2i32);
}
impl ::core::marker::Copy for InkInputProcessingMode {}
impl ::core::clone::Clone for InkInputProcessingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkInputProcessingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkInputProcessingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkInputProcessingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInputProcessingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputProcessingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputProcessingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkInputRightDragAction(pub i32);
impl InkInputRightDragAction {
    pub const LeaveUnprocessed: Self = Self(0i32);
    pub const AllowProcessing: Self = Self(1i32);
}
impl ::core::marker::Copy for InkInputRightDragAction {}
impl ::core::clone::Clone for InkInputRightDragAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkInputRightDragAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkInputRightDragAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkInputRightDragAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInputRightDragAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputRightDragAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputRightDragAction;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkManipulationMode(pub i32);
impl InkManipulationMode {
    pub const Inking: Self = Self(0i32);
    pub const Erasing: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
}
impl ::core::marker::Copy for InkManipulationMode {}
impl ::core::clone::Clone for InkManipulationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkManipulationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkManipulationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkManipulationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkManipulationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkManipulationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkManipulationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkPersistenceFormat(pub i32);
impl InkPersistenceFormat {
    pub const GifWithEmbeddedIsf: Self = Self(0i32);
    pub const Isf: Self = Self(1i32);
}
impl ::core::marker::Copy for InkPersistenceFormat {}
impl ::core::clone::Clone for InkPersistenceFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkPersistenceFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkPersistenceFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPersistenceFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPersistenceFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPersistenceFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPersistenceFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkPresenterPredefinedConfiguration(pub i32);
impl InkPresenterPredefinedConfiguration {
    pub const SimpleSinglePointer: Self = Self(0i32);
    pub const SimpleMultiplePointer: Self = Self(1i32);
}
impl ::core::marker::Copy for InkPresenterPredefinedConfiguration {}
impl ::core::clone::Clone for InkPresenterPredefinedConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkPresenterPredefinedConfiguration {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkPresenterPredefinedConfiguration {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPresenterPredefinedConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenterPredefinedConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterPredefinedConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterPredefinedConfiguration;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkPresenterStencilKind(pub i32);
impl InkPresenterStencilKind {
    pub const Other: Self = Self(0i32);
    pub const Ruler: Self = Self(1i32);
    pub const Protractor: Self = Self(2i32);
}
impl ::core::marker::Copy for InkPresenterStencilKind {}
impl ::core::clone::Clone for InkPresenterStencilKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkPresenterStencilKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkPresenterStencilKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPresenterStencilKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenterStencilKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterStencilKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterStencilKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkRecognitionTarget(pub i32);
impl InkRecognitionTarget {
    pub const All: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Recent: Self = Self(2i32);
}
impl ::core::marker::Copy for InkRecognitionTarget {}
impl ::core::clone::Clone for InkRecognitionTarget {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkRecognitionTarget {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InkRecognitionTarget {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRecognitionTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognitionTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkRecognitionTarget;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PenHandedness(pub i32);
impl PenHandedness {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for PenHandedness {}
impl ::core::clone::Clone for PenHandedness {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PenHandedness {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PenHandedness {
    type Abi = Self;
}
impl ::core::fmt::Debug for PenHandedness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenHandedness").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PenHandedness {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenHandedness;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PenTipShape(pub i32);
impl PenTipShape {
    pub const Circle: Self = Self(0i32);
    pub const Rectangle: Self = Self(1i32);
}
impl ::core::marker::Copy for PenTipShape {}
impl ::core::clone::Clone for PenTipShape {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PenTipShape {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PenTipShape {
    type Abi = Self;
}
impl ::core::fmt::Debug for PenTipShape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTipShape").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PenTipShape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenTipShape;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
