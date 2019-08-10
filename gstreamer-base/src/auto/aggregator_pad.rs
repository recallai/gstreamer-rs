// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::Value;
use glib_sys;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use gobject_sys;
use gst;
use gst_base_sys;
use gst_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct AggregatorPad(Object<gst_base_sys::GstAggregatorPad, gst_base_sys::GstAggregatorPadClass, AggregatorPadClass>) @extends gst::Pad, gst::Object;

    match fn {
        get_type => || gst_base_sys::gst_aggregator_pad_get_type(),
    }
}

unsafe impl Send for AggregatorPad {}
unsafe impl Sync for AggregatorPad {}

pub const NONE_AGGREGATOR_PAD: Option<&AggregatorPad> = None;

pub trait AggregatorPadExt: 'static {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn drop_buffer(&self) -> bool;

    #[cfg(any(feature = "v1_14_1", feature = "dox"))]
    fn has_buffer(&self) -> bool;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn is_eos(&self) -> bool;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn peek_buffer(&self) -> Option<gst::Buffer>;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn pop_buffer(&self) -> Option<gst::Buffer>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_property_emit_signals(&self) -> bool;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_property_emit_signals(&self, emit_signals: bool);

    fn connect_buffer_consumed<F: Fn(&Self, &gst::Buffer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn connect_property_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AggregatorPad>> AggregatorPadExt for O {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn drop_buffer(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_aggregator_pad_drop_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14_1", feature = "dox"))]
    fn has_buffer(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_aggregator_pad_has_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn is_eos(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_aggregator_pad_is_eos(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn peek_buffer(&self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_base_sys::gst_aggregator_pad_peek_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn pop_buffer(&self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_base_sys::gst_aggregator_pad_pop_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_property_emit_signals(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"emit-signals\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `emit-signals` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_property_emit_signals(&self, emit_signals: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"emit-signals\0".as_ptr() as *const _,
                Value::from(&emit_signals).to_glib_none().0,
            );
        }
    }

    fn connect_buffer_consumed<F: Fn(&Self, &gst::Buffer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn buffer_consumed_trampoline<
            P,
            F: Fn(&P, &gst::Buffer) + Send + Sync + 'static,
        >(
            this: *mut gst_base_sys::GstAggregatorPad,
            object: *mut gst_sys::GstBuffer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AggregatorPad>,
        {
            let f: &F = &*(f as *const F);
            f(
                &AggregatorPad::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"buffer-consumed\0".as_ptr() as *const _,
                Some(transmute(buffer_consumed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn connect_property_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_emit_signals_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstAggregatorPad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AggregatorPad>,
        {
            let f: &F = &*(f as *const F);
            f(&AggregatorPad::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::emit-signals\0".as_ptr() as *const _,
                Some(transmute(
                    notify_emit_signals_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}
