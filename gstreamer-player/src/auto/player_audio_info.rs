// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use PlayerStreamInfo;
use glib::GString;
use glib::translate::*;
use gst_player_sys;

glib_wrapper! {
    pub struct PlayerAudioInfo(Object<gst_player_sys::GstPlayerAudioInfo, gst_player_sys::GstPlayerAudioInfoClass, PlayerAudioInfoClass>) @extends PlayerStreamInfo;

    match fn {
        get_type => || gst_player_sys::gst_player_audio_info_get_type(),
    }
}

impl PlayerAudioInfo {
    pub fn get_bitrate(&self) -> i32 {
        unsafe {
            gst_player_sys::gst_player_audio_info_get_bitrate(self.to_glib_none().0)
        }
    }

    pub fn get_channels(&self) -> i32 {
        unsafe {
            gst_player_sys::gst_player_audio_info_get_channels(self.to_glib_none().0)
        }
    }

    pub fn get_language(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_player_sys::gst_player_audio_info_get_language(self.to_glib_none().0))
        }
    }

    pub fn get_max_bitrate(&self) -> i32 {
        unsafe {
            gst_player_sys::gst_player_audio_info_get_max_bitrate(self.to_glib_none().0)
        }
    }

    pub fn get_sample_rate(&self) -> i32 {
        unsafe {
            gst_player_sys::gst_player_audio_info_get_sample_rate(self.to_glib_none().0)
        }
    }
}

unsafe impl Send for PlayerAudioInfo {}
unsafe impl Sync for PlayerAudioInfo {}
