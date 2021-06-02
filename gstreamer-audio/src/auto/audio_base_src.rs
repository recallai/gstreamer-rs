// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

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
    #[doc(alias = "GstAudioBaseSrc")]
    pub struct AudioBaseSrc(Object<ffi::GstAudioBaseSrc, ffi::GstAudioBaseSrcClass>) @extends gst_base::BaseSrc, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_base_src_get_type(),
    }
}

unsafe impl Send for AudioBaseSrc {}
unsafe impl Sync for AudioBaseSrc {}

pub const NONE_AUDIO_BASE_SRC: Option<&AudioBaseSrc> = None;

pub trait AudioBaseSrcExt: 'static {
    //#[doc(alias = "gst_audio_base_src_create_ringbuffer")]
    //fn create_ringbuffer(&self) -> /*Ignored*/Option<AudioRingBuffer>;

    #[doc(alias = "gst_audio_base_src_get_provide_clock")]
    #[doc(alias = "get_provide_clock")]
    fn is_provide_clock(&self) -> bool;

    //#[doc(alias = "gst_audio_base_src_get_slave_method")]
    //#[doc(alias = "get_slave_method")]
    //fn slave_method(&self) -> /*Ignored*/AudioBaseSrcSlaveMethod;

    #[doc(alias = "gst_audio_base_src_set_provide_clock")]
    fn set_provide_clock(&self, provide: bool);

    //#[doc(alias = "gst_audio_base_src_set_slave_method")]
    //fn set_slave_method(&self, method: /*Ignored*/AudioBaseSrcSlaveMethod);

    #[doc(alias = "actual-buffer-time")]
    fn actual_buffer_time(&self) -> i64;

    #[doc(alias = "actual-latency-time")]
    fn actual_latency_time(&self) -> i64;

    #[doc(alias = "buffer-time")]
    fn buffer_time(&self) -> i64;

    #[doc(alias = "buffer-time")]
    fn set_buffer_time(&self, buffer_time: i64);

    #[doc(alias = "latency-time")]
    fn latency_time(&self) -> i64;

    #[doc(alias = "latency-time")]
    fn set_latency_time(&self, latency_time: i64);

    #[doc(alias = "actual-buffer-time")]
    fn connect_actual_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "actual-latency-time")]
    fn connect_actual_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "buffer-time")]
    fn connect_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "latency-time")]
    fn connect_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "provide-clock")]
    fn connect_provide_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "slave-method")]
    fn connect_slave_method_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AudioBaseSrc>> AudioBaseSrcExt for O {
    //fn create_ringbuffer(&self) -> /*Ignored*/Option<AudioRingBuffer> {
    //    unsafe { TODO: call ffi:gst_audio_base_src_create_ringbuffer() }
    //}

    fn is_provide_clock(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_base_src_get_provide_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn slave_method(&self) -> /*Ignored*/AudioBaseSrcSlaveMethod {
    //    unsafe { TODO: call ffi:gst_audio_base_src_get_slave_method() }
    //}

    fn set_provide_clock(&self, provide: bool) {
        unsafe {
            ffi::gst_audio_base_src_set_provide_clock(
                self.as_ref().to_glib_none().0,
                provide.into_glib(),
            );
        }
    }

    //fn set_slave_method(&self, method: /*Ignored*/AudioBaseSrcSlaveMethod) {
    //    unsafe { TODO: call ffi:gst_audio_base_src_set_slave_method() }
    //}

    fn actual_buffer_time(&self) -> i64 {
        unsafe {
            let mut value = glib::Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"actual-buffer-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `actual-buffer-time` getter")
        }
    }

    fn actual_latency_time(&self) -> i64 {
        unsafe {
            let mut value = glib::Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"actual-latency-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `actual-latency-time` getter")
        }
    }

    fn buffer_time(&self) -> i64 {
        unsafe {
            let mut value = glib::Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"buffer-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffer-time` getter")
        }
    }

    fn set_buffer_time(&self, buffer_time: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"buffer-time\0".as_ptr() as *const _,
                buffer_time.to_value().to_glib_none().0,
            );
        }
    }

    fn latency_time(&self) -> i64 {
        unsafe {
            let mut value = glib::Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"latency-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `latency-time` getter")
        }
    }

    fn set_latency_time(&self, latency_time: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"latency-time\0".as_ptr() as *const _,
                latency_time.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "actual-buffer-time")]
    fn connect_actual_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_actual_buffer_time_trampoline<
            P: IsA<AudioBaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::actual-buffer-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actual_buffer_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "actual-latency-time")]
    fn connect_actual_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_actual_latency_time_trampoline<
            P: IsA<AudioBaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::actual-latency-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actual_latency_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "buffer-time")]
    fn connect_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_time_trampoline<
            P: IsA<AudioBaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffer_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "latency-time")]
    fn connect_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_latency_time_trampoline<
            P: IsA<AudioBaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::latency-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_latency_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "provide-clock")]
    fn connect_provide_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_provide_clock_trampoline<
            P: IsA<AudioBaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::provide-clock\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_provide_clock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "slave-method")]
    fn connect_slave_method_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_slave_method_trampoline<
            P: IsA<AudioBaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::slave-method\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_slave_method_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
