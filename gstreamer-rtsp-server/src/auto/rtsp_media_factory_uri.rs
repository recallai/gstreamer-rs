// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use RTSPMediaFactory;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gst_rtsp_server_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct RTSPMediaFactoryURI(Object<gst_rtsp_server_sys::GstRTSPMediaFactoryURI, gst_rtsp_server_sys::GstRTSPMediaFactoryURIClass, RTSPMediaFactoryURIClass>) @extends RTSPMediaFactory;

    match fn {
        get_type => || gst_rtsp_server_sys::gst_rtsp_media_factory_uri_get_type(),
    }
}

impl RTSPMediaFactoryURI {
    pub fn new() -> RTSPMediaFactoryURI {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_factory_uri_new())
        }
    }
}

impl Default for RTSPMediaFactoryURI {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMediaFactoryURI {}
unsafe impl Sync for RTSPMediaFactoryURI {}

pub const NONE_RTSP_MEDIA_FACTORY_URI: Option<&RTSPMediaFactoryURI> = None;

pub trait RTSPMediaFactoryURIExt: 'static {
    fn get_uri(&self) -> Option<GString>;

    fn set_uri(&self, uri: &str);

    fn get_property_use_gstpay(&self) -> bool;

    fn set_property_use_gstpay(&self, use_gstpay: bool);

    fn connect_property_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_gstpay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPMediaFactoryURI>> RTSPMediaFactoryURIExt for O {
    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_factory_uri_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_uri_set_uri(self.as_ref().to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn get_property_use_gstpay(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"use-gstpay\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_use_gstpay(&self, use_gstpay: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"use-gstpay\0".as_ptr() as *const _, Value::from(&use_gstpay).to_glib_none().0);
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uri\0".as_ptr() as *const _,
                Some(transmute(notify_uri_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_gstpay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-gstpay\0".as_ptr() as *const _,
                Some(transmute(notify_use_gstpay_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_uri_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMediaFactoryURI, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<RTSPMediaFactoryURI> {
    let f: &F = &*(f as *const F);
    f(&RTSPMediaFactoryURI::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_gstpay_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMediaFactoryURI, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<RTSPMediaFactoryURI> {
    let f: &F = &*(f as *const F);
    f(&RTSPMediaFactoryURI::from_glib_borrow(this).unsafe_cast())
}
