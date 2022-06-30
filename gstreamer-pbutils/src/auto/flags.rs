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
    #[doc(alias = "GstDiscovererSerializeFlags")]
    pub struct DiscovererSerializeFlags: u32 {
        #[doc(alias = "GST_DISCOVERER_SERIALIZE_CAPS")]
        const CAPS = ffi::GST_DISCOVERER_SERIALIZE_CAPS as u32;
        #[doc(alias = "GST_DISCOVERER_SERIALIZE_TAGS")]
        const TAGS = ffi::GST_DISCOVERER_SERIALIZE_TAGS as u32;
        #[doc(alias = "GST_DISCOVERER_SERIALIZE_MISC")]
        const MISC = ffi::GST_DISCOVERER_SERIALIZE_MISC as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for DiscovererSerializeFlags {
    type GlibType = ffi::GstDiscovererSerializeFlags;

    fn into_glib(self) -> ffi::GstDiscovererSerializeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstDiscovererSerializeFlags> for DiscovererSerializeFlags {
    unsafe fn from_glib(value: ffi::GstDiscovererSerializeFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for DiscovererSerializeFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_discoverer_serialize_flags_get_type()) }
    }
}

impl glib::value::ValueType for DiscovererSerializeFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DiscovererSerializeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DiscovererSerializeFlags {
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

#[cfg(any(feature = "v1_20", feature = "dox"))]
bitflags! {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "GstPbUtilsCapsDescriptionFlags")]
    pub struct PbUtilsCapsDescriptionFlags: u32 {
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_CONTAINER")]
        const CONTAINER = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_CONTAINER as u32;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_AUDIO")]
        const AUDIO = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_AUDIO as u32;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_VIDEO")]
        const VIDEO = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_VIDEO as u32;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_IMAGE")]
        const IMAGE = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_IMAGE as u32;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_SUBTITLE")]
        const SUBTITLE = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_SUBTITLE as u32;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_TAG")]
        const TAG = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_TAG as u32;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_GENERIC")]
        const GENERIC = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_GENERIC as u32;
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_METADATA")]
        const METADATA = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_METADATA as u32;
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for PbUtilsCapsDescriptionFlags {
    type GlibType = ffi::GstPbUtilsCapsDescriptionFlags;

    fn into_glib(self) -> ffi::GstPbUtilsCapsDescriptionFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstPbUtilsCapsDescriptionFlags> for PbUtilsCapsDescriptionFlags {
    unsafe fn from_glib(value: ffi::GstPbUtilsCapsDescriptionFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl StaticType for PbUtilsCapsDescriptionFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_pb_utils_caps_description_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for PbUtilsCapsDescriptionFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl<'a> FromValue<'a> for PbUtilsCapsDescriptionFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl ToValue for PbUtilsCapsDescriptionFlags {
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
