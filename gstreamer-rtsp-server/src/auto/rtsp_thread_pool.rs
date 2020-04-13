// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gst_rtsp_server_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use RTSPContext;
use RTSPThread;
use RTSPThreadType;

glib_wrapper! {
    pub struct RTSPThreadPool(Object<gst_rtsp_server_sys::GstRTSPThreadPool, gst_rtsp_server_sys::GstRTSPThreadPoolClass, RTSPThreadPoolClass>);

    match fn {
        get_type => || gst_rtsp_server_sys::gst_rtsp_thread_pool_get_type(),
    }
}

impl RTSPThreadPool {
    pub fn new() -> RTSPThreadPool {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_rtsp_server_sys::gst_rtsp_thread_pool_new()) }
    }

    pub fn cleanup() {
        assert_initialized_main_thread!();
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_thread_pool_cleanup();
        }
    }
}

impl Default for RTSPThreadPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPThreadPool {}
unsafe impl Sync for RTSPThreadPool {}

pub const NONE_RTSP_THREAD_POOL: Option<&RTSPThreadPool> = None;

pub trait RTSPThreadPoolExt: 'static {
    fn get_max_threads(&self) -> i32;

    fn get_thread(&self, type_: RTSPThreadType, ctx: &RTSPContext) -> Option<RTSPThread>;

    fn set_max_threads(&self, max_threads: i32);

    fn connect_property_max_threads_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPThreadPool>> RTSPThreadPoolExt for O {
    fn get_max_threads(&self) -> i32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_thread_pool_get_max_threads(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    fn get_thread(&self, type_: RTSPThreadType, ctx: &RTSPContext) -> Option<RTSPThread> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_thread_pool_get_thread(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
                ctx.to_glib_none().0,
            ))
        }
    }

    fn set_max_threads(&self, max_threads: i32) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_thread_pool_set_max_threads(
                self.as_ref().to_glib_none().0,
                max_threads,
            );
        }
    }

    fn connect_property_max_threads_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_threads_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPThreadPool,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPThreadPool>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPThreadPool::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-threads\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_threads_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
