// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstARGBControlBinding")]
    pub struct ARGBControlBinding(Object<ffi::GstARGBControlBinding, ffi::GstARGBControlBindingClass>) @extends gst::ControlBinding, gst::Object;

    match fn {
        type_ => || ffi::gst_argb_control_binding_get_type(),
    }
}

impl ARGBControlBinding {
    #[doc(alias = "gst_argb_control_binding_new")]
    pub fn new<
        P: IsA<gst::Object>,
        Q: IsA<gst::ControlSource>,
        R: IsA<gst::ControlSource>,
        S: IsA<gst::ControlSource>,
        T: IsA<gst::ControlSource>,
    >(
        object: &P,
        property_name: &str,
        cs_a: &Q,
        cs_r: &R,
        cs_g: &S,
        cs_b: &T,
    ) -> ARGBControlBinding {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlBinding::from_glib_none(ffi::gst_argb_control_binding_new(
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                cs_a.as_ref().to_glib_none().0,
                cs_r.as_ref().to_glib_none().0,
                cs_g.as_ref().to_glib_none().0,
                cs_b.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

unsafe impl Send for ARGBControlBinding {}
unsafe impl Sync for ARGBControlBinding {}

pub const NONE_ARGB_CONTROL_BINDING: Option<&ARGBControlBinding> = None;

pub trait ARGBControlBindingExt: 'static {
    #[doc(alias = "control-source-a")]
    fn control_source_a(&self) -> Option<gst::ControlSource>;

    #[doc(alias = "control-source-a")]
    fn set_control_source_a<P: IsA<gst::ControlSource>>(&self, control_source_a: Option<&P>);

    #[doc(alias = "control-source-b")]
    fn control_source_b(&self) -> Option<gst::ControlSource>;

    #[doc(alias = "control-source-b")]
    fn set_control_source_b<P: IsA<gst::ControlSource>>(&self, control_source_b: Option<&P>);

    #[doc(alias = "control-source-g")]
    fn control_source_g(&self) -> Option<gst::ControlSource>;

    #[doc(alias = "control-source-g")]
    fn set_control_source_g<P: IsA<gst::ControlSource>>(&self, control_source_g: Option<&P>);

    #[doc(alias = "control-source-r")]
    fn control_source_r(&self) -> Option<gst::ControlSource>;

    #[doc(alias = "control-source-r")]
    fn set_control_source_r<P: IsA<gst::ControlSource>>(&self, control_source_r: Option<&P>);

    #[doc(alias = "control-source-a")]
    fn connect_control_source_a_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "control-source-b")]
    fn connect_control_source_b_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "control-source-g")]
    fn connect_control_source_g_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "control-source-r")]
    fn connect_control_source_r_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<ARGBControlBinding>> ARGBControlBindingExt for O {
    fn control_source_a(&self) -> Option<gst::ControlSource> {
        unsafe {
            let mut value =
                glib::Value::from_type(<gst::ControlSource as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source-a\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `control-source-a` getter")
        }
    }

    fn set_control_source_a<P: IsA<gst::ControlSource>>(&self, control_source_a: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source-a\0".as_ptr() as *const _,
                control_source_a.to_value().to_glib_none().0,
            );
        }
    }

    fn control_source_b(&self) -> Option<gst::ControlSource> {
        unsafe {
            let mut value =
                glib::Value::from_type(<gst::ControlSource as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source-b\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `control-source-b` getter")
        }
    }

    fn set_control_source_b<P: IsA<gst::ControlSource>>(&self, control_source_b: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source-b\0".as_ptr() as *const _,
                control_source_b.to_value().to_glib_none().0,
            );
        }
    }

    fn control_source_g(&self) -> Option<gst::ControlSource> {
        unsafe {
            let mut value =
                glib::Value::from_type(<gst::ControlSource as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source-g\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `control-source-g` getter")
        }
    }

    fn set_control_source_g<P: IsA<gst::ControlSource>>(&self, control_source_g: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source-g\0".as_ptr() as *const _,
                control_source_g.to_value().to_glib_none().0,
            );
        }
    }

    fn control_source_r(&self) -> Option<gst::ControlSource> {
        unsafe {
            let mut value =
                glib::Value::from_type(<gst::ControlSource as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source-r\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `control-source-r` getter")
        }
    }

    fn set_control_source_r<P: IsA<gst::ControlSource>>(&self, control_source_r: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source-r\0".as_ptr() as *const _,
                control_source_r.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_control_source_a_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_a_trampoline<
            P: IsA<ARGBControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstARGBControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ARGBControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source-a\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_a_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_control_source_b_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_b_trampoline<
            P: IsA<ARGBControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstARGBControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ARGBControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source-b\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_b_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_control_source_g_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_g_trampoline<
            P: IsA<ARGBControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstARGBControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ARGBControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source-g\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_g_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_control_source_r_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_r_trampoline<
            P: IsA<ARGBControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstARGBControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ARGBControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source-r\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_r_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
