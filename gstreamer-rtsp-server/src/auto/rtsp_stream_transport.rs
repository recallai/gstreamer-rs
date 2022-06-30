// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPStream;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstRTSPStreamTransport")]
    pub struct RTSPStreamTransport(Object<ffi::GstRTSPStreamTransport, ffi::GstRTSPStreamTransportClass>);

    match fn {
        type_ => || ffi::gst_rtsp_stream_transport_get_type(),
    }
}

impl RTSPStreamTransport {
    pub const NONE: Option<&'static RTSPStreamTransport> = None;

    //#[doc(alias = "gst_rtsp_stream_transport_new")]
    //pub fn new(stream: &impl IsA<RTSPStream>, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> RTSPStreamTransport {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_transport_new() }
    //}
}

pub trait RTSPStreamTransportExt: 'static {
    #[doc(alias = "gst_rtsp_stream_transport_get_rtpinfo")]
    #[doc(alias = "get_rtpinfo")]
    fn rtpinfo(&self, start_time: impl Into<Option<gst::ClockTime>>) -> Option<glib::GString>;

    #[doc(alias = "gst_rtsp_stream_transport_get_stream")]
    #[doc(alias = "get_stream")]
    fn stream(&self) -> Option<RTSPStream>;

    //#[doc(alias = "gst_rtsp_stream_transport_get_transport")]
    //#[doc(alias = "get_transport")]
    //fn transport(&self) -> /*Ignored*/Option<gst_rtsp::RTSPTransport>;

    #[doc(alias = "gst_rtsp_stream_transport_get_url")]
    #[doc(alias = "get_url")]
    fn url(&self) -> Option<gst_rtsp::RTSPUrl>;

    #[doc(alias = "gst_rtsp_stream_transport_is_timed_out")]
    fn is_timed_out(&self) -> bool;

    #[doc(alias = "gst_rtsp_stream_transport_keep_alive")]
    fn keep_alive(&self);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_transport_message_sent")]
    fn message_sent(&self);

    #[doc(alias = "gst_rtsp_stream_transport_recv_data")]
    fn recv_data(
        &self,
        channel: u32,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_rtsp_stream_transport_send_rtcp")]
    fn send_rtcp(&self, buffer: &gst::Buffer) -> Result<(), glib::error::BoolError>;

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "gst_rtsp_stream_transport_send_rtcp_list")]
    //fn send_rtcp_list(&self, buffer_list: /*Ignored*/&gst::BufferList) -> bool;

    #[doc(alias = "gst_rtsp_stream_transport_send_rtp")]
    fn send_rtp(&self, buffer: &gst::Buffer) -> Result<(), glib::error::BoolError>;

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "gst_rtsp_stream_transport_send_rtp_list")]
    //fn send_rtp_list(&self, buffer_list: /*Ignored*/&gst::BufferList) -> bool;

    #[doc(alias = "gst_rtsp_stream_transport_set_active")]
    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError>;

    //#[doc(alias = "gst_rtsp_stream_transport_set_callbacks")]
    //fn set_callbacks<P: Fn(&gst::Buffer, u8) -> bool + 'static, Q: Fn(&gst::Buffer, u8) -> bool + 'static>(&self, send_rtp: P, send_rtcp: Q);

    #[doc(alias = "gst_rtsp_stream_transport_set_keepalive")]
    fn set_keepalive<P: Fn() + 'static>(&self, keep_alive: P);

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "gst_rtsp_stream_transport_set_list_callbacks")]
    //fn set_list_callbacks(&self, send_rtp_list: /*Unimplemented*/Fn(/*Ignored*/gst::BufferList, u8) -> bool, send_rtcp_list: /*Unimplemented*/Fn(/*Ignored*/gst::BufferList, u8) -> bool, user_data: /*Unimplemented*/Option<Basic: Pointer>);

    #[doc(alias = "gst_rtsp_stream_transport_set_message_sent")]
    fn set_message_sent<P: Fn() + 'static>(&self, message_sent: P);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_stream_transport_set_message_sent_full")]
    fn set_message_sent_full<P: Fn(&RTSPStreamTransport) + 'static>(&self, message_sent: P);

    #[doc(alias = "gst_rtsp_stream_transport_set_timed_out")]
    fn set_timed_out(&self, timedout: bool);

    //#[doc(alias = "gst_rtsp_stream_transport_set_transport")]
    //fn set_transport(&self, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport);

    #[doc(alias = "gst_rtsp_stream_transport_set_url")]
    fn set_url(&self, url: Option<&gst_rtsp::RTSPUrl>);
}

