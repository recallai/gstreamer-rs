// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use NetClientClock;
use glib::translate::*;
use gst;
use gst_net_sys;

glib_wrapper! {
    pub struct NtpClock(Object<gst_net_sys::GstNtpClock, gst_net_sys::GstNtpClockClass, NtpClockClass>) @extends NetClientClock, gst::Clock, gst::Object;

    match fn {
        get_type => || gst_net_sys::gst_ntp_clock_get_type(),
    }
}

impl NtpClock {}

unsafe impl Send for NtpClock {}
unsafe impl Sync for NtpClock {}
