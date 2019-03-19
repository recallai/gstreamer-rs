// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use PlayerSignalDispatcher;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::ObjectType;
use glib::translate::*;
use gobject_sys;
use gst_player_sys;

glib_wrapper! {
    pub struct PlayerGMainContextSignalDispatcher(Object<gst_player_sys::GstPlayerGMainContextSignalDispatcher, gst_player_sys::GstPlayerGMainContextSignalDispatcherClass, PlayerGMainContextSignalDispatcherClass>) @implements PlayerSignalDispatcher;

    match fn {
        get_type => || gst_player_sys::gst_player_g_main_context_signal_dispatcher_get_type(),
    }
}

impl PlayerGMainContextSignalDispatcher {
    pub fn get_property_application_context(&self) -> Option<glib::MainContext> {
        unsafe {
            let mut value = Value::from_type(<glib::MainContext as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"application-context\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }
}

unsafe impl Send for PlayerGMainContextSignalDispatcher {}
unsafe impl Sync for PlayerGMainContextSignalDispatcher {}
