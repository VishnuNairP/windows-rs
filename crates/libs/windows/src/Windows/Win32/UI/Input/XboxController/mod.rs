#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn XInputEnable<'a, P0>(enable: P0)
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn XInputEnable(enable: super::super::super::Foundation::BOOL);
    }
    XInputEnable(enable.into())
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[inline]
pub unsafe fn XInputGetAudioDeviceIds(dwuserindex: u32, prenderdeviceid: ::windows::core::PWSTR, prendercount: ::core::option::Option<*mut u32>, pcapturedeviceid: ::windows::core::PWSTR, pcapturecount: ::core::option::Option<*mut u32>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn XInputGetAudioDeviceIds(dwuserindex: u32, prenderdeviceid: ::windows::core::PWSTR, prendercount: *mut u32, pcapturedeviceid: ::windows::core::PWSTR, pcapturecount: *mut u32) -> u32;
    }
    XInputGetAudioDeviceIds(dwuserindex, ::core::mem::transmute(prenderdeviceid), ::core::mem::transmute(prendercount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcapturedeviceid), ::core::mem::transmute(pcapturecount.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[inline]
pub unsafe fn XInputGetBatteryInformation(dwuserindex: u32, devtype: BATTERY_DEVTYPE, pbatteryinformation: *mut XINPUT_BATTERY_INFORMATION) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn XInputGetBatteryInformation(dwuserindex: u32, devtype: BATTERY_DEVTYPE, pbatteryinformation: *mut XINPUT_BATTERY_INFORMATION) -> u32;
    }
    XInputGetBatteryInformation(dwuserindex, devtype, ::core::mem::transmute(pbatteryinformation))
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[inline]
pub unsafe fn XInputGetCapabilities(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn XInputGetCapabilities(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32;
    }
    XInputGetCapabilities(dwuserindex, dwflags, ::core::mem::transmute(pcapabilities))
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[inline]
pub unsafe fn XInputGetKeystroke(dwuserindex: u32, dwreserved: u32, pkeystroke: *mut XINPUT_KEYSTROKE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn XInputGetKeystroke(dwuserindex: u32, dwreserved: u32, pkeystroke: *mut XINPUT_KEYSTROKE) -> u32;
    }
    XInputGetKeystroke(dwuserindex, dwreserved, ::core::mem::transmute(pkeystroke))
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[inline]
pub unsafe fn XInputGetState(dwuserindex: u32, pstate: *mut XINPUT_STATE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn XInputGetState(dwuserindex: u32, pstate: *mut XINPUT_STATE) -> u32;
    }
    XInputGetState(dwuserindex, ::core::mem::transmute(pstate))
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[inline]
pub unsafe fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32;
    }
    XInputSetState(dwuserindex, ::core::mem::transmute(pvibration))
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DLL: ::windows::core::PCWSTR = ::windows::w!("xinput1_4.dll");
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DLL_A: ::windows::core::PCSTR = ::windows::s!("xinput1_4.dll");
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DLL_W: ::windows::core::PCWSTR = ::windows::w!("xinput1_4.dll");
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XUSER_INDEX_ANY: u32 = 255u32;
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XUSER_MAX_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BATTERY_DEVTYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_DEVTYPE_GAMEPAD: BATTERY_DEVTYPE = BATTERY_DEVTYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_DEVTYPE_HEADSET: BATTERY_DEVTYPE = BATTERY_DEVTYPE(1u32);
impl ::core::marker::Copy for BATTERY_DEVTYPE {}
impl ::core::clone::Clone for BATTERY_DEVTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BATTERY_DEVTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BATTERY_DEVTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BATTERY_DEVTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_DEVTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BATTERY_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_LEVEL_EMPTY: BATTERY_LEVEL = BATTERY_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_LEVEL_LOW: BATTERY_LEVEL = BATTERY_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_LEVEL_MEDIUM: BATTERY_LEVEL = BATTERY_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_LEVEL_FULL: BATTERY_LEVEL = BATTERY_LEVEL(3u32);
impl ::core::marker::Copy for BATTERY_LEVEL {}
impl ::core::clone::Clone for BATTERY_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BATTERY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BATTERY_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for BATTERY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BATTERY_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_TYPE_DISCONNECTED: BATTERY_TYPE = BATTERY_TYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_TYPE_WIRED: BATTERY_TYPE = BATTERY_TYPE(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_TYPE_ALKALINE: BATTERY_TYPE = BATTERY_TYPE(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_TYPE_NIMH: BATTERY_TYPE = BATTERY_TYPE(3u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const BATTERY_TYPE_UNKNOWN: BATTERY_TYPE = BATTERY_TYPE(255u32);
impl ::core::marker::Copy for BATTERY_TYPE {}
impl ::core::clone::Clone for BATTERY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BATTERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BATTERY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BATTERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XINPUT_CAPABILITIES_FLAGS(pub u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_CAPS_VOICE_SUPPORTED: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(4u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_CAPS_FFB_SUPPORTED: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(1u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_CAPS_WIRELESS: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(2u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_CAPS_PMD_SUPPORTED: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(8u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_CAPS_NO_NAVIGATION: XINPUT_CAPABILITIES_FLAGS = XINPUT_CAPABILITIES_FLAGS(16u16);
impl ::core::marker::Copy for XINPUT_CAPABILITIES_FLAGS {}
impl ::core::clone::Clone for XINPUT_CAPABILITIES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XINPUT_CAPABILITIES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XINPUT_CAPABILITIES_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XINPUT_CAPABILITIES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_CAPABILITIES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XINPUT_CAPABILITIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XINPUT_CAPABILITIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XINPUT_DEVSUBTYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_GAMEPAD: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_UNKNOWN: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_WHEEL: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_ARCADE_STICK: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(3u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_FLIGHT_STICK: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(4u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_DANCE_PAD: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(5u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_GUITAR: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(6u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_GUITAR_ALTERNATE: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(7u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_DRUM_KIT: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(8u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_GUITAR_BASS: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(11u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVSUBTYPE_ARCADE_PAD: XINPUT_DEVSUBTYPE = XINPUT_DEVSUBTYPE(19u32);
impl ::core::marker::Copy for XINPUT_DEVSUBTYPE {}
impl ::core::clone::Clone for XINPUT_DEVSUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XINPUT_DEVSUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XINPUT_DEVSUBTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XINPUT_DEVSUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_DEVSUBTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XINPUT_DEVTYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_DEVTYPE_GAMEPAD: XINPUT_DEVTYPE = XINPUT_DEVTYPE(1u32);
impl ::core::marker::Copy for XINPUT_DEVTYPE {}
impl ::core::clone::Clone for XINPUT_DEVTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XINPUT_DEVTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XINPUT_DEVTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XINPUT_DEVTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_DEVTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XINPUT_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_FLAG_ALL: XINPUT_FLAG = XINPUT_FLAG(0u32);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_FLAG_GAMEPAD: XINPUT_FLAG = XINPUT_FLAG(1u32);
impl ::core::marker::Copy for XINPUT_FLAG {}
impl ::core::clone::Clone for XINPUT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XINPUT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XINPUT_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for XINPUT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XINPUT_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XINPUT_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XINPUT_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XINPUT_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XINPUT_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XINPUT_GAMEPAD_BUTTON_FLAGS(pub u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_DPAD_UP: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(1u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_DPAD_DOWN: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(2u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_DPAD_LEFT: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(4u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_DPAD_RIGHT: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(8u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_START: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(16u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_BACK: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(32u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_LEFT_THUMB: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(64u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_RIGHT_THUMB: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(128u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_LEFT_SHOULDER: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(256u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_RIGHT_SHOULDER: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(512u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_A: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(4096u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_B: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(8192u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_X: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(16384u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_Y: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(32768u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(7849u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(8689u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_GAMEPAD_TRIGGER_THRESHOLD: XINPUT_GAMEPAD_BUTTON_FLAGS = XINPUT_GAMEPAD_BUTTON_FLAGS(30u16);
impl ::core::marker::Copy for XINPUT_GAMEPAD_BUTTON_FLAGS {}
impl ::core::clone::Clone for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_GAMEPAD_BUTTON_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XINPUT_KEYSTROKE_FLAGS(pub u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_KEYSTROKE_KEYDOWN: XINPUT_KEYSTROKE_FLAGS = XINPUT_KEYSTROKE_FLAGS(1u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_KEYSTROKE_KEYUP: XINPUT_KEYSTROKE_FLAGS = XINPUT_KEYSTROKE_FLAGS(2u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const XINPUT_KEYSTROKE_REPEAT: XINPUT_KEYSTROKE_FLAGS = XINPUT_KEYSTROKE_FLAGS(4u16);
impl ::core::marker::Copy for XINPUT_KEYSTROKE_FLAGS {}
impl ::core::clone::Clone for XINPUT_KEYSTROKE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XINPUT_KEYSTROKE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XINPUT_KEYSTROKE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XINPUT_KEYSTROKE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_KEYSTROKE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XINPUT_KEYSTROKE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XINPUT_KEYSTROKE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XINPUT_VIRTUAL_KEY(pub u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_A: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22528u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_B: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22529u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_X: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22530u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_Y: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22531u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RSHOULDER: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22532u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LSHOULDER: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22533u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTRIGGER: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22534u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTRIGGER: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22535u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_DPAD_UP: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22544u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_DPAD_DOWN: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22545u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_DPAD_LEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22546u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_DPAD_RIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22547u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_START: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22548u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_BACK: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22549u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_PRESS: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22550u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_PRESS: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22551u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_UP: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22560u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_DOWN: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22561u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_RIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22562u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_LEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22563u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_UPLEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22564u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_UPRIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22565u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_DOWNRIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22566u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_LTHUMB_DOWNLEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22567u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_UP: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22576u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_DOWN: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22577u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_RIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22578u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_LEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22579u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_UPLEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22580u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_UPRIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22581u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_DOWNRIGHT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22582u16);
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub const VK_PAD_RTHUMB_DOWNLEFT: XINPUT_VIRTUAL_KEY = XINPUT_VIRTUAL_KEY(22583u16);
impl ::core::marker::Copy for XINPUT_VIRTUAL_KEY {}
impl ::core::clone::Clone for XINPUT_VIRTUAL_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XINPUT_VIRTUAL_KEY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XINPUT_VIRTUAL_KEY {
    type Abi = Self;
}
impl ::core::fmt::Debug for XINPUT_VIRTUAL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_VIRTUAL_KEY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub struct XINPUT_BATTERY_INFORMATION {
    pub BatteryType: BATTERY_TYPE,
    pub BatteryLevel: BATTERY_LEVEL,
}
impl ::core::marker::Copy for XINPUT_BATTERY_INFORMATION {}
impl ::core::clone::Clone for XINPUT_BATTERY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XINPUT_BATTERY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_BATTERY_INFORMATION").field("BatteryType", &self.BatteryType).field("BatteryLevel", &self.BatteryLevel).finish()
    }
}
unsafe impl ::windows::core::Abi for XINPUT_BATTERY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XINPUT_BATTERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XINPUT_BATTERY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for XINPUT_BATTERY_INFORMATION {}
impl ::core::default::Default for XINPUT_BATTERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub struct XINPUT_CAPABILITIES {
    pub Type: XINPUT_DEVTYPE,
    pub SubType: XINPUT_DEVSUBTYPE,
    pub Flags: XINPUT_CAPABILITIES_FLAGS,
    pub Gamepad: XINPUT_GAMEPAD,
    pub Vibration: XINPUT_VIBRATION,
}
impl ::core::marker::Copy for XINPUT_CAPABILITIES {}
impl ::core::clone::Clone for XINPUT_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XINPUT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_CAPABILITIES").field("Type", &self.Type).field("SubType", &self.SubType).field("Flags", &self.Flags).field("Gamepad", &self.Gamepad).field("Vibration", &self.Vibration).finish()
    }
}
unsafe impl ::windows::core::Abi for XINPUT_CAPABILITIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XINPUT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XINPUT_CAPABILITIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for XINPUT_CAPABILITIES {}
impl ::core::default::Default for XINPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub struct XINPUT_GAMEPAD {
    pub wButtons: XINPUT_GAMEPAD_BUTTON_FLAGS,
    pub bLeftTrigger: u8,
    pub bRightTrigger: u8,
    pub sThumbLX: i16,
    pub sThumbLY: i16,
    pub sThumbRX: i16,
    pub sThumbRY: i16,
}
impl ::core::marker::Copy for XINPUT_GAMEPAD {}
impl ::core::clone::Clone for XINPUT_GAMEPAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XINPUT_GAMEPAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_GAMEPAD").field("wButtons", &self.wButtons).field("bLeftTrigger", &self.bLeftTrigger).field("bRightTrigger", &self.bRightTrigger).field("sThumbLX", &self.sThumbLX).field("sThumbLY", &self.sThumbLY).field("sThumbRX", &self.sThumbRX).field("sThumbRY", &self.sThumbRY).finish()
    }
}
unsafe impl ::windows::core::Abi for XINPUT_GAMEPAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XINPUT_GAMEPAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XINPUT_GAMEPAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for XINPUT_GAMEPAD {}
impl ::core::default::Default for XINPUT_GAMEPAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub struct XINPUT_KEYSTROKE {
    pub VirtualKey: XINPUT_VIRTUAL_KEY,
    pub Unicode: u16,
    pub Flags: XINPUT_KEYSTROKE_FLAGS,
    pub UserIndex: u8,
    pub HidCode: u8,
}
impl ::core::marker::Copy for XINPUT_KEYSTROKE {}
impl ::core::clone::Clone for XINPUT_KEYSTROKE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XINPUT_KEYSTROKE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_KEYSTROKE").field("VirtualKey", &self.VirtualKey).field("Unicode", &self.Unicode).field("Flags", &self.Flags).field("UserIndex", &self.UserIndex).field("HidCode", &self.HidCode).finish()
    }
}
unsafe impl ::windows::core::Abi for XINPUT_KEYSTROKE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XINPUT_KEYSTROKE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XINPUT_KEYSTROKE>()) == 0 }
    }
}
impl ::core::cmp::Eq for XINPUT_KEYSTROKE {}
impl ::core::default::Default for XINPUT_KEYSTROKE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub struct XINPUT_STATE {
    pub dwPacketNumber: u32,
    pub Gamepad: XINPUT_GAMEPAD,
}
impl ::core::marker::Copy for XINPUT_STATE {}
impl ::core::clone::Clone for XINPUT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XINPUT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_STATE").field("dwPacketNumber", &self.dwPacketNumber).field("Gamepad", &self.Gamepad).finish()
    }
}
unsafe impl ::windows::core::Abi for XINPUT_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XINPUT_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XINPUT_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for XINPUT_STATE {}
impl ::core::default::Default for XINPUT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_XboxController\"`*"]
pub struct XINPUT_VIBRATION {
    pub wLeftMotorSpeed: u16,
    pub wRightMotorSpeed: u16,
}
impl ::core::marker::Copy for XINPUT_VIBRATION {}
impl ::core::clone::Clone for XINPUT_VIBRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XINPUT_VIBRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_VIBRATION").field("wLeftMotorSpeed", &self.wLeftMotorSpeed).field("wRightMotorSpeed", &self.wRightMotorSpeed).finish()
    }
}
unsafe impl ::windows::core::Abi for XINPUT_VIBRATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XINPUT_VIBRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XINPUT_VIBRATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for XINPUT_VIBRATION {}
impl ::core::default::Default for XINPUT_VIBRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
