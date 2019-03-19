// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::Quark;
use glib::StaticType;
use glib::Type;
use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_sys;
use gst_player_sys;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum PlayerColorBalanceType {
    Hue,
    Brightness,
    Saturation,
    Contrast,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PlayerColorBalanceType {
    type GlibType = gst_player_sys::GstPlayerColorBalanceType;

    fn to_glib(&self) -> gst_player_sys::GstPlayerColorBalanceType {
        match *self {
            PlayerColorBalanceType::Hue => gst_player_sys::GST_PLAYER_COLOR_BALANCE_HUE,
            PlayerColorBalanceType::Brightness => gst_player_sys::GST_PLAYER_COLOR_BALANCE_BRIGHTNESS,
            PlayerColorBalanceType::Saturation => gst_player_sys::GST_PLAYER_COLOR_BALANCE_SATURATION,
            PlayerColorBalanceType::Contrast => gst_player_sys::GST_PLAYER_COLOR_BALANCE_CONTRAST,
            PlayerColorBalanceType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_player_sys::GstPlayerColorBalanceType> for PlayerColorBalanceType {
    fn from_glib(value: gst_player_sys::GstPlayerColorBalanceType) -> Self {
        skip_assert_initialized!();
        match value {
            3 => PlayerColorBalanceType::Hue,
            0 => PlayerColorBalanceType::Brightness,
            2 => PlayerColorBalanceType::Saturation,
            1 => PlayerColorBalanceType::Contrast,
            value => PlayerColorBalanceType::__Unknown(value),
        }
    }
}

impl StaticType for PlayerColorBalanceType {
    fn static_type() -> Type {
        unsafe { from_glib(gst_player_sys::gst_player_color_balance_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PlayerColorBalanceType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PlayerColorBalanceType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PlayerColorBalanceType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum PlayerError {
    Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PlayerError {
    type GlibType = gst_player_sys::GstPlayerError;

    fn to_glib(&self) -> gst_player_sys::GstPlayerError {
        match *self {
            PlayerError::Failed => gst_player_sys::GST_PLAYER_ERROR_FAILED,
            PlayerError::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_player_sys::GstPlayerError> for PlayerError {
    fn from_glib(value: gst_player_sys::GstPlayerError) -> Self {
        skip_assert_initialized!();
        match value {
            0 => PlayerError::Failed,
            value => PlayerError::__Unknown(value),
        }
    }
}

impl ErrorDomain for PlayerError {
    fn domain() -> Quark {
        skip_assert_initialized!();
        unsafe { from_glib(gst_player_sys::gst_player_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(PlayerError::Failed),
            _ => Some(PlayerError::Failed),
        }
    }
}

impl StaticType for PlayerError {
    fn static_type() -> Type {
        unsafe { from_glib(gst_player_sys::gst_player_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PlayerError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PlayerError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PlayerError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum PlayerSnapshotFormat {
    RawNative,
    RawXrgb,
    RawBgrx,
    Jpg,
    Png,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PlayerSnapshotFormat {
    type GlibType = gst_player_sys::GstPlayerSnapshotFormat;

    fn to_glib(&self) -> gst_player_sys::GstPlayerSnapshotFormat {
        match *self {
            PlayerSnapshotFormat::RawNative => gst_player_sys::GST_PLAYER_THUMBNAIL_RAW_NATIVE,
            PlayerSnapshotFormat::RawXrgb => gst_player_sys::GST_PLAYER_THUMBNAIL_RAW_xRGB,
            PlayerSnapshotFormat::RawBgrx => gst_player_sys::GST_PLAYER_THUMBNAIL_RAW_BGRx,
            PlayerSnapshotFormat::Jpg => gst_player_sys::GST_PLAYER_THUMBNAIL_JPG,
            PlayerSnapshotFormat::Png => gst_player_sys::GST_PLAYER_THUMBNAIL_PNG,
            PlayerSnapshotFormat::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_player_sys::GstPlayerSnapshotFormat> for PlayerSnapshotFormat {
    fn from_glib(value: gst_player_sys::GstPlayerSnapshotFormat) -> Self {
        skip_assert_initialized!();
        match value {
            0 => PlayerSnapshotFormat::RawNative,
            1 => PlayerSnapshotFormat::RawXrgb,
            2 => PlayerSnapshotFormat::RawBgrx,
            3 => PlayerSnapshotFormat::Jpg,
            4 => PlayerSnapshotFormat::Png,
            value => PlayerSnapshotFormat::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum PlayerState {
    Stopped,
    Buffering,
    Paused,
    Playing,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PlayerState {
    type GlibType = gst_player_sys::GstPlayerState;

    fn to_glib(&self) -> gst_player_sys::GstPlayerState {
        match *self {
            PlayerState::Stopped => gst_player_sys::GST_PLAYER_STATE_STOPPED,
            PlayerState::Buffering => gst_player_sys::GST_PLAYER_STATE_BUFFERING,
            PlayerState::Paused => gst_player_sys::GST_PLAYER_STATE_PAUSED,
            PlayerState::Playing => gst_player_sys::GST_PLAYER_STATE_PLAYING,
            PlayerState::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_player_sys::GstPlayerState> for PlayerState {
    fn from_glib(value: gst_player_sys::GstPlayerState) -> Self {
        skip_assert_initialized!();
        match value {
            0 => PlayerState::Stopped,
            1 => PlayerState::Buffering,
            2 => PlayerState::Paused,
            3 => PlayerState::Playing,
            value => PlayerState::__Unknown(value),
        }
    }
}

impl StaticType for PlayerState {
    fn static_type() -> Type {
        unsafe { from_glib(gst_player_sys::gst_player_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PlayerState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PlayerState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PlayerState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

