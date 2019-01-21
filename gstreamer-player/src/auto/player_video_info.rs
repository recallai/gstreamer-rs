// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use PlayerStreamInfo;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct PlayerVideoInfo(Object<ffi::GstPlayerVideoInfo, ffi::GstPlayerVideoInfoClass, PlayerVideoInfoClass>) @extends PlayerStreamInfo;

    match fn {
        get_type => || ffi::gst_player_video_info_get_type(),
    }
}

impl PlayerVideoInfo {
    pub fn get_bitrate(&self) -> i32 {
        unsafe {
            ffi::gst_player_video_info_get_bitrate(self.to_glib_none().0)
        }
    }

    pub fn get_height(&self) -> i32 {
        unsafe {
            ffi::gst_player_video_info_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_max_bitrate(&self) -> i32 {
        unsafe {
            ffi::gst_player_video_info_get_max_bitrate(self.to_glib_none().0)
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe {
            ffi::gst_player_video_info_get_width(self.to_glib_none().0)
        }
    }
}

unsafe impl Send for PlayerVideoInfo {}
unsafe impl Sync for PlayerVideoInfo {}
