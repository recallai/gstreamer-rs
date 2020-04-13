// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use gst_audio_sys;
use gst_base;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct AudioBaseSrc(Object<gst_audio_sys::GstAudioBaseSrc, gst_audio_sys::GstAudioBaseSrcClass, AudioBaseSrcClass>) @extends gst_base::BaseSrc, gst::Element, gst::Object;

    match fn {
        get_type => || gst_audio_sys::gst_audio_base_src_get_type(),
    }
}

unsafe impl Send for AudioBaseSrc {}
unsafe impl Sync for AudioBaseSrc {}

pub const NONE_AUDIO_BASE_SRC: Option<&AudioBaseSrc> = None;

pub trait AudioBaseSrcExt: 'static {
    //fn create_ringbuffer(&self) -> /*Ignored*/Option<AudioRingBuffer>;

    fn get_provide_clock(&self) -> bool;

    //fn get_slave_method(&self) -> /*Ignored*/AudioBaseSrcSlaveMethod;

    fn set_provide_clock(&self, provide: bool);

    //fn set_slave_method(&self, method: /*Ignored*/AudioBaseSrcSlaveMethod);

    fn get_property_actual_buffer_time(&self) -> i64;

    fn get_property_actual_latency_time(&self) -> i64;

    fn get_property_buffer_time(&self) -> i64;

    fn set_property_buffer_time(&self, buffer_time: i64);

    fn get_property_latency_time(&self) -> i64;

    fn set_property_latency_time(&self, latency_time: i64);

    fn connect_property_actual_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_actual_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_provide_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_slave_method_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AudioBaseSrc>> AudioBaseSrcExt for O {
    //fn create_ringbuffer(&self) -> /*Ignored*/Option<AudioRingBuffer> {
    //    unsafe { TODO: call gst_audio_sys:gst_audio_base_src_create_ringbuffer() }
    //}

    fn get_provide_clock(&self) -> bool {
        unsafe {
            from_glib(gst_audio_sys::gst_audio_base_src_get_provide_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_slave_method(&self) -> /*Ignored*/AudioBaseSrcSlaveMethod {
    //    unsafe { TODO: call gst_audio_sys:gst_audio_base_src_get_slave_method() }
    //}

    fn set_provide_clock(&self, provide: bool) {
        unsafe {
            gst_audio_sys::gst_audio_base_src_set_provide_clock(
                self.as_ref().to_glib_none().0,
                provide.to_glib(),
            );
        }
    }

    //fn set_slave_method(&self, method: /*Ignored*/AudioBaseSrcSlaveMethod) {
    //    unsafe { TODO: call gst_audio_sys:gst_audio_base_src_set_slave_method() }
    //}

    fn get_property_actual_buffer_time(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"actual-buffer-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `actual-buffer-time` getter")
                .unwrap()
        }
    }

    fn get_property_actual_latency_time(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"actual-latency-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `actual-latency-time` getter")
                .unwrap()
        }
    }

    fn get_property_buffer_time(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"buffer-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffer-time` getter")
                .unwrap()
        }
    }

    fn set_property_buffer_time(&self, buffer_time: i64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"buffer-time\0".as_ptr() as *const _,
                Value::from(&buffer_time).to_glib_none().0,
            );
        }
    }

    fn get_property_latency_time(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"latency-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `latency-time` getter")
                .unwrap()
        }
    }

    fn set_property_latency_time(&self, latency_time: i64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"latency-time\0".as_ptr() as *const _,
                Value::from(&latency_time).to_glib_none().0,
            );
        }
    }

    fn connect_property_actual_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_actual_buffer_time_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_audio_sys::GstAudioBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioBaseSrc>,
        {
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

    fn connect_property_actual_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_actual_latency_time_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_audio_sys::GstAudioBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioBaseSrc>,
        {
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

    fn connect_property_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_time_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_audio_sys::GstAudioBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioBaseSrc>,
        {
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

    fn connect_property_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_latency_time_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_audio_sys::GstAudioBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioBaseSrc>,
        {
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

    fn connect_property_provide_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_provide_clock_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_audio_sys::GstAudioBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioBaseSrc>,
        {
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

    fn connect_property_slave_method_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_slave_method_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_audio_sys::GstAudioBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioBaseSrc>,
        {
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
