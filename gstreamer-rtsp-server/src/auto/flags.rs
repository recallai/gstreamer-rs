// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;

bitflags! {
    #[doc(alias = "GstRTSPAddressFlags")]
    pub struct RTSPAddressFlags: u32 {
        #[doc(alias = "GST_RTSP_ADDRESS_FLAG_IPV4")]
        const IPV4 = ffi::GST_RTSP_ADDRESS_FLAG_IPV4 as u32;
        #[doc(alias = "GST_RTSP_ADDRESS_FLAG_IPV6")]
        const IPV6 = ffi::GST_RTSP_ADDRESS_FLAG_IPV6 as u32;
        #[doc(alias = "GST_RTSP_ADDRESS_FLAG_EVEN_PORT")]
        const EVEN_PORT = ffi::GST_RTSP_ADDRESS_FLAG_EVEN_PORT as u32;
        #[doc(alias = "GST_RTSP_ADDRESS_FLAG_MULTICAST")]
        const MULTICAST = ffi::GST_RTSP_ADDRESS_FLAG_MULTICAST as u32;
        #[doc(alias = "GST_RTSP_ADDRESS_FLAG_UNICAST")]
        const UNICAST = ffi::GST_RTSP_ADDRESS_FLAG_UNICAST as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for RTSPAddressFlags {
    type GlibType = ffi::GstRTSPAddressFlags;

    fn into_glib(self) -> ffi::GstRTSPAddressFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPAddressFlags> for RTSPAddressFlags {
    unsafe fn from_glib(value: ffi::GstRTSPAddressFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    #[doc(alias = "GstRTSPTransportMode")]
    pub struct RTSPTransportMode: u32 {
        #[doc(alias = "GST_RTSP_TRANSPORT_MODE_PLAY")]
        const PLAY = ffi::GST_RTSP_TRANSPORT_MODE_PLAY as u32;
        #[doc(alias = "GST_RTSP_TRANSPORT_MODE_RECORD")]
        const RECORD = ffi::GST_RTSP_TRANSPORT_MODE_RECORD as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for RTSPTransportMode {
    type GlibType = ffi::GstRTSPTransportMode;

    fn into_glib(self) -> ffi::GstRTSPTransportMode {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPTransportMode> for RTSPTransportMode {
    unsafe fn from_glib(value: ffi::GstRTSPTransportMode) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTSPTransportMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_transport_mode_get_type()) }
    }
}

impl glib::value::ValueType for RTSPTransportMode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTSPTransportMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTSPTransportMode {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
