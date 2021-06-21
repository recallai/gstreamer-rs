// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Asset;
use crate::BaseEffect;
use crate::Container;
use crate::Extractable;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::FrameNumber;
use crate::Layer;
use crate::TimelineElement;
use crate::Track;
use crate::TrackElement;
use crate::TrackType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GESClip")]
    pub struct Clip(Object<ffi::GESClip, ffi::GESClipClass>) @extends Container, TimelineElement, @implements Extractable;

    match fn {
        type_ => || ffi::ges_clip_get_type(),
    }
}

pub const NONE_CLIP: Option<&Clip> = None;

pub trait ClipExt: 'static {
    #[doc(alias = "ges_clip_add_asset")]
    fn add_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<TrackElement, glib::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_add_child_to_track")]
    fn add_child_to_track<P: IsA<TrackElement>, Q: IsA<Track>>(
        &self,
        child: &P,
        track: &Q,
    ) -> Result<TrackElement, glib::Error>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_add_top_effect")]
    fn add_top_effect<P: IsA<BaseEffect>>(&self, effect: &P, index: i32)
        -> Result<(), glib::Error>;

    #[doc(alias = "ges_clip_find_track_element")]
    fn find_track_element<P: IsA<Track>>(
        &self,
        track: Option<&P>,
        type_: glib::types::Type,
    ) -> Option<TrackElement>;

    #[doc(alias = "ges_clip_find_track_elements")]
    fn find_track_elements<P: IsA<Track>>(
        &self,
        track: Option<&P>,
        track_type: TrackType,
        type_: glib::types::Type,
    ) -> Vec<TrackElement>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_get_duration_limit")]
    #[doc(alias = "get_duration_limit")]
    fn duration_limit(&self) -> gst::ClockTime;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_get_internal_time_from_timeline_time")]
    #[doc(alias = "get_internal_time_from_timeline_time")]
    fn internal_time_from_timeline_time<P: IsA<TrackElement>>(
        &self,
        child: &P,
        timeline_time: impl Into<Option<gst::ClockTime>>,
    ) -> Result<Option<gst::ClockTime>, glib::Error>;

    #[doc(alias = "ges_clip_get_layer")]
    #[doc(alias = "get_layer")]
    fn layer(&self) -> Option<Layer>;

    #[doc(alias = "ges_clip_get_supported_formats")]
    #[doc(alias = "get_supported_formats")]
    fn supported_formats(&self) -> TrackType;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_get_timeline_time_from_internal_time")]
    #[doc(alias = "get_timeline_time_from_internal_time")]
    fn timeline_time_from_internal_time<P: IsA<TrackElement>>(
        &self,
        child: &P,
        internal_time: impl Into<Option<gst::ClockTime>>,
    ) -> Result<Option<gst::ClockTime>, glib::Error>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_get_timeline_time_from_source_frame")]
    #[doc(alias = "get_timeline_time_from_source_frame")]
    fn timeline_time_from_source_frame(
        &self,
        frame_number: FrameNumber,
    ) -> Result<Option<gst::ClockTime>, glib::Error>;

    #[doc(alias = "ges_clip_get_top_effect_index")]
    #[doc(alias = "get_top_effect_index")]
    fn top_effect_index<P: IsA<BaseEffect>>(&self, effect: &P) -> i32;

    #[doc(alias = "ges_clip_get_top_effect_position")]
    #[doc(alias = "get_top_effect_position")]
    fn top_effect_position<P: IsA<BaseEffect>>(&self, effect: &P) -> i32;

    #[doc(alias = "ges_clip_get_top_effects")]
    #[doc(alias = "get_top_effects")]
    fn top_effects(&self) -> Vec<TrackElement>;

    #[doc(alias = "ges_clip_move_to_layer")]
    fn move_to_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_move_to_layer_full")]
    fn move_to_layer_full<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_remove_top_effect")]
    fn remove_top_effect<P: IsA<BaseEffect>>(&self, effect: &P) -> Result<(), glib::Error>;

    #[doc(alias = "ges_clip_set_supported_formats")]
    fn set_supported_formats(&self, supportedformats: TrackType);

    #[doc(alias = "ges_clip_set_top_effect_index")]
    fn set_top_effect_index<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newindex: u32,
    ) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_set_top_effect_index_full")]
    fn set_top_effect_index_full<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newindex: u32,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "ges_clip_set_top_effect_priority")]
    fn set_top_effect_priority<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newpriority: u32,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_clip_split")]
    fn split(&self, position: u64) -> Result<Clip, glib::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_split_full")]
    fn split_full(&self, position: u64) -> Result<Option<Clip>, glib::Error>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "duration-limit")]
    fn connect_duration_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "layer")]
    fn connect_layer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "supported-formats")]
    fn connect_supported_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Clip>> ClipExt for O {
    fn add_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<TrackElement, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_clip_add_asset(
                self.as_ref().to_glib_none().0,
                asset.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to add asset"))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn add_child_to_track<P: IsA<TrackElement>, Q: IsA<Track>>(
        &self,
        child: &P,
        track: &Q,
    ) -> Result<TrackElement, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_clip_add_child_to_track(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                track.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn add_top_effect<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        index: i32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_clip_add_top_effect(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
                index,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn find_track_element<P: IsA<Track>>(
        &self,
        track: Option<&P>,
        type_: glib::types::Type,
    ) -> Option<TrackElement> {
        unsafe {
            from_glib_full(ffi::ges_clip_find_track_element(
                self.as_ref().to_glib_none().0,
                track.map(|p| p.as_ref()).to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    fn find_track_elements<P: IsA<Track>>(
        &self,
        track: Option<&P>,
        track_type: TrackType,
        type_: glib::types::Type,
    ) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_clip_find_track_elements(
                self.as_ref().to_glib_none().0,
                track.map(|p| p.as_ref()).to_glib_none().0,
                track_type.into_glib(),
                type_.into_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn duration_limit(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::ges_clip_get_duration_limit(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn internal_time_from_timeline_time<P: IsA<TrackElement>>(
        &self,
        child: &P,
        timeline_time: impl Into<Option<gst::ClockTime>>,
    ) -> Result<Option<gst::ClockTime>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_clip_get_internal_time_from_timeline_time(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                timeline_time.into().into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn layer(&self) -> Option<Layer> {
        unsafe { from_glib_full(ffi::ges_clip_get_layer(self.as_ref().to_glib_none().0)) }
    }

    fn supported_formats(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_clip_get_supported_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn timeline_time_from_internal_time<P: IsA<TrackElement>>(
        &self,
        child: &P,
        internal_time: impl Into<Option<gst::ClockTime>>,
    ) -> Result<Option<gst::ClockTime>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_clip_get_timeline_time_from_internal_time(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                internal_time.into().into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn timeline_time_from_source_frame(
        &self,
        frame_number: FrameNumber,
    ) -> Result<Option<gst::ClockTime>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_clip_get_timeline_time_from_source_frame(
                self.as_ref().to_glib_none().0,
                frame_number,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn top_effect_index<P: IsA<BaseEffect>>(&self, effect: &P) -> i32 {
        unsafe {
            ffi::ges_clip_get_top_effect_index(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
            )
        }
    }

    fn top_effect_position<P: IsA<BaseEffect>>(&self, effect: &P) -> i32 {
        unsafe {
            ffi::ges_clip_get_top_effect_position(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
            )
        }
    }

    fn top_effects(&self) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_clip_get_top_effects(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn move_to_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_clip_move_to_layer(
                    self.as_ref().to_glib_none().0,
                    layer.as_ref().to_glib_none().0
                ),
                "Failed to move clip to specified layer"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn move_to_layer_full<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_clip_move_to_layer_full(
                self.as_ref().to_glib_none().0,
                layer.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn remove_top_effect<P: IsA<BaseEffect>>(&self, effect: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_clip_remove_top_effect(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_supported_formats(&self, supportedformats: TrackType) {
        unsafe {
            ffi::ges_clip_set_supported_formats(
                self.as_ref().to_glib_none().0,
                supportedformats.into_glib(),
            );
        }
    }

    fn set_top_effect_index<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newindex: u32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_clip_set_top_effect_index(
                    self.as_ref().to_glib_none().0,
                    effect.as_ref().to_glib_none().0,
                    newindex
                ),
                "Failed to move effect"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_top_effect_index_full<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newindex: u32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_clip_set_top_effect_index_full(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
                newindex,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_top_effect_priority<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newpriority: u32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_clip_set_top_effect_priority(
                    self.as_ref().to_glib_none().0,
                    effect.as_ref().to_glib_none().0,
                    newpriority
                ),
                "Failed to the set top effect priority"
            )
        }
    }

    fn split(&self, position: u64) -> Result<Clip, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_clip_split(
                self.as_ref().to_glib_none().0,
                position,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to split clip"))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn split_full(&self, position: u64) -> Result<Option<Clip>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::ges_clip_split_full(self.as_ref().to_glib_none().0, position, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_duration_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_limit_trampoline<P: IsA<Clip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration-limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_limit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_layer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layer_trampoline<P: IsA<Clip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::layer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_layer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_supported_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_supported_formats_trampoline<
            P: IsA<Clip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::supported-formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_supported_formats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
