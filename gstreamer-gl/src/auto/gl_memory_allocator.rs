// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::GLBaseMemoryAllocator;
use crate::GLContext;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstGLMemoryAllocator")]
    pub struct GLMemoryAllocator(Object<ffi::GstGLMemoryAllocator, ffi::GstGLMemoryAllocatorClass>) @extends GLBaseMemoryAllocator, gst::Allocator, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_memory_allocator_get_type(),
    }
}

impl GLMemoryAllocator {
    #[doc(alias = "gst_gl_memory_allocator_get_default")]
    #[doc(alias = "get_default")]
    pub fn default<P: IsA<GLContext>>(context: &P) -> Option<GLMemoryAllocator> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_memory_allocator_get_default(
                context.as_ref().to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for GLMemoryAllocator {}
unsafe impl Sync for GLMemoryAllocator {}

pub const NONE_GL_MEMORY_ALLOCATOR: Option<&GLMemoryAllocator> = None;
