// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Asset;
use Error;
use Timeline;
use ges_sys;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gst_pbutils;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Project(Object<ges_sys::GESProject, ges_sys::GESProjectClass, ProjectClass>) @extends Asset;

    match fn {
        get_type => || ges_sys::ges_project_get_type(),
    }
}

impl Project {
    pub fn new(uri: Option<&str>) -> Project {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ges_sys::ges_project_new(uri.to_glib_none().0))
        }
    }
}

pub const NONE_PROJECT: Option<&Project> = None;

pub trait ProjectExt: 'static {
    fn add_asset<P: IsA<Asset>>(&self, asset: &P) -> bool;

    fn add_encoding_profile<P: IsA<gst_pbutils::EncodingProfile>>(&self, profile: &P) -> Result<(), glib::error::BoolError>;

    fn create_asset(&self, id: Option<&str>, extractable_type: glib::types::Type) -> bool;

    fn create_asset_sync(&self, id: Option<&str>, extractable_type: glib::types::Type) -> Result<Option<Asset>, Error>;

    fn get_asset(&self, id: &str, extractable_type: glib::types::Type) -> Option<Asset>;

    fn get_loading_assets(&self) -> Vec<Asset>;

    fn get_uri(&self) -> Option<GString>;

    fn list_assets(&self, filter: glib::types::Type) -> Vec<Asset>;

    fn list_encoding_profiles(&self) -> Vec<gst_pbutils::EncodingProfile>;

    fn load<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), Error>;

    fn remove_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<(), glib::error::BoolError>;

    fn save<P: IsA<Timeline>, Q: IsA<Asset>>(&self, timeline: &P, uri: &str, formatter_asset: Option<&Q>, overwrite: bool) -> Result<(), Error>;

    fn connect_asset_added<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_asset_loading<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_asset_removed<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_error_loading_asset<F: Fn(&Self, &Error, &str, glib::types::Type) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_loaded<F: Fn(&Self, &Timeline) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_missing_uri<F: Fn(&Self, &Error, &Asset) -> Option<GString> + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Project>> ProjectExt for O {
    fn add_asset<P: IsA<Asset>>(&self, asset: &P) -> bool {
        unsafe {
            from_glib(ges_sys::ges_project_add_asset(self.as_ref().to_glib_none().0, asset.as_ref().to_glib_none().0))
        }
    }

    fn add_encoding_profile<P: IsA<gst_pbutils::EncodingProfile>>(&self, profile: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ges_sys::ges_project_add_encoding_profile(self.as_ref().to_glib_none().0, profile.as_ref().to_glib_none().0), "Failed to add profile")
        }
    }

    fn create_asset(&self, id: Option<&str>, extractable_type: glib::types::Type) -> bool {
        unsafe {
            from_glib(ges_sys::ges_project_create_asset(self.as_ref().to_glib_none().0, id.to_glib_none().0, extractable_type.to_glib()))
        }
    }

    fn create_asset_sync(&self, id: Option<&str>, extractable_type: glib::types::Type) -> Result<Option<Asset>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ges_sys::ges_project_create_asset_sync(self.as_ref().to_glib_none().0, id.to_glib_none().0, extractable_type.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_asset(&self, id: &str, extractable_type: glib::types::Type) -> Option<Asset> {
        unsafe {
            from_glib_full(ges_sys::ges_project_get_asset(self.as_ref().to_glib_none().0, id.to_glib_none().0, extractable_type.to_glib()))
        }
    }

    fn get_loading_assets(&self) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ges_sys::ges_project_get_loading_assets(self.as_ref().to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ges_sys::ges_project_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn list_assets(&self, filter: glib::types::Type) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ges_sys::ges_project_list_assets(self.as_ref().to_glib_none().0, filter.to_glib()))
        }
    }

    fn list_encoding_profiles(&self) -> Vec<gst_pbutils::EncodingProfile> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ges_sys::ges_project_list_encoding_profiles(self.as_ref().to_glib_none().0))
        }
    }

    fn load<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ges_sys::ges_project_load(self.as_ref().to_glib_none().0, timeline.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ges_sys::ges_project_remove_asset(self.as_ref().to_glib_none().0, asset.as_ref().to_glib_none().0), "Failed to remove asset")
        }
    }

    fn save<P: IsA<Timeline>, Q: IsA<Asset>>(&self, timeline: &P, uri: &str, formatter_asset: Option<&Q>, overwrite: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ges_sys::ges_project_save(self.as_ref().to_glib_none().0, timeline.as_ref().to_glib_none().0, uri.to_glib_none().0, formatter_asset.map(|p| p.as_ref()).to_glib_none().0, overwrite.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_asset_added<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"asset-added\0".as_ptr() as *const _,
                Some(transmute(asset_added_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_asset_loading<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"asset-loading\0".as_ptr() as *const _,
                Some(transmute(asset_loading_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_asset_removed<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"asset-removed\0".as_ptr() as *const _,
                Some(transmute(asset_removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_error_loading_asset<F: Fn(&Self, &Error, &str, glib::types::Type) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"error-loading-asset\0".as_ptr() as *const _,
                Some(transmute(error_loading_asset_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_loaded<F: Fn(&Self, &Timeline) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"loaded\0".as_ptr() as *const _,
                Some(transmute(loaded_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_missing_uri<F: Fn(&Self, &Error, &Asset) -> Option<GString> + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"missing-uri\0".as_ptr() as *const _,
                Some(transmute(missing_uri_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn asset_added_trampoline<P, F: Fn(&P, &Asset) + 'static>(this: *mut ges_sys::GESProject, asset: *mut ges_sys::GESAsset, f: glib_sys::gpointer)
where P: IsA<Project> {
    let f: &F = &*(f as *const F);
    f(&Project::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(asset))
}

unsafe extern "C" fn asset_loading_trampoline<P, F: Fn(&P, &Asset) + 'static>(this: *mut ges_sys::GESProject, asset: *mut ges_sys::GESAsset, f: glib_sys::gpointer)
where P: IsA<Project> {
    let f: &F = &*(f as *const F);
    f(&Project::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(asset))
}

unsafe extern "C" fn asset_removed_trampoline<P, F: Fn(&P, &Asset) + 'static>(this: *mut ges_sys::GESProject, asset: *mut ges_sys::GESAsset, f: glib_sys::gpointer)
where P: IsA<Project> {
    let f: &F = &*(f as *const F);
    f(&Project::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(asset))
}

unsafe extern "C" fn error_loading_asset_trampoline<P, F: Fn(&P, &Error, &str, glib::types::Type) + 'static>(this: *mut ges_sys::GESProject, error: *mut glib_sys::GError, id: *mut libc::c_char, extractable_type: glib_sys::GType, f: glib_sys::gpointer)
where P: IsA<Project> {
    let f: &F = &*(f as *const F);
    f(&Project::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(error), &GString::from_glib_borrow(id), from_glib(extractable_type))
}

unsafe extern "C" fn loaded_trampoline<P, F: Fn(&P, &Timeline) + 'static>(this: *mut ges_sys::GESProject, timeline: *mut ges_sys::GESTimeline, f: glib_sys::gpointer)
where P: IsA<Project> {
    let f: &F = &*(f as *const F);
    f(&Project::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(timeline))
}

unsafe extern "C" fn missing_uri_trampoline<P, F: Fn(&P, &Error, &Asset) -> Option<GString> + 'static>(this: *mut ges_sys::GESProject, error: *mut glib_sys::GError, wrong_asset: *mut ges_sys::GESAsset, f: glib_sys::gpointer) -> *mut libc::c_char
where P: IsA<Project> {
    let f: &F = &*(f as *const F);
    f(&Project::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(error), &from_glib_borrow(wrong_asset)).to_glib_full()
}
