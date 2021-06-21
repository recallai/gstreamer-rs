// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::LFOWaveform;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstLFOControlSource")]
    pub struct LFOControlSource(Object<ffi::GstLFOControlSource, ffi::GstLFOControlSourceClass>) @extends gst::ControlSource, gst::Object;

    match fn {
        type_ => || ffi::gst_lfo_control_source_get_type(),
    }
}

impl LFOControlSource {
    #[doc(alias = "gst_lfo_control_source_new")]
    pub fn new() -> LFOControlSource {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlSource::from_glib_full(ffi::gst_lfo_control_source_new()).unsafe_cast()
        }
    }
}

impl Default for LFOControlSource {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for LFOControlSource {}
unsafe impl Sync for LFOControlSource {}

pub const NONE_LFO_CONTROL_SOURCE: Option<&LFOControlSource> = None;

pub trait LFOControlSourceExt: 'static {
    fn amplitude(&self) -> f64;

    fn set_amplitude(&self, amplitude: f64);

    fn frequency(&self) -> f64;

    fn set_frequency(&self, frequency: f64);

    fn offset(&self) -> f64;

    fn set_offset(&self, offset: f64);

    fn timeshift(&self) -> u64;

    fn set_timeshift(&self, timeshift: u64);

    fn waveform(&self) -> LFOWaveform;

    fn set_waveform(&self, waveform: LFOWaveform);

    #[doc(alias = "amplitude")]
    fn connect_amplitude_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "frequency")]
    fn connect_frequency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "offset")]
    fn connect_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "timeshift")]
    fn connect_timeshift_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "waveform")]
    fn connect_waveform_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<LFOControlSource>> LFOControlSourceExt for O {
    fn amplitude(&self) -> f64 {
        unsafe {
            let mut value = glib::Value::from_type(<f64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"amplitude\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `amplitude` getter")
        }
    }

    fn set_amplitude(&self, amplitude: f64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"amplitude\0".as_ptr() as *const _,
                amplitude.to_value().to_glib_none().0,
            );
        }
    }

    fn frequency(&self) -> f64 {
        unsafe {
            let mut value = glib::Value::from_type(<f64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"frequency\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `frequency` getter")
        }
    }

    fn set_frequency(&self, frequency: f64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"frequency\0".as_ptr() as *const _,
                frequency.to_value().to_glib_none().0,
            );
        }
    }

    fn offset(&self) -> f64 {
        unsafe {
            let mut value = glib::Value::from_type(<f64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"offset\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `offset` getter")
        }
    }

    fn set_offset(&self, offset: f64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"offset\0".as_ptr() as *const _,
                offset.to_value().to_glib_none().0,
            );
        }
    }

    fn timeshift(&self) -> u64 {
        unsafe {
            let mut value = glib::Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"timeshift\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `timeshift` getter")
        }
    }

    fn set_timeshift(&self, timeshift: u64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"timeshift\0".as_ptr() as *const _,
                timeshift.to_value().to_glib_none().0,
            );
        }
    }

    fn waveform(&self) -> LFOWaveform {
        unsafe {
            let mut value = glib::Value::from_type(<LFOWaveform as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"waveform\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `waveform` getter")
        }
    }

    fn set_waveform(&self, waveform: LFOWaveform) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"waveform\0".as_ptr() as *const _,
                waveform.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_amplitude_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_amplitude_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::amplitude\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_amplitude_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_frequency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_frequency_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::frequency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_frequency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_timeshift_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeshift_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeshift\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeshift_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_waveform_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_waveform_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::waveform\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_waveform_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
