// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Object;
use crate::Stream;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstStreamCollection")]
    pub struct StreamCollection(Object<ffi::GstStreamCollection, ffi::GstStreamCollectionClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_stream_collection_get_type(),
    }
}

impl StreamCollection {
    #[doc(alias = "gst_stream_collection_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> u32 {
        unsafe { ffi::gst_stream_collection_get_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_stream_collection_get_stream")]
    #[doc(alias = "get_stream")]
    pub fn stream(&self, index: u32) -> Option<Stream> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_stream(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[doc(alias = "gst_stream_collection_get_upstream_id")]
    #[doc(alias = "get_upstream_id")]
    pub fn upstream_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_upstream_id(
                self.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "stream-notify")]
    //pub fn connect_stream_notify<Unsupported or ignored types>(&self, detail: Option<&str>, f: F) -> SignalHandlerId {
    //    Ignored p0: GObject.ParamSpec
    //}
}

unsafe impl Send for StreamCollection {}
unsafe impl Sync for StreamCollection {}
