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
use std::ffi::CStr;
use std::fmt;

bitflags! {
    #[doc(alias = "GESPipelineFlags")]
    pub struct PipelineFlags: u32 {
        #[doc(alias = "GES_PIPELINE_MODE_PREVIEW_AUDIO")]
        const AUDIO_PREVIEW = ffi::GES_PIPELINE_MODE_PREVIEW_AUDIO as u32;
        #[doc(alias = "GES_PIPELINE_MODE_PREVIEW_VIDEO")]
        const VIDEO_PREVIEW = ffi::GES_PIPELINE_MODE_PREVIEW_VIDEO as u32;
        #[doc(alias = "GES_PIPELINE_MODE_PREVIEW")]
        const FULL_PREVIEW = ffi::GES_PIPELINE_MODE_PREVIEW as u32;
        #[doc(alias = "GES_PIPELINE_MODE_RENDER")]
        const RENDER = ffi::GES_PIPELINE_MODE_RENDER as u32;
        #[doc(alias = "GES_PIPELINE_MODE_SMART_RENDER")]
        const SMART_RENDER = ffi::GES_PIPELINE_MODE_SMART_RENDER as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for PipelineFlags {
    type GlibType = ffi::GESPipelineFlags;

    fn into_glib(self) -> ffi::GESPipelineFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESPipelineFlags> for PipelineFlags {
    unsafe fn from_glib(value: ffi::GESPipelineFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PipelineFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ges_pipeline_flags_get_type()) }
    }
}

impl glib::value::ValueType for PipelineFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PipelineFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PipelineFlags {
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

bitflags! {
    #[doc(alias = "GESTrackType")]
    pub struct TrackType: u32 {
        #[doc(alias = "GES_TRACK_TYPE_UNKNOWN")]
        const UNKNOWN = ffi::GES_TRACK_TYPE_UNKNOWN as u32;
        #[doc(alias = "GES_TRACK_TYPE_AUDIO")]
        const AUDIO = ffi::GES_TRACK_TYPE_AUDIO as u32;
        #[doc(alias = "GES_TRACK_TYPE_VIDEO")]
        const VIDEO = ffi::GES_TRACK_TYPE_VIDEO as u32;
        #[doc(alias = "GES_TRACK_TYPE_TEXT")]
        const TEXT = ffi::GES_TRACK_TYPE_TEXT as u32;
        #[doc(alias = "GES_TRACK_TYPE_CUSTOM")]
        const CUSTOM = ffi::GES_TRACK_TYPE_CUSTOM as u32;
    }
}

impl TrackType {
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::ges_track_type_name(self.into_glib())
                    .as_ref()
                    .expect("ges_track_type_name returned NULL"),
            )
            .to_str()
            .expect("ges_track_type_name returned an invalid string")
        }
    }
}

impl fmt::Display for TrackType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for TrackType {
    type GlibType = ffi::GESTrackType;

    fn into_glib(self) -> ffi::GESTrackType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESTrackType> for TrackType {
    unsafe fn from_glib(value: ffi::GESTrackType) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for TrackType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ges_track_type_get_type()) }
    }
}

impl glib::value::ValueType for TrackType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TrackType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TrackType {
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
