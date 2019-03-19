// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Timeline;
use TrackElement;
use TrackType;
use ges_sys;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gst;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Track(Object<ges_sys::GESTrack, ges_sys::GESTrackClass, TrackClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || ges_sys::ges_track_get_type(),
    }
}

impl Track {
    pub fn new(type_: TrackType, caps: &gst::Caps) -> Track {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ges_sys::ges_track_new(type_.to_glib(), caps.to_glib_full()))
        }
    }
}

pub const NONE_TRACK: Option<&Track> = None;

pub trait GESTrackExt: 'static {
    fn add_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError>;

    fn commit(&self) -> bool;

    fn get_caps(&self) -> Option<gst::Caps>;

    fn get_elements(&self) -> Vec<TrackElement>;

    fn get_mixing(&self) -> bool;

    fn get_timeline(&self) -> Option<Timeline>;

    fn remove_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError>;

    //fn set_create_element_for_gap_func<P: Fn(&Track) -> gst::Element + 'static>(&self, func: P);

    fn set_mixing(&self, mixing: bool);

    fn set_restriction_caps(&self, caps: &gst::Caps);

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P);

    fn update_restriction_caps(&self, caps: &gst::Caps);

    fn get_property_duration(&self) -> u64;

    fn get_property_restriction_caps(&self) -> Option<gst::Caps>;

    fn get_property_track_type(&self) -> TrackType;

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_track_element_added<F: Fn(&Self, &TrackElement) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_track_element_removed<F: Fn(&Self, &TrackElement) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mixing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_restriction_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Track>> GESTrackExt for O {
    fn add_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ges_sys::ges_track_add_element(self.as_ref().to_glib_none().0, object.as_ref().to_glib_none().0), "Failed to add element")
        }
    }

    fn commit(&self) -> bool {
        unsafe {
            from_glib(ges_sys::ges_track_commit(self.as_ref().to_glib_none().0))
        }
    }

    fn get_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_none(ges_sys::ges_track_get_caps(self.as_ref().to_glib_none().0))
        }
    }

    fn get_elements(&self) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ges_sys::ges_track_get_elements(self.as_ref().to_glib_none().0))
        }
    }

    fn get_mixing(&self) -> bool {
        unsafe {
            from_glib(ges_sys::ges_track_get_mixing(self.as_ref().to_glib_none().0))
        }
    }

    fn get_timeline(&self) -> Option<Timeline> {
        unsafe {
            from_glib_none(ges_sys::ges_track_get_timeline(self.as_ref().to_glib_none().0))
        }
    }

    fn remove_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ges_sys::ges_track_remove_element(self.as_ref().to_glib_none().0, object.as_ref().to_glib_none().0), "Failed to remove element")
        }
    }

    //fn set_create_element_for_gap_func<P: Fn(&Track) -> gst::Element + 'static>(&self, func: P) {
    //    unsafe { TODO: call ges_sys:ges_track_set_create_element_for_gap_func() }
    //}

    fn set_mixing(&self, mixing: bool) {
        unsafe {
            ges_sys::ges_track_set_mixing(self.as_ref().to_glib_none().0, mixing.to_glib());
        }
    }

    fn set_restriction_caps(&self, caps: &gst::Caps) {
        unsafe {
            ges_sys::ges_track_set_restriction_caps(self.as_ref().to_glib_none().0, caps.to_glib_none().0);
        }
    }

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) {
        unsafe {
            ges_sys::ges_track_set_timeline(self.as_ref().to_glib_none().0, timeline.as_ref().to_glib_none().0);
        }
    }

    fn update_restriction_caps(&self, caps: &gst::Caps) {
        unsafe {
            ges_sys::ges_track_update_restriction_caps(self.as_ref().to_glib_none().0, caps.to_glib_none().0);
        }
    }

    fn get_property_duration(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"duration\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_restriction_caps(&self) -> Option<gst::Caps> {
        unsafe {
            let mut value = Value::from_type(<gst::Caps as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"restriction-caps\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_track_type(&self) -> TrackType {
        unsafe {
            let mut value = Value::from_type(<TrackType as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"track-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"commited\0".as_ptr() as *const _,
                Some(transmute(commited_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_track_element_added<F: Fn(&Self, &TrackElement) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"track-element-added\0".as_ptr() as *const _,
                Some(transmute(track_element_added_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_track_element_removed<F: Fn(&Self, &TrackElement) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"track-element-removed\0".as_ptr() as *const _,
                Some(transmute(track_element_removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::duration\0".as_ptr() as *const _,
                Some(transmute(notify_duration_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_mixing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mixing\0".as_ptr() as *const _,
                Some(transmute(notify_mixing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_restriction_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::restriction-caps\0".as_ptr() as *const _,
                Some(transmute(notify_restriction_caps_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn commited_trampoline<P, F: Fn(&P) + 'static>(this: *mut ges_sys::GESTrack, f: glib_sys::gpointer)
where P: IsA<Track> {
    let f: &F = &*(f as *const F);
    f(&Track::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn track_element_added_trampoline<P, F: Fn(&P, &TrackElement) + 'static>(this: *mut ges_sys::GESTrack, effect: *mut ges_sys::GESTrackElement, f: glib_sys::gpointer)
where P: IsA<Track> {
    let f: &F = &*(f as *const F);
    f(&Track::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(effect))
}

unsafe extern "C" fn track_element_removed_trampoline<P, F: Fn(&P, &TrackElement) + 'static>(this: *mut ges_sys::GESTrack, effect: *mut ges_sys::GESTrackElement, f: glib_sys::gpointer)
where P: IsA<Track> {
    let f: &F = &*(f as *const F);
    f(&Track::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(effect))
}

unsafe extern "C" fn notify_duration_trampoline<P, F: Fn(&P) + 'static>(this: *mut ges_sys::GESTrack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Track> {
    let f: &F = &*(f as *const F);
    f(&Track::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_mixing_trampoline<P, F: Fn(&P) + 'static>(this: *mut ges_sys::GESTrack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Track> {
    let f: &F = &*(f as *const F);
    f(&Track::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_restriction_caps_trampoline<P, F: Fn(&P) + 'static>(this: *mut ges_sys::GESTrack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Track> {
    let f: &F = &*(f as *const F);
    f(&Track::from_glib_borrow(this).unsafe_cast())
}
