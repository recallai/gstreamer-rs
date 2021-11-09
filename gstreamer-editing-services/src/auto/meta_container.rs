// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::MarkerList;
use crate::MetaFlag;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GESMetaContainer")]
    pub struct MetaContainer(Interface<ffi::GESMetaContainer, ffi::GESMetaContainerInterface>);

    match fn {
        type_ => || ffi::ges_meta_container_get_type(),
    }
}

impl MetaContainer {
    pub const NONE: Option<&'static MetaContainer> = None;
}

pub trait MetaContainerExt: 'static {
    #[doc(alias = "ges_meta_container_add_metas_from_string")]
    fn add_metas_from_string(&self, str: &str) -> bool;

    #[doc(alias = "ges_meta_container_check_meta_registered")]
    fn check_meta_registered(&self, meta_item: &str) -> Option<(MetaFlag, glib::types::Type)>;

    #[doc(alias = "ges_meta_container_foreach")]
    fn foreach<P: FnMut(&MetaContainer, &str, &glib::Value)>(&self, func: P);

    #[doc(alias = "ges_meta_container_get_boolean")]
    #[doc(alias = "get_boolean")]
    fn boolean(&self, meta_item: &str) -> Option<bool>;

    #[doc(alias = "ges_meta_container_get_date")]
    #[doc(alias = "get_date")]
    fn date(&self, meta_item: &str) -> Option<glib::Date>;

    #[doc(alias = "ges_meta_container_get_date_time")]
    #[doc(alias = "get_date_time")]
    fn date_time(&self, meta_item: &str) -> Option<gst::DateTime>;

    #[doc(alias = "ges_meta_container_get_double")]
    #[doc(alias = "get_double")]
    fn double(&self, meta_item: &str) -> Option<f64>;

    #[doc(alias = "ges_meta_container_get_float")]
    #[doc(alias = "get_float")]
    fn float(&self, meta_item: &str) -> Option<f32>;

    #[doc(alias = "ges_meta_container_get_int")]
    #[doc(alias = "get_int")]
    fn int(&self, meta_item: &str) -> Option<i32>;

    #[doc(alias = "ges_meta_container_get_int64")]
    #[doc(alias = "get_int64")]
    fn int64(&self, meta_item: &str) -> Option<i64>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_meta_container_get_marker_list")]
    #[doc(alias = "get_marker_list")]
    fn marker_list(&self, key: &str) -> Option<MarkerList>;

    #[doc(alias = "ges_meta_container_get_meta")]
    #[doc(alias = "get_meta")]
    fn meta(&self, key: &str) -> Option<glib::Value>;

    #[doc(alias = "ges_meta_container_get_string")]
    #[doc(alias = "get_string")]
    fn string(&self, meta_item: &str) -> Option<glib::GString>;

    #[doc(alias = "ges_meta_container_get_uint")]
    #[doc(alias = "get_uint")]
    fn uint(&self, meta_item: &str) -> Option<u32>;

    #[doc(alias = "ges_meta_container_get_uint64")]
    #[doc(alias = "get_uint64")]
    fn uint64(&self, meta_item: &str) -> Option<u64>;

    #[doc(alias = "ges_meta_container_metas_to_string")]
    fn metas_to_string(&self) -> Option<glib::GString>;

    #[doc(alias = "ges_meta_container_register_meta")]
    fn register_meta(&self, flags: MetaFlag, meta_item: &str, value: &glib::Value) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_boolean")]
    fn register_meta_boolean(&self, flags: MetaFlag, meta_item: &str, value: bool) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_date")]
    fn register_meta_date(&self, flags: MetaFlag, meta_item: &str, value: &glib::Date) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_date_time")]
    fn register_meta_date_time(
        &self,
        flags: MetaFlag,
        meta_item: &str,
        value: &gst::DateTime,
    ) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_double")]
    fn register_meta_double(&self, flags: MetaFlag, meta_item: &str, value: f64) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_float")]
    fn register_meta_float(&self, flags: MetaFlag, meta_item: &str, value: f32) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_int")]
    fn register_meta_int(&self, flags: MetaFlag, meta_item: &str, value: i32) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_int64")]
    fn register_meta_int64(&self, flags: MetaFlag, meta_item: &str, value: i64) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_string")]
    fn register_meta_string(&self, flags: MetaFlag, meta_item: &str, value: &str) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_uint")]
    fn register_meta_uint(&self, flags: MetaFlag, meta_item: &str, value: u32) -> bool;

    #[doc(alias = "ges_meta_container_register_meta_uint64")]
    fn register_meta_uint64(&self, flags: MetaFlag, meta_item: &str, value: u64) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_meta_container_register_static_meta")]
    fn register_static_meta(
        &self,
        flags: MetaFlag,
        meta_item: &str,
        type_: glib::types::Type,
    ) -> bool;

    #[doc(alias = "ges_meta_container_set_boolean")]
    fn set_boolean(&self, meta_item: &str, value: bool) -> bool;

    #[doc(alias = "ges_meta_container_set_date")]
    fn set_date(&self, meta_item: &str, value: &glib::Date) -> bool;

    #[doc(alias = "ges_meta_container_set_date_time")]
    fn set_date_time(&self, meta_item: &str, value: &gst::DateTime) -> bool;

    #[doc(alias = "ges_meta_container_set_double")]
    fn set_double(&self, meta_item: &str, value: f64) -> bool;

    #[doc(alias = "ges_meta_container_set_float")]
    fn set_float(&self, meta_item: &str, value: f32) -> bool;

    #[doc(alias = "ges_meta_container_set_int")]
    fn set_int(&self, meta_item: &str, value: i32) -> bool;

    #[doc(alias = "ges_meta_container_set_int64")]
    fn set_int64(&self, meta_item: &str, value: i64) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_meta_container_set_marker_list")]
    fn set_marker_list(&self, meta_item: &str, list: &MarkerList) -> bool;

    #[doc(alias = "ges_meta_container_set_meta")]
    fn set_meta(&self, meta_item: &str, value: Option<&glib::Value>) -> bool;

    #[doc(alias = "ges_meta_container_set_string")]
    fn set_string(&self, meta_item: &str, value: &str) -> bool;

    #[doc(alias = "ges_meta_container_set_uint")]
    fn set_uint(&self, meta_item: &str, value: u32) -> bool;

    #[doc(alias = "ges_meta_container_set_uint64")]
    fn set_uint64(&self, meta_item: &str, value: u64) -> bool;

    #[doc(alias = "notify-meta")]
    fn connect_notify_meta<F: Fn(&Self, &str, Option<&glib::Value>) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<MetaContainer>> MetaContainerExt for O {
    fn add_metas_from_string(&self, str: &str) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_add_metas_from_string(
                self.as_ref().to_glib_none().0,
                str.to_glib_none().0,
            ))
        }
    }

    fn check_meta_registered(&self, meta_item: &str) -> Option<(MetaFlag, glib::types::Type)> {
        unsafe {
            let mut flags = mem::MaybeUninit::uninit();
            let mut type_ = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_meta_container_check_meta_registered(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                flags.as_mut_ptr(),
                type_.as_mut_ptr(),
            ));
            let flags = flags.assume_init();
            let type_ = type_.assume_init();
            if ret {
                Some((from_glib(flags), from_glib(type_)))
            } else {
                None
            }
        }
    }

    fn foreach<P: FnMut(&MetaContainer, &str, &glib::Value)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&MetaContainer, &str, &glib::Value)>(
            container: *const ffi::GESMetaContainer,
            key: *const libc::c_char,
            value: *const glib::gobject_ffi::GValue,
            user_data: glib::ffi::gpointer,
        ) {
            let container = from_glib_borrow(container);
            let key: Borrowed<glib::GString> = from_glib_borrow(key);
            let value = from_glib_borrow(value);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&container, key.as_str(), &value);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::ges_meta_container_foreach(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn boolean(&self, meta_item: &str) -> Option<bool> {
        unsafe {
            let mut dest = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_meta_container_get_boolean(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                dest.as_mut_ptr(),
            ));
            let dest = dest.assume_init();
            if ret {
                Some(from_glib(dest))
            } else {
                None
            }
        }
    }

    fn date(&self, meta_item: &str) -> Option<glib::Date> {
        unsafe {
            let mut dest = ptr::null_mut();
            let ret = from_glib(ffi::ges_meta_container_get_date(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                &mut dest,
            ));
            if ret {
                Some(from_glib_full(dest))
            } else {
                None
            }
        }
    }

    fn date_time(&self, meta_item: &str) -> Option<gst::DateTime> {
        unsafe {
            let mut dest = ptr::null_mut();
            let ret = from_glib(ffi::ges_meta_container_get_date_time(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                &mut dest,
            ));
            if ret {
                Some(from_glib_full(dest))
            } else {
                None
            }
        }
    }

    fn double(&self, meta_item: &str) -> Option<f64> {
        unsafe {
            let mut dest = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_meta_container_get_double(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                dest.as_mut_ptr(),
            ));
            let dest = dest.assume_init();
            if ret {
                Some(dest)
            } else {
                None
            }
        }
    }

    fn float(&self, meta_item: &str) -> Option<f32> {
        unsafe {
            let mut dest = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_meta_container_get_float(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                dest.as_mut_ptr(),
            ));
            let dest = dest.assume_init();
            if ret {
                Some(dest)
            } else {
                None
            }
        }
    }

    fn int(&self, meta_item: &str) -> Option<i32> {
        unsafe {
            let mut dest = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_meta_container_get_int(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                dest.as_mut_ptr(),
            ));
            let dest = dest.assume_init();
            if ret {
                Some(dest)
            } else {
                None
            }
        }
    }

    fn int64(&self, meta_item: &str) -> Option<i64> {
        unsafe {
            let mut dest = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_meta_container_get_int64(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                dest.as_mut_ptr(),
            ));
            let dest = dest.assume_init();
            if ret {
                Some(dest)
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn marker_list(&self, key: &str) -> Option<MarkerList> {
        unsafe {
            from_glib_full(ffi::ges_meta_container_get_marker_list(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn meta(&self, key: &str) -> Option<glib::Value> {
        unsafe {
            from_glib_none(ffi::ges_meta_container_get_meta(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn string(&self, meta_item: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ges_meta_container_get_string(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
            ))
        }
    }

    fn uint(&self, meta_item: &str) -> Option<u32> {
        unsafe {
            let mut dest = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_meta_container_get_uint(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                dest.as_mut_ptr(),
            ));
            let dest = dest.assume_init();
            if ret {
                Some(dest)
            } else {
                None
            }
        }
    }

    fn uint64(&self, meta_item: &str) -> Option<u64> {
        unsafe {
            let mut dest = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_meta_container_get_uint64(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                dest.as_mut_ptr(),
            ));
            let dest = dest.assume_init();
            if ret {
                Some(dest)
            } else {
                None
            }
        }
    }

    fn metas_to_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::ges_meta_container_metas_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn register_meta(&self, flags: MetaFlag, meta_item: &str, value: &glib::Value) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn register_meta_boolean(&self, flags: MetaFlag, meta_item: &str, value: bool) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_boolean(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value.into_glib(),
            ))
        }
    }

    fn register_meta_date(&self, flags: MetaFlag, meta_item: &str, value: &glib::Date) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_date(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn register_meta_date_time(
        &self,
        flags: MetaFlag,
        meta_item: &str,
        value: &gst::DateTime,
    ) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_date_time(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn register_meta_double(&self, flags: MetaFlag, meta_item: &str, value: f64) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_double(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn register_meta_float(&self, flags: MetaFlag, meta_item: &str, value: f32) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_float(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn register_meta_int(&self, flags: MetaFlag, meta_item: &str, value: i32) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_int(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn register_meta_int64(&self, flags: MetaFlag, meta_item: &str, value: i64) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_int64(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn register_meta_string(&self, flags: MetaFlag, meta_item: &str, value: &str) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_string(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn register_meta_uint(&self, flags: MetaFlag, meta_item: &str, value: u32) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_uint(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn register_meta_uint64(&self, flags: MetaFlag, meta_item: &str, value: u64) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_meta_uint64(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn register_static_meta(
        &self,
        flags: MetaFlag,
        meta_item: &str,
        type_: glib::types::Type,
    ) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_register_static_meta(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                meta_item.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    fn set_boolean(&self, meta_item: &str, value: bool) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_boolean(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value.into_glib(),
            ))
        }
    }

    fn set_date(&self, meta_item: &str, value: &glib::Date) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_date(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn set_date_time(&self, meta_item: &str, value: &gst::DateTime) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_date_time(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn set_double(&self, meta_item: &str, value: f64) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_double(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_float(&self, meta_item: &str, value: f32) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_float(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_int(&self, meta_item: &str, value: i32) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_int(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_int64(&self, meta_item: &str, value: i64) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_int64(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_marker_list(&self, meta_item: &str, list: &MarkerList) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_marker_list(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                list.to_glib_none().0,
            ))
        }
    }

    fn set_meta(&self, meta_item: &str, value: Option<&glib::Value>) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_meta(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn set_string(&self, meta_item: &str, value: &str) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_string(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn set_uint(&self, meta_item: &str, value: u32) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_uint(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn set_uint64(&self, meta_item: &str, value: u64) -> bool {
        unsafe {
            from_glib(ffi::ges_meta_container_set_uint64(
                self.as_ref().to_glib_none().0,
                meta_item.to_glib_none().0,
                value,
            ))
        }
    }

    fn connect_notify_meta<F: Fn(&Self, &str, Option<&glib::Value>) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_meta_trampoline<
            P: IsA<MetaContainer>,
            F: Fn(&P, &str, Option<&glib::Value>) + 'static,
        >(
            this: *mut ffi::GESMetaContainer,
            key: *mut libc::c_char,
            value: *mut glib::gobject_ffi::GValue,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                MetaContainer::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(key),
                Option::<glib::Value>::from_glib_borrow(value)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("notify-meta::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"notify-meta\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_meta_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}