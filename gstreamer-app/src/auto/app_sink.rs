// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use gst_app_sys;
use gst_base;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct AppSink(Object<gst_app_sys::GstAppSink, gst_app_sys::GstAppSinkClass, AppSinkClass>) @extends gst_base::BaseSink, gst::Element, gst::Object, @implements gst::URIHandler;

    match fn {
        get_type => || gst_app_sys::gst_app_sink_get_type(),
    }
}

impl AppSink {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_buffer_list_support(&self) -> bool {
        unsafe {
            from_glib(gst_app_sys::gst_app_sink_get_buffer_list_support(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_caps(&self) -> Option<gst::Caps> {
        unsafe { from_glib_full(gst_app_sys::gst_app_sink_get_caps(self.to_glib_none().0)) }
    }

    pub fn get_drop(&self) -> bool {
        unsafe { from_glib(gst_app_sys::gst_app_sink_get_drop(self.to_glib_none().0)) }
    }

    pub fn get_emit_signals(&self) -> bool {
        unsafe {
            from_glib(gst_app_sys::gst_app_sink_get_emit_signals(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_max_buffers(&self) -> u32 {
        unsafe { gst_app_sys::gst_app_sink_get_max_buffers(self.to_glib_none().0) }
    }

    pub fn get_wait_on_eos(&self) -> bool {
        unsafe {
            from_glib(gst_app_sys::gst_app_sink_get_wait_on_eos(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn is_eos(&self) -> bool {
        unsafe { from_glib(gst_app_sys::gst_app_sink_is_eos(self.to_glib_none().0)) }
    }

    pub fn pull_preroll(&self) -> Result<gst::Sample, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_app_sys::gst_app_sink_pull_preroll(
                self.to_glib_none().0,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to pull preroll sample"))
        }
    }

    pub fn pull_sample(&self) -> Result<gst::Sample, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_app_sys::gst_app_sink_pull_sample(
                self.to_glib_none().0,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to pull sample"))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn set_buffer_list_support(&self, enable_lists: bool) {
        unsafe {
            gst_app_sys::gst_app_sink_set_buffer_list_support(
                self.to_glib_none().0,
                enable_lists.to_glib(),
            );
        }
    }

    //pub fn set_callbacks(&self, callbacks: /*Ignored*/&mut AppSinkCallbacks, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gst_app_sys:gst_app_sink_set_callbacks() }
    //}

    pub fn set_caps(&self, caps: Option<&gst::Caps>) {
        unsafe {
            gst_app_sys::gst_app_sink_set_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    pub fn set_drop(&self, drop: bool) {
        unsafe {
            gst_app_sys::gst_app_sink_set_drop(self.to_glib_none().0, drop.to_glib());
        }
    }

    pub fn set_emit_signals(&self, emit: bool) {
        unsafe {
            gst_app_sys::gst_app_sink_set_emit_signals(self.to_glib_none().0, emit.to_glib());
        }
    }

    pub fn set_max_buffers(&self, max: u32) {
        unsafe {
            gst_app_sys::gst_app_sink_set_max_buffers(self.to_glib_none().0, max);
        }
    }

    pub fn set_wait_on_eos(&self, wait: bool) {
        unsafe {
            gst_app_sys::gst_app_sink_set_wait_on_eos(self.to_glib_none().0, wait.to_glib());
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn try_pull_preroll(&self, timeout: gst::ClockTime) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(gst_app_sys::gst_app_sink_try_pull_preroll(
                self.to_glib_none().0,
                timeout.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn try_pull_sample(&self, timeout: gst::ClockTime) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(gst_app_sys::gst_app_sink_try_pull_sample(
                self.to_glib_none().0,
                timeout.to_glib(),
            ))
        }
    }

    pub fn get_property_buffer_list(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"buffer-list\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffer-list` getter")
                .unwrap()
        }
    }

    pub fn set_property_buffer_list(&self, buffer_list: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"buffer-list\0".as_ptr() as *const _,
                Value::from(&buffer_list).to_glib_none().0,
            );
        }
    }

    pub fn get_property_eos(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"eos\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `eos` getter")
                .unwrap()
        }
    }

    pub fn connect_eos<F: Fn(&AppSink) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn eos_trampoline<F: Fn(&AppSink) + Send + 'static>(
            this: *mut gst_app_sys::GstAppSink,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"eos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    eos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_buffer_list_notify<F: Fn(&AppSink) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_list_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut gst_app_sys::GstAppSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-list\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffer_list_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_caps_notify<F: Fn(&AppSink) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut gst_app_sys::GstAppSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_drop_notify<F: Fn(&AppSink) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_drop_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut gst_app_sys::GstAppSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_emit_signals_notify<F: Fn(&AppSink) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_emit_signals_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut gst_app_sys::GstAppSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::emit-signals\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_emit_signals_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_eos_notify<F: Fn(&AppSink) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_eos_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut gst_app_sys::GstAppSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::eos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_eos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_max_buffers_notify<F: Fn(&AppSink) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_buffers_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut gst_app_sys::GstAppSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-buffers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_buffers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_wait_on_eos_notify<F: Fn(&AppSink) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_wait_on_eos_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut gst_app_sys::GstAppSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wait-on-eos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wait_on_eos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for AppSink {}
unsafe impl Sync for AppSink {}
