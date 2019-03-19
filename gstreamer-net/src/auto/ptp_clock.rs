// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::StaticType;
use glib::Value;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gst;
use gst_net_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct PtpClock(Object<gst_net_sys::GstPtpClock, gst_net_sys::GstPtpClockClass, PtpClockClass>) @extends gst::Clock, gst::Object;

    match fn {
        get_type => || gst_net_sys::gst_ptp_clock_get_type(),
    }
}

impl PtpClock {
    pub fn get_property_domain(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"domain\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_grandmaster_clock_id(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"grandmaster-clock-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_internal_clock(&self) -> Option<gst::Clock> {
        unsafe {
            let mut value = Value::from_type(<gst::Clock as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"internal-clock\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn get_property_master_clock_id(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"master-clock-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn connect_property_grandmaster_clock_id_notify<F: Fn(&PtpClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::grandmaster-clock-id\0".as_ptr() as *const _,
                Some(transmute(notify_grandmaster_clock_id_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_internal_clock_notify<F: Fn(&PtpClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::internal-clock\0".as_ptr() as *const _,
                Some(transmute(notify_internal_clock_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_master_clock_id_notify<F: Fn(&PtpClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::master-clock-id\0".as_ptr() as *const _,
                Some(transmute(notify_master_clock_id_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe impl Send for PtpClock {}
unsafe impl Sync for PtpClock {}

unsafe extern "C" fn notify_grandmaster_clock_id_trampoline<F: Fn(&PtpClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstPtpClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_internal_clock_trampoline<F: Fn(&PtpClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstPtpClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_master_clock_id_trampoline<F: Fn(&PtpClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstPtpClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}
