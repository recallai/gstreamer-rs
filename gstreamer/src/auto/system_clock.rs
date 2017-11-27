// This file was generated by gir (d50d839) from gir-files (???)
// DO NOT EDIT

use Clock;
use ClockType;
use Object;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SystemClock(Object<ffi::GstSystemClock, ffi::GstSystemClockClass>): Clock, Object;

    match fn {
        get_type => || ffi::gst_system_clock_get_type(),
    }
}

impl SystemClock {
    pub fn obtain() -> Clock {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_system_clock_obtain())
        }
    }

    pub fn set_default<P: IsA<Clock>>(new_clock: &P) {
        skip_assert_initialized!();
        unsafe {
            ffi::gst_system_clock_set_default(new_clock.to_glib_none().0);
        }
    }
}

unsafe impl Send for SystemClock {}
unsafe impl Sync for SystemClock {}

pub trait SystemClockExt {
    fn get_property_clock_type(&self) -> ClockType;

    fn set_property_clock_type(&self, clock_type: ClockType);

    fn connect_property_clock_type_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SystemClock> + IsA<glib::object::Object>> SystemClockExt for O {
    fn get_property_clock_type(&self) -> ClockType {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <ClockType as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "clock-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_clock_type(&self, clock_type: ClockType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "clock-type".to_glib_none().0, Value::from(&clock_type).to_glib_none().0);
        }
    }

    fn connect_property_clock_type_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::clock-type",
                transmute(notify_clock_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_clock_type_trampoline<P>(this: *mut ffi::GstSystemClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SystemClock> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&SystemClock::from_glib_borrow(this).downcast_unchecked())
}
