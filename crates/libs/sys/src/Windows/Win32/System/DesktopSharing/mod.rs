pub type IRDPSRAPIApplication = *mut ::core::ffi::c_void;
pub type IRDPSRAPIApplicationFilter = *mut ::core::ffi::c_void;
pub type IRDPSRAPIApplicationList = *mut ::core::ffi::c_void;
pub type IRDPSRAPIAttendee = *mut ::core::ffi::c_void;
pub type IRDPSRAPIAttendeeDisconnectInfo = *mut ::core::ffi::c_void;
pub type IRDPSRAPIAttendeeManager = *mut ::core::ffi::c_void;
pub type IRDPSRAPIAudioStream = *mut ::core::ffi::c_void;
pub type IRDPSRAPIClipboardUseEvents = *mut ::core::ffi::c_void;
pub type IRDPSRAPIDebug = *mut ::core::ffi::c_void;
pub type IRDPSRAPIFrameBuffer = *mut ::core::ffi::c_void;
pub type IRDPSRAPIInvitation = *mut ::core::ffi::c_void;
pub type IRDPSRAPIInvitationManager = *mut ::core::ffi::c_void;
pub type IRDPSRAPIPerfCounterLogger = *mut ::core::ffi::c_void;
pub type IRDPSRAPIPerfCounterLoggingManager = *mut ::core::ffi::c_void;
pub type IRDPSRAPISessionProperties = *mut ::core::ffi::c_void;
pub type IRDPSRAPISharingSession = *mut ::core::ffi::c_void;
pub type IRDPSRAPISharingSession2 = *mut ::core::ffi::c_void;
pub type IRDPSRAPITcpConnectionInfo = *mut ::core::ffi::c_void;
pub type IRDPSRAPITransportStream = *mut ::core::ffi::c_void;
pub type IRDPSRAPITransportStreamBuffer = *mut ::core::ffi::c_void;
pub type IRDPSRAPITransportStreamEvents = *mut ::core::ffi::c_void;
pub type IRDPSRAPIViewer = *mut ::core::ffi::c_void;
pub type IRDPSRAPIVirtualChannel = *mut ::core::ffi::c_void;
pub type IRDPSRAPIVirtualChannelManager = *mut ::core::ffi::c_void;
pub type IRDPSRAPIWindow = *mut ::core::ffi::c_void;
pub type IRDPSRAPIWindowList = *mut ::core::ffi::c_void;
pub type IRDPViewerInputSink = *mut ::core::ffi::c_void;
pub type _IRDPSessionEvents = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPAPI_EVENT_ON_BOUNDING_RECT_CHANGED: u32 = 340u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPFILTER_UPDATE: u32 = 322u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_CLOSE: u32 = 317u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_OPEN: u32 = 316u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_UPDATE: u32 = 318u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_CONNECTED: u32 = 301u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_DISCONNECTED: u32 = 302u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_UPDATE: u32 = 303u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_REQUEST: u32 = 309u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_RESPONSE: u32 = 338u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ERROR: u32 = 304u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_FOCUSRELEASED: u32 = 324u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_PAUSED: u32 = 310u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_RESUMED: u32 = 311u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_DESKTOP_SETTINGS_CHANGED: u32 = 325u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_RECT_CHANGED: u32 = 323u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_CLOSED: u32 = 634u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_DATARECEIVED: u32 = 633u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_SENDCOMPLETED: u32 = 632u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_AUTHENTICATED: u32 = 307u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTED: u32 = 305u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTFAILED: u32 = 308u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_DISCONNECTED: u32 = 306u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_DATARECEIVED: u32 = 314u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_JOIN: u32 = 312u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_LEAVE: u32 = 313u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_SENDCOMPLETED: u32 = 315u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_CLOSE: u32 = 320u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_OPEN: u32 = 319u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_UPDATE: u32 = 321u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_BUTTON_RECEIVED: u32 = 700u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_MOVE_RECEIVED: u32 = 701u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_WHEEL_RECEIVED: u32 = 702u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_ADD_TOUCH_INPUT: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_BEGIN_TOUCH_FRAME: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CLOSE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CONNECTTOCLIENT: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CONNECTUSINGTRANSPORTSTREAM: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CREATE_INVITATION: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_END_TOUCH_FRAME: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_GETFRAMEBUFFERBITS: u32 = 149u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_GETSHAREDRECT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_OPEN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_PAUSE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_REQUEST_COLOR_DEPTH_CHANGE: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_REQUEST_CONTROL: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_RESUME: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SENDCONTROLLEVELCHANGERESPONSE: u32 = 148u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_KEYBOARD_EVENT: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_BUTTON_EVENT: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_MOVE_EVENT: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_WHEEL_EVENT: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_SYNC_EVENT: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SETSHAREDRECT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SET_RENDERING_SURFACE: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SHOW_WINDOW: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STARTREVCONNECTLISTENER: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMCLOSE: u32 = 426u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMOPEN: u32 = 425u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMREADDATA: u32 = 424u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMSENDDATA: u32 = 423u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAM_ALLOCBUFFER: u32 = 421u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAM_FREEBUFFER: u32 = 422u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_TERMINATE_CONNECTION: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIEWERCONNECT: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIEWERDISCONNECT: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_CREATE: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SEND_DATA: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SET_ACCESS: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFILTERENABLED: u32 = 219u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFILTER_ENABLED: u32 = 218u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFLAGS: u32 = 223u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION: u32 = 211u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION_FILTER: u32 = 215u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION_LIST: u32 = 217u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPNAME: u32 = 214u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEELIMIT: u32 = 235u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEES: u32 = 203u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEE_FLAGS: u32 = 230u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CHANNELMANAGER: u32 = 206u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CODE: u32 = 241u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CONINFO: u32 = 231u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CONNECTION_STRING: u32 = 232u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_COUNT: u32 = 244u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CTRL_LEVEL: u32 = 242u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DBG_CLX_CMDLINE: u32 = 222u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DISCONNECTED_STRING: u32 = 237u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DISPIDVALUE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER: u32 = 254u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_BPP: u32 = 253u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_HEIGHT: u32 = 251u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_WIDTH: u32 = 252u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_GROUP_NAME: u32 = 233u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ID: u32 = 201u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATION: u32 = 205u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATIONITEM: u32 = 221u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATIONS: u32 = 204u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_LOCAL_IP: u32 = 227u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_LOCAL_PORT: u32 = 226u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PASSWORD: u32 = 234u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PEER_IP: u32 = 229u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PEER_PORT: u32 = 228u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PROTOCOL_TYPE: u32 = 225u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REASON: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REMOTENAME: u32 = 243u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REVOKED: u32 = 236u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SESSION_COLORDEPTH: u32 = 239u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SESSION_PROPERTIES: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SHARED: u32 = 220u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_CONTEXT: u32 = 560u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_FLAGS: u32 = 561u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADOFFSET: u32 = 559u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADSIZE: u32 = 558u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORAGE: u32 = 555u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORESIZE: u32 = 562u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_USESMARTSIZING: u32 = 238u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETFLAGS: u32 = 208u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETNAME: u32 = 207u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETPRIORITY: u32 = 209u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWID: u32 = 210u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWNAME: u32 = 213u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWSHARED: u32 = 212u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOW_LIST: u32 = 216u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WNDFLAGS: u32 = 224u32;
pub const RDPSRAPIApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc116a484_4b25_4b9f_8a54_b934b06e57fa);
pub const RDPSRAPIApplicationFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe35ace89_c7e8_427e_a4f9_b9da072826bd);
pub const RDPSRAPIApplicationList: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9e31c815_7433_4876_97fb_ed59fe2baa22);
pub const RDPSRAPIAttendee: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x74f93bb5_755f_488e_8a29_2390108aef55);
pub const RDPSRAPIAttendeeDisconnectInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb47d7250_5bdb_405d_b487_caad9c56f4f8);
pub const RDPSRAPIAttendeeManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd7b13a01_f7d4_42a6_8595_12fc8c24e851);
pub const RDPSRAPIFrameBuffer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa4f66bcc_538e_4101_951d_30847adb5101);
pub const RDPSRAPIInvitation: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x49174dc6_0731_4b5e_8ee1_83a63d3868fa);
pub const RDPSRAPIInvitationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x53d9c9db_75ab_4271_948a_4c4eb36a8f2b);
pub const RDPSRAPISessionProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdd7594ff_ea2a_4c06_8fdf_132de48b6510);
pub const RDPSRAPITcpConnectionInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbe49db3f_ebb6_4278_8ce0_d5455833eaee);
pub const RDPSRAPIWindow: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x03cf46db_ce45_4d36_86ed_ed28b74398bf);
pub const RDPSRAPIWindowList: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c21e2b8_5dd4_42cc_81ba_1c099852e6fa);
pub const RDPSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9b78f0e6_3e05_4a5b_b2e8_e743a8956b65);
pub const RDPTransportStreamBuffer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8d4a1c69_f17f_4549_a699_761c6e6b5c0a);
pub const RDPTransportStreamEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31e3ab20_5350_483f_9dc6_6748665efdeb);
pub const RDPViewer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32be5ed2_5c86_480f_a914_0ff8885a1b3f);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type ATTENDEE_DISCONNECT_REASON = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_MIN: ATTENDEE_DISCONNECT_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_APP: ATTENDEE_DISCONNECT_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_ERR: ATTENDEE_DISCONNECT_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_CLI: ATTENDEE_DISCONNECT_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_MAX: ATTENDEE_DISCONNECT_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type CHANNEL_ACCESS_ENUM = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_ACCESS_ENUM_NONE: CHANNEL_ACCESS_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_ACCESS_ENUM_SENDRECEIVE: CHANNEL_ACCESS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type CHANNEL_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_LEGACY: CHANNEL_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_UNCOMPRESSED: CHANNEL_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_DYNAMIC: CHANNEL_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type CHANNEL_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_LO: CHANNEL_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_MED: CHANNEL_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_HI: CHANNEL_PRIORITY = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type CTRL_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_MIN: CTRL_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_INVALID: CTRL_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_NONE: CTRL_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_VIEW: CTRL_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_INTERACTIVE: CTRL_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_REQCTRL_VIEW: CTRL_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_REQCTRL_INTERACTIVE: CTRL_LEVEL = 5i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_MAX: CTRL_LEVEL = 5i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPENCOMAPI_ATTENDEE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_FLAGS_LOCAL: RDPENCOMAPI_ATTENDEE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPENCOMAPI_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_CHANNEL_MESSAGE_SIZE: RDPENCOMAPI_CONSTANTS = 1024i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_CHANNEL_NAME_LEN: RDPENCOMAPI_CONSTANTS = 8i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_LEGACY_CHANNEL_MESSAGE_SIZE: RDPENCOMAPI_CONSTANTS = 409600i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_EVERYONE: RDPENCOMAPI_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_HOST: RDPENCOMAPI_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_CONN_INTERVAL: RDPENCOMAPI_CONSTANTS = 50i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_DEFAULT: RDPENCOMAPI_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_APP_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const APP_FLAG_PRIVILEGED: RDPSRAPI_APP_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_KBD_CODE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_CODE_SCANCODE: RDPSRAPI_KBD_CODE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_CODE_UNICODE: RDPSRAPI_KBD_CODE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_KBD_SYNC_FLAG = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_SCROLL_LOCK: RDPSRAPI_KBD_SYNC_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_NUM_LOCK: RDPSRAPI_KBD_SYNC_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_CAPS_LOCK: RDPSRAPI_KBD_SYNC_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_KANA_LOCK: RDPSRAPI_KBD_SYNC_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_MOUSE_BUTTON_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_WND_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const WND_FLAG_PRIVILEGED: RDPSRAPI_WND_FLAGS = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub struct __ReferenceRemainingTypes__ {
    pub __ctrlLevel__: CTRL_LEVEL,
    pub __attendeeDisconnectReason__: ATTENDEE_DISCONNECT_REASON,
    pub __channelPriority__: CHANNEL_PRIORITY,
    pub __channelFlags__: CHANNEL_FLAGS,
    pub __channelAccessEnum__: CHANNEL_ACCESS_ENUM,
    pub __rdpencomapiAttendeeFlags__: RDPENCOMAPI_ATTENDEE_FLAGS,
    pub __rdpsrapiWndFlags__: RDPSRAPI_WND_FLAGS,
    pub __rdpsrapiAppFlags__: RDPSRAPI_APP_FLAGS,
}
impl ::core::marker::Copy for __ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
