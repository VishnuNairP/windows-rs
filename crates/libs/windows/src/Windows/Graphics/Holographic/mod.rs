#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCamera {
    type Vtable = IHolographicCamera_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCamera {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4e98445_9bed_4980_9ba0_e87680d1cb74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RenderTargetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderTargetSize: usize,
    pub ViewportScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetViewportScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetNearPlaneDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub SetFarPlaneDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCamera2 {
    type Vtable = IHolographicCamera2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCamera2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb55b9f1a_ba8c_4f84_ad79_2e7e1e2450f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LeftViewportParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RightViewportParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCamera3 {
    type Vtable = IHolographicCamera3_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCamera3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45aa4fb3_7b59_524e_4a3f_4a6ad6650477);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPrimaryLayerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPrimaryLayerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MaxQuadLayerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub QuadLayers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QuadLayers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCamera4 {
    type Vtable = IHolographicCamera4_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCamera4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2531d6_4723_4f39_a9a5_9d05181d9b44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanOverrideViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCamera5 {
    type Vtable = IHolographicCamera5_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCamera5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x229706f2_628d_4ef5_9c08_a63fdd7787c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsHardwareContentProtectionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHardwareContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsHardwareContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera6(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCamera6 {
    type Vtable = IHolographicCamera6_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCamera6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0209194f_632d_5154_ab52_0b5d15b12505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ViewConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraPose(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCameraPose {
    type Vtable = IHolographicCameraPose_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCameraPose {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d7d7e30_12de_45bd_912b_c7f6561599d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraPose_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HolographicCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Viewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Viewport: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetViewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetViewTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectionTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicStereoTransform) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectionTransform: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetCullingFrustum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetCullingFrustum: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetVisibleFrustum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetVisibleFrustum: usize,
    pub NearPlaneDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub FarPlaneDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraPose2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCameraPose2 {
    type Vtable = IHolographicCameraPose2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCameraPose2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x232be073_5d2d_4560_814e_2697c4fce16b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraPose2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub OverrideViewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, coordinatesystemtoviewtransform: HolographicStereoTransform) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    OverrideViewTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub OverrideProjectionTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectiontransform: HolographicStereoTransform) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OverrideProjectionTransform: usize,
    #[cfg(feature = "Foundation")]
    pub OverrideViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leftviewport: super::super::Foundation::Rect, rightviewport: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverrideViewport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCameraRenderingParameters {
    type Vtable = IHolographicCameraRenderingParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCameraRenderingParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8eac2ed1_5bf4_4e16_8236_ae0800c11d0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPointWithNormal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPointWithNormal: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPointWithNormalLinearVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3, linearvelocity: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPointWithNormalLinearVelocity: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11BackBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11BackBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCameraRenderingParameters2 {
    type Vtable = IHolographicCameraRenderingParameters2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCameraRenderingParameters2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x261270e3_b696_4634_94d6_be0681643599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReprojectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicReprojectionMode) -> ::windows::core::HRESULT,
    pub SetReprojectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HolographicReprojectionMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CommitDirect3D11DepthBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CommitDirect3D11DepthBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCameraRenderingParameters3 {
    type Vtable = IHolographicCameraRenderingParameters3_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCameraRenderingParameters3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1aa513f_136d_4b06_b9d4_e4b914cd0683);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCameraRenderingParameters4 {
    type Vtable = IHolographicCameraRenderingParameters4_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCameraRenderingParameters4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0878fa4c_e163_57dc_82b7_c406ab3e0537);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DepthReprojectionMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicDepthReprojectionMethod) -> ::windows::core::HRESULT,
    pub SetDepthReprojectionMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HolographicDepthReprojectionMethod) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraViewportParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicCameraViewportParameters {
    type Vtable = IHolographicCameraViewportParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicCameraViewportParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80cdf3f7_842a_41e1_93ed_5692ab1fbb10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraViewportParameters_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub HiddenAreaMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HiddenAreaMesh: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub VisibleAreaMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VisibleAreaMesh: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicDisplay(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicDisplay {
    type Vtable = IHolographicDisplay_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicDisplay {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9acea414_1d9f_4090_a388_90c06f6eae9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxViewportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxViewportSize: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsOpaque: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicAdapterId) -> ::windows::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub SpatialLocator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    SpatialLocator: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicDisplay2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicDisplay2 {
    type Vtable = IHolographicDisplay2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicDisplay2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75ac3f82_e755_436c_8d96_4d32d131473e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RefreshRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicDisplay3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicDisplay3 {
    type Vtable = IHolographicDisplay3_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicDisplay3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc4c6ac6_6480_5008_b29e_157d77c843f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryGetViewConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: HolographicViewConfigurationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicDisplayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicDisplayStatics {
    type Vtable = IHolographicDisplayStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicDisplayStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb374983_e7b0_4841_8355_3ae5b536e9a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplayStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicFrame {
    type Vtable = IHolographicFrame_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6988eb6_a8b9_3054_a6eb_d624b6536375);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedCameras: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedCameras: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemovedCameras: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemovedCameras: usize,
    pub GetRenderingParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, camerapose: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub CurrentPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateCurrentPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PresentUsingCurrentPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicFramePresentResult) -> ::windows::core::HRESULT,
    pub PresentUsingCurrentPredictionWithBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitbehavior: HolographicFramePresentWaitBehavior, result__: *mut HolographicFramePresentResult) -> ::windows::core::HRESULT,
    pub WaitForFrameToFinish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrame2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicFrame2 {
    type Vtable = IHolographicFrame2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicFrame2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x283f37bf_3bf2_5e91_6633_870574e6f217);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetQuadLayerUpdateParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, layer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrame3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicFrame3 {
    type Vtable = IHolographicFrame3_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicFrame3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5e964c9_8a27_55d3_9f98_94530d369052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicFrameId) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFramePrediction(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicFramePrediction {
    type Vtable = IHolographicFramePrediction_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicFramePrediction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520f4de1_5c0a_4e79_a81e_6abe02bb2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePrediction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CameraPoses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CameraPoses: usize,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicFramePresentationMonitor(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IHolographicFramePresentationMonitor {
    type Vtable = IHolographicFramePresentationMonitor_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IHolographicFramePresentationMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca87256c_6fae_428e_bb83_25dfee51136b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePresentationMonitor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ReadReports: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicFramePresentationReport(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IHolographicFramePresentationReport {
    type Vtable = IHolographicFramePresentationReport_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IHolographicFramePresentationReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80baf614_f2f4_4c8a_8de3_065c78f6d5de);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePresentationReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CompositorGpuDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CompositorGpuDuration: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AppGpuDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AppGpuDuration: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AppGpuOverrun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AppGpuOverrun: usize,
    #[cfg(feature = "deprecated")]
    pub MissedPresentationOpportunityCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MissedPresentationOpportunityCount: usize,
    #[cfg(feature = "deprecated")]
    pub PresentationCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresentationCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrameRenderingReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicFrameRenderingReport {
    type Vtable = IHolographicFrameRenderingReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicFrameRenderingReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05f32de4_e384_51b3_b934_f0d3a0f78606);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameRenderingReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicFrameId) -> ::windows::core::HRESULT,
    pub MissedLatchCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeFrameReadyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeFrameReadyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeActualGpuFinishTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeActualGpuFinishTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTargetLatchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTargetLatchTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrameScanoutMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicFrameScanoutMonitor {
    type Vtable = IHolographicFrameScanoutMonitor_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicFrameScanoutMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e83efa9_843c_5401_8095_9bc1b8b08638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameScanoutMonitor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrameScanoutReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicFrameScanoutReport {
    type Vtable = IHolographicFrameScanoutReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicFrameScanoutReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ebbe606_03a0_5ca0_b46e_bba068d7233f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameScanoutReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RenderingReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MissedScanoutCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeLatchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeLatchTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeScanoutStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeScanoutStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativePhotonTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativePhotonTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicQuadLayer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicQuadLayer {
    type Vtable = IHolographicQuadLayer_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicQuadLayer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x903460c9_c9d9_5d5c_41ac_a2d5ab0fd331);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicQuadLayerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicQuadLayerFactory {
    type Vtable = IHolographicQuadLayerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicQuadLayerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa67538f3_5a14_5a10_489a_455065b37b76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub CreateWithPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))]
    CreateWithPixelFormat: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicQuadLayerUpdateParameters {
    type Vtable = IHolographicQuadLayerUpdateParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicQuadLayerUpdateParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b0ea3b0_798d_5bca_55c2_2c0c762ebb08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParameters_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub AcquireBufferToUpdateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    AcquireBufferToUpdateContent: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateViewport: usize,
    pub UpdateContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpdateExtents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpdateExtents: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UpdateLocationWithStationaryMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UpdateLocationWithStationaryMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpdateLocationWithDisplayRelativeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpdateLocationWithDisplayRelativeMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParameters2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicQuadLayerUpdateParameters2 {
    type Vtable = IHolographicQuadLayerUpdateParameters2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicQuadLayerUpdateParameters2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f33d32d_82c1_46c1_8980_3cb70d98182b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParameters2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanAcquireWithHardwareProtection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub AcquireBufferToUpdateContentWithHardwareProtection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    AcquireBufferToUpdateContentWithHardwareProtection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpace(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicSpace {
    type Vtable = IHolographicSpace_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpace {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4380dba6_5e78_434f_807c_3433d1efe8b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrimaryAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicAdapterId) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub SetDirect3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    SetDirect3D11Device: usize,
    #[cfg(feature = "Foundation")]
    pub CameraAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraAdded: usize,
    #[cfg(feature = "Foundation")]
    pub CameraRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraRemoved: usize,
    pub CreateNextFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpace2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicSpace2 {
    type Vtable = IHolographicSpace2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpace2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f81a9a8_b7ff_4883_9827_7d677287ea70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UserPresence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicSpaceUserPresence) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserPresenceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserPresenceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserPresenceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserPresenceChanged: usize,
    pub WaitForNextFrameReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WaitForNextFrameReadyWithHeadStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedheadstartduration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForNextFrameReadyWithHeadStart: usize,
    #[cfg(feature = "deprecated")]
    pub CreateFramePresentationMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxqueuedreports: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateFramePresentationMonitor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpace3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicSpace3 {
    type Vtable = IHolographicSpace3_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpace3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf1733d1_f224_587e_8d71_1e8fc8f07b1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFrameScanoutMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxqueuedreports: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceCameraAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicSpaceCameraAddedEventArgs {
    type Vtable = IHolographicSpaceCameraAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpaceCameraAddedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58f1da35_bbb3_3c8f_993d_6c80e7feb99f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceCameraAddedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Camera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceCameraRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicSpaceCameraRemovedEventArgs {
    type Vtable = IHolographicSpaceCameraRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpaceCameraRemovedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x805444a8_f2ae_322e_8da9_836a0a95a4c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceCameraRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Camera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicSpaceStatics {
    type Vtable = IHolographicSpaceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpaceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x364e6064_c8f2_3ba1_8391_66b8489e67fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub CreateForCoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CreateForCoreWindow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicSpaceStatics2 {
    type Vtable = IHolographicSpaceStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpaceStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e777088_75fc_48af_8758_0652f6f07c59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsAvailableChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsAvailableChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicSpaceStatics3 {
    type Vtable = IHolographicSpaceStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpaceStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b00de3d_b1a3_4dfe_8e79_fec5909e6df8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsConfigured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicViewConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicViewConfiguration {
    type Vtable = IHolographicViewConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicViewConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c1de6e6_67e9_5004_b02c_67a3a122b576);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicViewConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub NativeRenderTargetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NativeRenderTargetSize: usize,
    #[cfg(feature = "Foundation")]
    pub RenderTargetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderTargetSize: usize,
    #[cfg(feature = "Foundation")]
    pub RequestRenderTargetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRenderTargetSize: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedPixelFormats: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetPixelFormat: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicViewConfigurationKind) -> ::windows::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicViewConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicViewConfiguration2 {
    type Vtable = IHolographicViewConfiguration2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicViewConfiguration2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe241756e_e0d0_5019_9af5_1b165bc2f54e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicViewConfiguration2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDepthReprojectionMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDepthReprojectionMethods: usize,
}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicCamera(::windows::core::IUnknown);
impl HolographicCamera {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderTargetSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn ViewportScaleFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ViewportScaleFactor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetViewportScaleFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetViewportScaleFactor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStereo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetNearPlaneDistance(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNearPlaneDistance)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetFarPlaneDistance(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFarPlaneDistance)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LeftViewportParameters(&self) -> ::windows::core::Result<HolographicCameraViewportParameters> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LeftViewportParameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCameraViewportParameters>(result__)
        }
    }
    pub fn RightViewportParameters(&self) -> ::windows::core::Result<HolographicCameraViewportParameters> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RightViewportParameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCameraViewportParameters>(result__)
        }
    }
    pub fn Display(&self) -> ::windows::core::Result<HolographicDisplay> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Display)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicDisplay>(result__)
        }
    }
    pub fn IsPrimaryLayerEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrimaryLayerEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPrimaryLayerEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsPrimaryLayerEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MaxQuadLayerCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxQuadLayerCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn QuadLayers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<HolographicQuadLayer>> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QuadLayers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<HolographicQuadLayer>>(result__)
        }
    }
    pub fn CanOverrideViewport(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanOverrideViewport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHardwareContentProtectionSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHardwareContentProtectionSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHardwareContentProtectionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHardwareContentProtectionEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHardwareContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsHardwareContentProtectionEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ViewConfiguration(&self) -> ::windows::core::Result<HolographicViewConfiguration> {
        let this = &::windows::core::Interface::cast::<IHolographicCamera6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ViewConfiguration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicViewConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicCamera {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicCamera {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicCamera {}
impl ::core::fmt::Debug for HolographicCamera {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicCamera").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicCamera {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCamera;{e4e98445-9bed-4980-9ba0-e87680d1cb74})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicCamera {
    type Vtable = IHolographicCamera_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicCamera {
    const IID: ::windows::core::GUID = <IHolographicCamera as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicCamera {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCamera";
}
::windows::core::interface_hierarchy!(HolographicCamera, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicCamera {}
unsafe impl ::core::marker::Sync for HolographicCamera {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicCameraPose(::windows::core::IUnknown);
impl HolographicCameraPose {
    pub fn HolographicCamera(&self) -> ::windows::core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HolographicCamera)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCamera>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Viewport(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Viewport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetViewTransform(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem) -> ::windows::core::Result<super::super::Foundation::IReference<HolographicStereoTransform>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetViewTransform)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<HolographicStereoTransform>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectionTransform(&self) -> ::windows::core::Result<HolographicStereoTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProjectionTransform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicStereoTransform>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetCullingFrustum(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetCullingFrustum)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetVisibleFrustum(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetVisibleFrustum)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>(result__)
        }
    }
    pub fn NearPlaneDistance(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NearPlaneDistance)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn FarPlaneDistance(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FarPlaneDistance)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn OverrideViewTransform(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem, coordinatesystemtoviewtransform: HolographicStereoTransform) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).OverrideViewTransform)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), coordinatesystemtoviewtransform).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OverrideProjectionTransform(&self, projectiontransform: HolographicStereoTransform) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).OverrideProjectionTransform)(::windows::core::Vtable::as_raw(this), projectiontransform).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OverrideViewport(&self, leftviewport: super::super::Foundation::Rect, rightviewport: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).OverrideViewport)(::windows::core::Vtable::as_raw(this), leftviewport, rightviewport).ok() }
    }
}
impl ::core::clone::Clone for HolographicCameraPose {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicCameraPose {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicCameraPose {}
impl ::core::fmt::Debug for HolographicCameraPose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicCameraPose").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicCameraPose {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraPose;{0d7d7e30-12de-45bd-912b-c7f6561599d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicCameraPose {
    type Vtable = IHolographicCameraPose_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicCameraPose {
    const IID: ::windows::core::GUID = <IHolographicCameraPose as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicCameraPose {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraPose";
}
::windows::core::interface_hierarchy!(HolographicCameraPose, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicCameraPose {}
unsafe impl ::core::marker::Sync for HolographicCameraPose {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicCameraRenderingParameters(::windows::core::IUnknown);
impl HolographicCameraRenderingParameters {
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPoint(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem, position: super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFocusPoint)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), position).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPointWithNormal(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFocusPointWithNormal)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), position, normal).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPointWithNormalLinearVelocity(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3, linearvelocity: super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFocusPointWithNormalLinearVelocity)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), position, normal, linearvelocity).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Direct3D11Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11BackBuffer(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Direct3D11BackBuffer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    pub fn ReprojectionMode(&self) -> ::windows::core::Result<HolographicReprojectionMode> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReprojectionMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicReprojectionMode>(result__)
        }
    }
    pub fn SetReprojectionMode(&self, value: HolographicReprojectionMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetReprojectionMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CommitDirect3D11DepthBuffer<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).CommitDirect3D11DepthBuffer)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn IsContentProtectionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraRenderingParameters3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsContentProtectionEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraRenderingParameters3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsContentProtectionEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DepthReprojectionMethod(&self) -> ::windows::core::Result<HolographicDepthReprojectionMethod> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraRenderingParameters4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DepthReprojectionMethod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicDepthReprojectionMethod>(result__)
        }
    }
    pub fn SetDepthReprojectionMethod(&self, value: HolographicDepthReprojectionMethod) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicCameraRenderingParameters4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDepthReprojectionMethod)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for HolographicCameraRenderingParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicCameraRenderingParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicCameraRenderingParameters {}
impl ::core::fmt::Debug for HolographicCameraRenderingParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicCameraRenderingParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicCameraRenderingParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraRenderingParameters;{8eac2ed1-5bf4-4e16-8236-ae0800c11d0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicCameraRenderingParameters {
    type Vtable = IHolographicCameraRenderingParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicCameraRenderingParameters {
    const IID: ::windows::core::GUID = <IHolographicCameraRenderingParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicCameraRenderingParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraRenderingParameters";
}
::windows::core::interface_hierarchy!(HolographicCameraRenderingParameters, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicCameraRenderingParameters {}
unsafe impl ::core::marker::Sync for HolographicCameraRenderingParameters {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicCameraViewportParameters(::windows::core::IUnknown);
impl HolographicCameraViewportParameters {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn HiddenAreaMesh(&self) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector2>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HiddenAreaMesh)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<super::super::Foundation::Numerics::Vector2>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn VisibleAreaMesh(&self) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector2>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisibleAreaMesh)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<super::super::Foundation::Numerics::Vector2>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::clone::Clone for HolographicCameraViewportParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicCameraViewportParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicCameraViewportParameters {}
impl ::core::fmt::Debug for HolographicCameraViewportParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicCameraViewportParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicCameraViewportParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraViewportParameters;{80cdf3f7-842a-41e1-93ed-5692ab1fbb10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicCameraViewportParameters {
    type Vtable = IHolographicCameraViewportParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicCameraViewportParameters {
    const IID: ::windows::core::GUID = <IHolographicCameraViewportParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicCameraViewportParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraViewportParameters";
}
::windows::core::interface_hierarchy!(HolographicCameraViewportParameters, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicCameraViewportParameters {}
unsafe impl ::core::marker::Sync for HolographicCameraViewportParameters {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicDisplay(::windows::core::IUnknown);
impl HolographicDisplay {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxViewportSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxViewportSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStereo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsOpaque(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOpaque)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AdapterId(&self) -> ::windows::core::Result<HolographicAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdapterId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicAdapterId>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    #[cfg(feature = "Perception_Spatial")]
    pub fn SpatialLocator(&self) -> ::windows::core::Result<super::super::Perception::Spatial::SpatialLocator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SpatialLocator)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Perception::Spatial::SpatialLocator>(result__)
        }
    }
    pub fn RefreshRate(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IHolographicDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RefreshRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn TryGetViewConfiguration(&self, kind: HolographicViewConfigurationKind) -> ::windows::core::Result<HolographicViewConfiguration> {
        let this = &::windows::core::Interface::cast::<IHolographicDisplay3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetViewConfiguration)(::windows::core::Vtable::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<HolographicViewConfiguration>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<HolographicDisplay> {
        Self::IHolographicDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicDisplay>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicDisplayStatics<R, F: FnOnce(&IHolographicDisplayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HolographicDisplay, IHolographicDisplayStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HolographicDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicDisplay {}
impl ::core::fmt::Debug for HolographicDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicDisplay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicDisplay;{9acea414-1d9f-4090-a388-90c06f6eae9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicDisplay {
    type Vtable = IHolographicDisplay_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicDisplay {
    const IID: ::windows::core::GUID = <IHolographicDisplay as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicDisplay {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicDisplay";
}
::windows::core::interface_hierarchy!(HolographicDisplay, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicDisplay {}
unsafe impl ::core::marker::Sync for HolographicDisplay {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicFrame(::windows::core::IUnknown);
impl HolographicFrame {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddedCameras(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddedCameras)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCamera>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemovedCameras(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemovedCameras)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCamera>>(result__)
        }
    }
    pub fn GetRenderingParameters(&self, camerapose: &HolographicCameraPose) -> ::windows::core::Result<HolographicCameraRenderingParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRenderingParameters)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(camerapose), result__.as_mut_ptr()).from_abi::<HolographicCameraRenderingParameters>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn CurrentPrediction(&self) -> ::windows::core::Result<HolographicFramePrediction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentPrediction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFramePrediction>(result__)
        }
    }
    pub fn UpdateCurrentPrediction(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateCurrentPrediction)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn PresentUsingCurrentPrediction(&self) -> ::windows::core::Result<HolographicFramePresentResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PresentUsingCurrentPrediction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFramePresentResult>(result__)
        }
    }
    pub fn PresentUsingCurrentPredictionWithBehavior(&self, waitbehavior: HolographicFramePresentWaitBehavior) -> ::windows::core::Result<HolographicFramePresentResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PresentUsingCurrentPredictionWithBehavior)(::windows::core::Vtable::as_raw(this), waitbehavior, result__.as_mut_ptr()).from_abi::<HolographicFramePresentResult>(result__)
        }
    }
    pub fn WaitForFrameToFinish(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WaitForFrameToFinish)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn GetQuadLayerUpdateParameters(&self, layer: &HolographicQuadLayer) -> ::windows::core::Result<HolographicQuadLayerUpdateParameters> {
        let this = &::windows::core::Interface::cast::<IHolographicFrame2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetQuadLayerUpdateParameters)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(layer), result__.as_mut_ptr()).from_abi::<HolographicQuadLayerUpdateParameters>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<HolographicFrameId> {
        let this = &::windows::core::Interface::cast::<IHolographicFrame3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFrameId>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFrame {}
impl ::core::fmt::Debug for HolographicFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrame;{c6988eb6-a8b9-3054-a6eb-d624b6536375})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicFrame {
    type Vtable = IHolographicFrame_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicFrame {
    const IID: ::windows::core::GUID = <IHolographicFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicFrame {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrame";
}
::windows::core::interface_hierarchy!(HolographicFrame, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicFrame {}
unsafe impl ::core::marker::Sync for HolographicFrame {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicFramePrediction(::windows::core::IUnknown);
impl HolographicFramePrediction {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CameraPoses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CameraPoses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception\"`*"]
    #[cfg(feature = "Perception")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFramePrediction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFramePrediction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFramePrediction {}
impl ::core::fmt::Debug for HolographicFramePrediction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePrediction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicFramePrediction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePrediction;{520f4de1-5c0a-4e79-a81e-6abe02bb2739})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicFramePrediction {
    type Vtable = IHolographicFramePrediction_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicFramePrediction {
    const IID: ::windows::core::GUID = <IHolographicFramePrediction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicFramePrediction {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePrediction";
}
::windows::core::interface_hierarchy!(HolographicFramePrediction, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicFramePrediction {}
unsafe impl ::core::marker::Sync for HolographicFramePrediction {}
#[doc = "*Required features: `\"Graphics_Holographic\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct HolographicFramePresentationMonitor(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl HolographicFramePresentationMonitor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadReports)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for HolographicFramePresentationMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for HolographicFramePresentationMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for HolographicFramePresentationMonitor {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for HolographicFramePresentationMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePresentationMonitor").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for HolographicFramePresentationMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePresentationMonitor;{ca87256c-6fae-428e-bb83-25dfee51136b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for HolographicFramePresentationMonitor {
    type Vtable = IHolographicFramePresentationMonitor_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for HolographicFramePresentationMonitor {
    const IID: ::windows::core::GUID = <IHolographicFramePresentationMonitor as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for HolographicFramePresentationMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePresentationMonitor";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(HolographicFramePresentationMonitor, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<HolographicFramePresentationMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HolographicFramePresentationMonitor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&HolographicFramePresentationMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HolographicFramePresentationMonitor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&HolographicFramePresentationMonitor> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HolographicFramePresentationMonitor) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for HolographicFramePresentationMonitor {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for HolographicFramePresentationMonitor {}
#[doc = "*Required features: `\"Graphics_Holographic\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct HolographicFramePresentationReport(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl HolographicFramePresentationReport {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CompositorGpuDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompositorGpuDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AppGpuDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppGpuDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AppGpuOverrun(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppGpuOverrun)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MissedPresentationOpportunityCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MissedPresentationOpportunityCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PresentationCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PresentationCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for HolographicFramePresentationReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for HolographicFramePresentationReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for HolographicFramePresentationReport {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for HolographicFramePresentationReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePresentationReport").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for HolographicFramePresentationReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePresentationReport;{80baf614-f2f4-4c8a-8de3-065c78f6d5de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for HolographicFramePresentationReport {
    type Vtable = IHolographicFramePresentationReport_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for HolographicFramePresentationReport {
    const IID: ::windows::core::GUID = <IHolographicFramePresentationReport as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for HolographicFramePresentationReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePresentationReport";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(HolographicFramePresentationReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for HolographicFramePresentationReport {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for HolographicFramePresentationReport {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicFrameRenderingReport(::windows::core::IUnknown);
impl HolographicFrameRenderingReport {
    pub fn FrameId(&self) -> ::windows::core::Result<HolographicFrameId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFrameId>(result__)
        }
    }
    pub fn MissedLatchCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MissedLatchCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeFrameReadyTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemRelativeFrameReadyTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeActualGpuFinishTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemRelativeActualGpuFinishTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTargetLatchTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemRelativeTargetLatchTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFrameRenderingReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFrameRenderingReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFrameRenderingReport {}
impl ::core::fmt::Debug for HolographicFrameRenderingReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFrameRenderingReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicFrameRenderingReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameRenderingReport;{05f32de4-e384-51b3-b934-f0d3a0f78606})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicFrameRenderingReport {
    type Vtable = IHolographicFrameRenderingReport_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicFrameRenderingReport {
    const IID: ::windows::core::GUID = <IHolographicFrameRenderingReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicFrameRenderingReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameRenderingReport";
}
::windows::core::interface_hierarchy!(HolographicFrameRenderingReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicFrameRenderingReport {}
unsafe impl ::core::marker::Sync for HolographicFrameRenderingReport {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicFrameScanoutMonitor(::windows::core::IUnknown);
impl HolographicFrameScanoutMonitor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadReports)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFrameScanoutMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFrameScanoutMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFrameScanoutMonitor {}
impl ::core::fmt::Debug for HolographicFrameScanoutMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFrameScanoutMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicFrameScanoutMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameScanoutMonitor;{7e83efa9-843c-5401-8095-9bc1b8b08638})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicFrameScanoutMonitor {
    type Vtable = IHolographicFrameScanoutMonitor_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicFrameScanoutMonitor {
    const IID: ::windows::core::GUID = <IHolographicFrameScanoutMonitor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicFrameScanoutMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameScanoutMonitor";
}
::windows::core::interface_hierarchy!(HolographicFrameScanoutMonitor, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HolographicFrameScanoutMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HolographicFrameScanoutMonitor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HolographicFrameScanoutMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HolographicFrameScanoutMonitor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&HolographicFrameScanoutMonitor> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HolographicFrameScanoutMonitor) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HolographicFrameScanoutMonitor {}
unsafe impl ::core::marker::Sync for HolographicFrameScanoutMonitor {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicFrameScanoutReport(::windows::core::IUnknown);
impl HolographicFrameScanoutReport {
    pub fn RenderingReport(&self) -> ::windows::core::Result<HolographicFrameRenderingReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderingReport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFrameRenderingReport>(result__)
        }
    }
    pub fn MissedScanoutCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MissedScanoutCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeLatchTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemRelativeLatchTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeScanoutStartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemRelativeScanoutStartTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativePhotonTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemRelativePhotonTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFrameScanoutReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFrameScanoutReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFrameScanoutReport {}
impl ::core::fmt::Debug for HolographicFrameScanoutReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFrameScanoutReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicFrameScanoutReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameScanoutReport;{0ebbe606-03a0-5ca0-b46e-bba068d7233f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicFrameScanoutReport {
    type Vtable = IHolographicFrameScanoutReport_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicFrameScanoutReport {
    const IID: ::windows::core::GUID = <IHolographicFrameScanoutReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicFrameScanoutReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameScanoutReport";
}
::windows::core::interface_hierarchy!(HolographicFrameScanoutReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicFrameScanoutReport {}
unsafe impl ::core::marker::Sync for HolographicFrameScanoutReport {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicQuadLayer(::windows::core::IUnknown);
impl HolographicQuadLayer {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PixelFormat(&self) -> ::windows::core::Result<super::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create(size: super::super::Foundation::Size) -> ::windows::core::Result<HolographicQuadLayer> {
        Self::IHolographicQuadLayerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), size, result__.as_mut_ptr()).from_abi::<HolographicQuadLayer>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub fn CreateWithPixelFormat(size: super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat) -> ::windows::core::Result<HolographicQuadLayer> {
        Self::IHolographicQuadLayerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithPixelFormat)(::windows::core::Vtable::as_raw(this), size, pixelformat, result__.as_mut_ptr()).from_abi::<HolographicQuadLayer>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicQuadLayerFactory<R, F: FnOnce(&IHolographicQuadLayerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HolographicQuadLayer, IHolographicQuadLayerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HolographicQuadLayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicQuadLayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicQuadLayer {}
impl ::core::fmt::Debug for HolographicQuadLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicQuadLayer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicQuadLayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicQuadLayer;{903460c9-c9d9-5d5c-41ac-a2d5ab0fd331})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicQuadLayer {
    type Vtable = IHolographicQuadLayer_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicQuadLayer {
    const IID: ::windows::core::GUID = <IHolographicQuadLayer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicQuadLayer {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicQuadLayer";
}
::windows::core::interface_hierarchy!(HolographicQuadLayer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HolographicQuadLayer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HolographicQuadLayer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HolographicQuadLayer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HolographicQuadLayer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&HolographicQuadLayer> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HolographicQuadLayer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HolographicQuadLayer {}
unsafe impl ::core::marker::Sync for HolographicQuadLayer {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicQuadLayerUpdateParameters(::windows::core::IUnknown);
impl HolographicQuadLayerUpdateParameters {
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn AcquireBufferToUpdateContent(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcquireBufferToUpdateContent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateViewport(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateViewport)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn UpdateContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateContentProtectionEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpdateExtents(&self, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateExtents)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UpdateLocationWithStationaryMode(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateLocationWithStationaryMode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), position, orientation).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpdateLocationWithDisplayRelativeMode(&self, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateLocationWithDisplayRelativeMode)(::windows::core::Vtable::as_raw(this), position, orientation).ok() }
    }
    pub fn CanAcquireWithHardwareProtection(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHolographicQuadLayerUpdateParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanAcquireWithHardwareProtection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn AcquireBufferToUpdateContentWithHardwareProtection(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = &::windows::core::Interface::cast::<IHolographicQuadLayerUpdateParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcquireBufferToUpdateContentWithHardwareProtection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicQuadLayerUpdateParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicQuadLayerUpdateParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicQuadLayerUpdateParameters {}
impl ::core::fmt::Debug for HolographicQuadLayerUpdateParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicQuadLayerUpdateParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicQuadLayerUpdateParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters;{2b0ea3b0-798d-5bca-55c2-2c0c762ebb08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicQuadLayerUpdateParameters {
    type Vtable = IHolographicQuadLayerUpdateParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicQuadLayerUpdateParameters {
    const IID: ::windows::core::GUID = <IHolographicQuadLayerUpdateParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicQuadLayerUpdateParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters";
}
::windows::core::interface_hierarchy!(HolographicQuadLayerUpdateParameters, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicQuadLayerUpdateParameters {}
unsafe impl ::core::marker::Sync for HolographicQuadLayerUpdateParameters {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicSpace(::windows::core::IUnknown);
impl HolographicSpace {
    pub fn PrimaryAdapterId(&self) -> ::windows::core::Result<HolographicAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrimaryAdapterId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicAdapterId>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn SetDirect3D11Device<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDirect3D11Device)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraAdded(&self, handler: &super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CameraAdded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraAdded(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCameraAdded)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraRemoved(&self, handler: &super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CameraRemoved)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraRemoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCameraRemoved)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    pub fn CreateNextFrame(&self) -> ::windows::core::Result<HolographicFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNextFrame)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFrame>(result__)
        }
    }
    pub fn UserPresence(&self) -> ::windows::core::Result<HolographicSpaceUserPresence> {
        let this = &::windows::core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserPresence)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicSpaceUserPresence>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserPresenceChanged(&self, handler: &super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserPresenceChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserPresenceChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUserPresenceChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn WaitForNextFrameReady(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).WaitForNextFrameReady)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WaitForNextFrameReadyWithHeadStart(&self, requestedheadstartduration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).WaitForNextFrameReadyWithHeadStart)(::windows::core::Vtable::as_raw(this), requestedheadstartduration).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateFramePresentationMonitor(&self, maxqueuedreports: u32) -> ::windows::core::Result<HolographicFramePresentationMonitor> {
        let this = &::windows::core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFramePresentationMonitor)(::windows::core::Vtable::as_raw(this), maxqueuedreports, result__.as_mut_ptr()).from_abi::<HolographicFramePresentationMonitor>(result__)
        }
    }
    pub fn CreateFrameScanoutMonitor(&self, maxqueuedreports: u32) -> ::windows::core::Result<HolographicFrameScanoutMonitor> {
        let this = &::windows::core::Interface::cast::<IHolographicSpace3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFrameScanoutMonitor)(::windows::core::Vtable::as_raw(this), maxqueuedreports, result__.as_mut_ptr()).from_abi::<HolographicFrameScanoutMonitor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn CreateForCoreWindow(window: &super::super::UI::Core::CoreWindow) -> ::windows::core::Result<HolographicSpace> {
        Self::IHolographicSpaceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForCoreWindow)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(window), result__.as_mut_ptr()).from_abi::<HolographicSpace>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsAvailable() -> ::windows::core::Result<bool> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAvailable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsAvailableChanged(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAvailableChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsAvailableChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IHolographicSpaceStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveIsAvailableChanged)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    pub fn IsConfigured() -> ::windows::core::Result<bool> {
        Self::IHolographicSpaceStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsConfigured)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicSpaceStatics<R, F: FnOnce(&IHolographicSpaceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HolographicSpace, IHolographicSpaceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHolographicSpaceStatics2<R, F: FnOnce(&IHolographicSpaceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HolographicSpace, IHolographicSpaceStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHolographicSpaceStatics3<R, F: FnOnce(&IHolographicSpaceStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HolographicSpace, IHolographicSpaceStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HolographicSpace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicSpace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicSpace {}
impl ::core::fmt::Debug for HolographicSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicSpace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpace;{4380dba6-5e78-434f-807c-3433d1efe8b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicSpace {
    type Vtable = IHolographicSpace_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicSpace {
    const IID: ::windows::core::GUID = <IHolographicSpace as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicSpace {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpace";
}
::windows::core::interface_hierarchy!(HolographicSpace, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicSpace {}
unsafe impl ::core::marker::Sync for HolographicSpace {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicSpaceCameraAddedEventArgs(::windows::core::IUnknown);
impl HolographicSpaceCameraAddedEventArgs {
    pub fn Camera(&self) -> ::windows::core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Camera)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCamera>(result__)
        }
    }
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
impl ::core::clone::Clone for HolographicSpaceCameraAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicSpaceCameraAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicSpaceCameraAddedEventArgs {}
impl ::core::fmt::Debug for HolographicSpaceCameraAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicSpaceCameraAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicSpaceCameraAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs;{58f1da35-bbb3-3c8f-993d-6c80e7feb99f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicSpaceCameraAddedEventArgs {
    type Vtable = IHolographicSpaceCameraAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicSpaceCameraAddedEventArgs {
    const IID: ::windows::core::GUID = <IHolographicSpaceCameraAddedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicSpaceCameraAddedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs";
}
::windows::core::interface_hierarchy!(HolographicSpaceCameraAddedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicSpaceCameraAddedEventArgs {}
unsafe impl ::core::marker::Sync for HolographicSpaceCameraAddedEventArgs {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicSpaceCameraRemovedEventArgs(::windows::core::IUnknown);
impl HolographicSpaceCameraRemovedEventArgs {
    pub fn Camera(&self) -> ::windows::core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Camera)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCamera>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicSpaceCameraRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicSpaceCameraRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicSpaceCameraRemovedEventArgs {}
impl ::core::fmt::Debug for HolographicSpaceCameraRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicSpaceCameraRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicSpaceCameraRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs;{805444a8-f2ae-322e-8da9-836a0a95a4c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicSpaceCameraRemovedEventArgs {
    type Vtable = IHolographicSpaceCameraRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicSpaceCameraRemovedEventArgs {
    const IID: ::windows::core::GUID = <IHolographicSpaceCameraRemovedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicSpaceCameraRemovedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs";
}
::windows::core::interface_hierarchy!(HolographicSpaceCameraRemovedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicSpaceCameraRemovedEventArgs {}
unsafe impl ::core::marker::Sync for HolographicSpaceCameraRemovedEventArgs {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicViewConfiguration(::windows::core::IUnknown);
impl HolographicViewConfiguration {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NativeRenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NativeRenderTargetSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderTargetSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestRenderTargetSize(&self, size: super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestRenderTargetSize)(::windows::core::Vtable::as_raw(this), size, result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedPixelFormats)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PixelFormat(&self) -> ::windows::core::Result<super::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetPixelFormat(&self, value: super::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPixelFormat)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStereo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RefreshRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RefreshRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<HolographicViewConfigurationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicViewConfigurationKind>(result__)
        }
    }
    pub fn Display(&self) -> ::windows::core::Result<HolographicDisplay> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Display)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicDisplay>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDepthReprojectionMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>> {
        let this = &::windows::core::Interface::cast::<IHolographicViewConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedDepthReprojectionMethods)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicViewConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicViewConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicViewConfiguration {}
impl ::core::fmt::Debug for HolographicViewConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicViewConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicViewConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicViewConfiguration;{5c1de6e6-67e9-5004-b02c-67a3a122b576})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HolographicViewConfiguration {
    type Vtable = IHolographicViewConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for HolographicViewConfiguration {
    const IID: ::windows::core::GUID = <IHolographicViewConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicViewConfiguration {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicViewConfiguration";
}
::windows::core::interface_hierarchy!(HolographicViewConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicViewConfiguration {}
unsafe impl ::core::marker::Sync for HolographicViewConfiguration {}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HolographicDepthReprojectionMethod(pub i32);
impl HolographicDepthReprojectionMethod {
    pub const DepthReprojection: Self = Self(0i32);
    pub const AutoPlanar: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicDepthReprojectionMethod {}
impl ::core::clone::Clone for HolographicDepthReprojectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicDepthReprojectionMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HolographicDepthReprojectionMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicDepthReprojectionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicDepthReprojectionMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicDepthReprojectionMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicDepthReprojectionMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HolographicFramePresentResult(pub i32);
impl HolographicFramePresentResult {
    pub const Success: Self = Self(0i32);
    pub const DeviceRemoved: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicFramePresentResult {}
impl ::core::clone::Clone for HolographicFramePresentResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicFramePresentResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HolographicFramePresentResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicFramePresentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePresentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicFramePresentResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicFramePresentResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HolographicFramePresentWaitBehavior(pub i32);
impl HolographicFramePresentWaitBehavior {
    pub const WaitForFrameToFinish: Self = Self(0i32);
    pub const DoNotWaitForFrameToFinish: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicFramePresentWaitBehavior {}
impl ::core::clone::Clone for HolographicFramePresentWaitBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicFramePresentWaitBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HolographicFramePresentWaitBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicFramePresentWaitBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePresentWaitBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicFramePresentWaitBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicFramePresentWaitBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HolographicReprojectionMode(pub i32);
impl HolographicReprojectionMode {
    pub const PositionAndOrientation: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for HolographicReprojectionMode {}
impl ::core::clone::Clone for HolographicReprojectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicReprojectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HolographicReprojectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicReprojectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicReprojectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicReprojectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicReprojectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HolographicSpaceUserPresence(pub i32);
impl HolographicSpaceUserPresence {
    pub const Absent: Self = Self(0i32);
    pub const PresentPassive: Self = Self(1i32);
    pub const PresentActive: Self = Self(2i32);
}
impl ::core::marker::Copy for HolographicSpaceUserPresence {}
impl ::core::clone::Clone for HolographicSpaceUserPresence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicSpaceUserPresence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HolographicSpaceUserPresence {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicSpaceUserPresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicSpaceUserPresence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicSpaceUserPresence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicSpaceUserPresence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HolographicViewConfigurationKind(pub i32);
impl HolographicViewConfigurationKind {
    pub const Display: Self = Self(0i32);
    pub const PhotoVideoCamera: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicViewConfigurationKind {}
impl ::core::clone::Clone for HolographicViewConfigurationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicViewConfigurationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HolographicViewConfigurationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicViewConfigurationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicViewConfigurationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicViewConfigurationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicViewConfigurationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
pub struct HolographicAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for HolographicAdapterId {}
impl ::core::clone::Clone for HolographicAdapterId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HolographicAdapterId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HolographicAdapterId").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
unsafe impl ::windows::core::Abi for HolographicAdapterId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HolographicAdapterId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicAdapterId;u4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for HolographicAdapterId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HolographicAdapterId>()) == 0 }
    }
}
impl ::core::cmp::Eq for HolographicAdapterId {}
impl ::core::default::Default for HolographicAdapterId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
pub struct HolographicFrameId {
    pub Value: u64,
}
impl ::core::marker::Copy for HolographicFrameId {}
impl ::core::clone::Clone for HolographicFrameId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HolographicFrameId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HolographicFrameId").field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for HolographicFrameId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HolographicFrameId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicFrameId;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for HolographicFrameId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HolographicFrameId>()) == 0 }
    }
}
impl ::core::cmp::Eq for HolographicFrameId {}
impl ::core::default::Default for HolographicFrameId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Holographic\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct HolographicStereoTransform {
    pub Left: super::super::Foundation::Numerics::Matrix4x4,
    pub Right: super::super::Foundation::Numerics::Matrix4x4,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for HolographicStereoTransform {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for HolographicStereoTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for HolographicStereoTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HolographicStereoTransform").field("Left", &self.Left).field("Right", &self.Right).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for HolographicStereoTransform {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for HolographicStereoTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicStereoTransform;struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for HolographicStereoTransform {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HolographicStereoTransform>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for HolographicStereoTransform {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for HolographicStereoTransform {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
