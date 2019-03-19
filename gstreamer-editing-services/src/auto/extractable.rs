// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Asset;
use ges_sys;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Extractable(Interface<ges_sys::GESExtractable>);

    match fn {
        get_type => || ges_sys::ges_extractable_get_type(),
    }
}

pub const NONE_EXTRACTABLE: Option<&Extractable> = None;

pub trait ExtractableExt: 'static {
    fn get_asset(&self) -> Option<Asset>;

    fn get_id(&self) -> Option<GString>;

    fn set_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<Extractable>> ExtractableExt for O {
    fn get_asset(&self) -> Option<Asset> {
        unsafe {
            from_glib_none(ges_sys::ges_extractable_get_asset(self.as_ref().to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ges_sys::ges_extractable_get_id(self.as_ref().to_glib_none().0))
        }
    }

    fn set_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ges_sys::ges_extractable_set_asset(self.as_ref().to_glib_none().0, asset.as_ref().to_glib_none().0), "Failed to set asset")
        }
    }
}
