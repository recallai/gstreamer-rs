// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use gst_web_rtc_sys;
use WebRTCRTPReceiver;
use WebRTCRTPSender;

glib_wrapper! {
    pub struct WebRTCRTPTransceiver(Object<gst_web_rtc_sys::GstWebRTCRTPTransceiver, gst_web_rtc_sys::GstWebRTCRTPTransceiverClass, WebRTCRTPTransceiverClass>);

    match fn {
        get_type => || gst_web_rtc_sys::gst_webrtc_rtp_transceiver_get_type(),
    }
}

impl WebRTCRTPTransceiver {
    pub fn get_property_mlineindex(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"mlineindex\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `mlineindex` getter")
                .unwrap()
        }
    }

    pub fn get_property_receiver(&self) -> Option<WebRTCRTPReceiver> {
        unsafe {
            let mut value = Value::from_type(<WebRTCRTPReceiver as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"receiver\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `receiver` getter")
        }
    }

    pub fn get_property_sender(&self) -> Option<WebRTCRTPSender> {
        unsafe {
            let mut value = Value::from_type(<WebRTCRTPSender as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"sender\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `sender` getter")
        }
    }
}

unsafe impl Send for WebRTCRTPTransceiver {}
unsafe impl Sync for WebRTCRTPTransceiver {}
