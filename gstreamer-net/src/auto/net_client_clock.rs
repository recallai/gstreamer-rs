// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::GString;
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
    pub struct NetClientClock(Object<gst_net_sys::GstNetClientClock, gst_net_sys::GstNetClientClockClass, NetClientClockClass>) @extends gst::Clock, gst::Object;

    match fn {
        get_type => || gst_net_sys::gst_net_client_clock_get_type(),
    }
}

impl NetClientClock {
    pub fn get_property_address(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"address\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_address(&self, address: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"address\0".as_ptr() as *const _, Value::from(address).to_glib_none().0);
        }
    }

    pub fn get_property_base_time(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"base-time\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_bus(&self) -> Option<gst::Bus> {
        unsafe {
            let mut value = Value::from_type(<gst::Bus as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"bus\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_bus(&self, bus: Option<&gst::Bus>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"bus\0".as_ptr() as *const _, Value::from(bus).to_glib_none().0);
        }
    }

    pub fn get_property_internal_clock(&self) -> Option<gst::Clock> {
        unsafe {
            let mut value = Value::from_type(<gst::Clock as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"internal-clock\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn get_property_minimum_update_interval(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"minimum-update-interval\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_minimum_update_interval(&self, minimum_update_interval: u64) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"minimum-update-interval\0".as_ptr() as *const _, Value::from(&minimum_update_interval).to_glib_none().0);
        }
    }

    pub fn get_property_port(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"port\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_port(&self, port: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"port\0".as_ptr() as *const _, Value::from(&port).to_glib_none().0);
        }
    }

    pub fn get_property_qos_dscp(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"qos-dscp\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_qos_dscp(&self, qos_dscp: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"qos-dscp\0".as_ptr() as *const _, Value::from(&qos_dscp).to_glib_none().0);
        }
    }

    pub fn get_property_round_trip_limit(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"round-trip-limit\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_round_trip_limit(&self, round_trip_limit: u64) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"round-trip-limit\0".as_ptr() as *const _, Value::from(&round_trip_limit).to_glib_none().0);
        }
    }

    pub fn connect_property_address_notify<F: Fn(&NetClientClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::address\0".as_ptr() as *const _,
                Some(transmute(notify_address_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_bus_notify<F: Fn(&NetClientClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::bus\0".as_ptr() as *const _,
                Some(transmute(notify_bus_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_internal_clock_notify<F: Fn(&NetClientClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::internal-clock\0".as_ptr() as *const _,
                Some(transmute(notify_internal_clock_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_minimum_update_interval_notify<F: Fn(&NetClientClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::minimum-update-interval\0".as_ptr() as *const _,
                Some(transmute(notify_minimum_update_interval_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_port_notify<F: Fn(&NetClientClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::port\0".as_ptr() as *const _,
                Some(transmute(notify_port_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_qos_dscp_notify<F: Fn(&NetClientClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::qos-dscp\0".as_ptr() as *const _,
                Some(transmute(notify_qos_dscp_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_round_trip_limit_notify<F: Fn(&NetClientClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::round-trip-limit\0".as_ptr() as *const _,
                Some(transmute(notify_round_trip_limit_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe impl Send for NetClientClock {}
unsafe impl Sync for NetClientClock {}

unsafe extern "C" fn notify_address_trampoline<F: Fn(&NetClientClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstNetClientClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_bus_trampoline<F: Fn(&NetClientClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstNetClientClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_internal_clock_trampoline<F: Fn(&NetClientClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstNetClientClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_minimum_update_interval_trampoline<F: Fn(&NetClientClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstNetClientClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_port_trampoline<F: Fn(&NetClientClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstNetClientClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_qos_dscp_trampoline<F: Fn(&NetClientClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstNetClientClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_round_trip_limit_trampoline<F: Fn(&NetClientClock) + Send + Sync + 'static>(this: *mut gst_net_sys::GstNetClientClock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}