impl<O: IsA<RTSPStreamTransport>> RTSPStreamTransportExt for O {
    fn rtpinfo(&self, start_time: impl Into<Option<gst::ClockTime>>) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_transport_get_rtpinfo(
                self.as_ref().to_glib_none().0,
                start_time.into().into_glib(),
            ))
        }
    }

    fn stream(&self) -> Option<RTSPStream> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_stream_transport_get_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn transport(&self) -> /*Ignored*/Option<gst_rtsp::RTSPTransport> {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_transport_get_transport() }
    //}

    fn url(&self) -> Option<gst_rtsp::RTSPUrl> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_stream_transport_get_url(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_timed_out(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_transport_is_timed_out(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn keep_alive(&self) {
        unsafe {
            ffi::gst_rtsp_stream_transport_keep_alive(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn message_sent(&self) {
        unsafe {
            ffi::gst_rtsp_stream_transport_message_sent(self.as_ref().to_glib_none().0);
        }
    }

    fn recv_data(
        &self,
        channel: u32,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtsp_stream_transport_recv_data(
                self.as_ref().to_glib_none().0,
                channel,
                buffer.to_glib_full(),
            ))
        }
    }

    fn send_rtcp(&self, buffer: &gst::Buffer) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_transport_send_rtcp(
                    self.as_ref().to_glib_none().0,
                    buffer.to_glib_none().0
                ),
                "Failed to send rtcp"
            )
        }
    }

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //fn send_rtcp_list(&self, buffer_list: /*Ignored*/&gst::BufferList) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_transport_send_rtcp_list() }
    //}

    fn send_rtp(&self, buffer: &gst::Buffer) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_transport_send_rtp(
                    self.as_ref().to_glib_none().0,
                    buffer.to_glib_none().0
                ),
                "Failed to send rtp"
            )
        }
    }

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //fn send_rtp_list(&self, buffer_list: /*Ignored*/&gst::BufferList) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_transport_send_rtp_list() }
    //}

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_transport_set_active(
                    self.as_ref().to_glib_none().0,
                    active.into_glib()
                ),
                "Failed to set active"
            )
        }
    }

    //fn set_callbacks<P: Fn(&gst::Buffer, u8) -> bool + 'static, Q: Fn(&gst::Buffer, u8) -> bool + 'static>(&self, send_rtp: P, send_rtcp: Q) {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_transport_set_callbacks() }
    //}

    fn set_keepalive<P: Fn() + 'static>(&self, keep_alive: P) {
        let keep_alive_data: Box_<P> = Box_::new(keep_alive);
        unsafe extern "C" fn keep_alive_func<P: Fn() + 'static>(user_data: glib::ffi::gpointer) {
            let callback: &P = &*(user_data as *mut _);
            (*callback)();
        }
        let keep_alive = Some(keep_alive_func::<P> as _);
        unsafe extern "C" fn notify_func<P: Fn() + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(notify_func::<P> as _);
        let super_callback0: Box_<P> = keep_alive_data;
        unsafe {
            ffi::gst_rtsp_stream_transport_set_keepalive(
                self.as_ref().to_glib_none().0,
                keep_alive,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //fn set_list_callbacks(&self, send_rtp_list: /*Unimplemented*/Fn(/*Ignored*/gst::BufferList, u8) -> bool, send_rtcp_list: /*Unimplemented*/Fn(/*Ignored*/gst::BufferList, u8) -> bool, user_data: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_transport_set_list_callbacks() }
    //}

    fn set_message_sent<P: Fn() + 'static>(&self, message_sent: P) {
        let message_sent_data: Box_<P> = Box_::new(message_sent);
        unsafe extern "C" fn message_sent_func<P: Fn() + 'static>(user_data: glib::ffi::gpointer) {
            let callback: &P = &*(user_data as *mut _);
            (*callback)();
        }
        let message_sent = Some(message_sent_func::<P> as _);
        unsafe extern "C" fn notify_func<P: Fn() + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(notify_func::<P> as _);
        let super_callback0: Box_<P> = message_sent_data;
        unsafe {
            ffi::gst_rtsp_stream_transport_set_message_sent(
                self.as_ref().to_glib_none().0,
                message_sent,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_message_sent_full<P: Fn(&RTSPStreamTransport) + 'static>(&self, message_sent: P) {
        let message_sent_data: Box_<P> = Box_::new(message_sent);
        unsafe extern "C" fn message_sent_func<P: Fn(&RTSPStreamTransport) + 'static>(
            trans: *mut ffi::GstRTSPStreamTransport,
            user_data: glib::ffi::gpointer,
        ) {
            let trans = from_glib_borrow(trans);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&trans);
        }
        let message_sent = Some(message_sent_func::<P> as _);
        unsafe extern "C" fn notify_func<P: Fn(&RTSPStreamTransport) + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(notify_func::<P> as _);
        let super_callback0: Box_<P> = message_sent_data;
        unsafe {
            ffi::gst_rtsp_stream_transport_set_message_sent_full(
                self.as_ref().to_glib_none().0,
                message_sent,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_timed_out(&self, timedout: bool) {
        unsafe {
            ffi::gst_rtsp_stream_transport_set_timed_out(
                self.as_ref().to_glib_none().0,
                timedout.into_glib(),
            );
        }
    }

    //fn set_transport(&self, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_transport_set_transport() }
    //}

    fn set_url(&self, url: Option<&gst_rtsp::RTSPUrl>) {
        unsafe {
            ffi::gst_rtsp_stream_transport_set_url(
                self.as_ref().to_glib_none().0,
                url.to_glib_none().0,
            );
        }
    }
}
