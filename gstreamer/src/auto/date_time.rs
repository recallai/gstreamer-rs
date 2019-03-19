// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::GString;
use glib::translate::*;
use gst_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DateTime(Shared<gst_sys::GstDateTime>);

    match fn {
        ref => |ptr| gst_sys::gst_date_time_ref(ptr),
        unref => |ptr| gst_sys::gst_date_time_unref(ptr),
        get_type => || gst_sys::gst_date_time_get_type(),
    }
}

impl DateTime {
    pub fn new(tzoffset: f32, year: i32, month: i32, day: i32, hour: i32, minute: i32, seconds: f64) -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new(tzoffset, year, month, day, hour, minute, seconds))
        }
    }

    pub fn new_from_g_date_time(dt: &glib::DateTime) -> Option<DateTime> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_from_g_date_time(dt.to_glib_full()))
        }
    }

    pub fn new_from_iso8601_string(string: &str) -> Option<DateTime> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_from_iso8601_string(string.to_glib_none().0))
        }
    }

    pub fn new_from_unix_epoch_local_time(secs: i64) -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_from_unix_epoch_local_time(secs))
        }
    }

    pub fn new_from_unix_epoch_utc(secs: i64) -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_from_unix_epoch_utc(secs))
        }
    }

    pub fn new_local_time(year: i32, month: i32, day: i32, hour: i32, minute: i32, seconds: f64) -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_local_time(year, month, day, hour, minute, seconds))
        }
    }

    pub fn new_now_local_time() -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_now_local_time())
        }
    }

    pub fn new_now_utc() -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_now_utc())
        }
    }

    pub fn new_y(year: i32) -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_y(year))
        }
    }

    pub fn new_ym(year: i32, month: i32) -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_ym(year, month))
        }
    }

    pub fn new_ymd(year: i32, month: i32, day: i32) -> DateTime {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_sys::gst_date_time_new_ymd(year, month, day))
        }
    }

    pub fn get_day(&self) -> i32 {
        unsafe {
            gst_sys::gst_date_time_get_day(self.to_glib_none().0)
        }
    }

    pub fn get_hour(&self) -> i32 {
        unsafe {
            gst_sys::gst_date_time_get_hour(self.to_glib_none().0)
        }
    }

    pub fn get_microsecond(&self) -> i32 {
        unsafe {
            gst_sys::gst_date_time_get_microsecond(self.to_glib_none().0)
        }
    }

    pub fn get_minute(&self) -> i32 {
        unsafe {
            gst_sys::gst_date_time_get_minute(self.to_glib_none().0)
        }
    }

    pub fn get_month(&self) -> i32 {
        unsafe {
            gst_sys::gst_date_time_get_month(self.to_glib_none().0)
        }
    }

    pub fn get_second(&self) -> i32 {
        unsafe {
            gst_sys::gst_date_time_get_second(self.to_glib_none().0)
        }
    }

    pub fn get_time_zone_offset(&self) -> f32 {
        unsafe {
            gst_sys::gst_date_time_get_time_zone_offset(self.to_glib_none().0)
        }
    }

    pub fn get_year(&self) -> i32 {
        unsafe {
            gst_sys::gst_date_time_get_year(self.to_glib_none().0)
        }
    }

    pub fn has_day(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_date_time_has_day(self.to_glib_none().0))
        }
    }

    pub fn has_month(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_date_time_has_month(self.to_glib_none().0))
        }
    }

    pub fn has_second(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_date_time_has_second(self.to_glib_none().0))
        }
    }

    pub fn has_time(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_date_time_has_time(self.to_glib_none().0))
        }
    }

    pub fn has_year(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_date_time_has_year(self.to_glib_none().0))
        }
    }

    pub fn to_g_date_time(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_full(gst_sys::gst_date_time_to_g_date_time(self.to_glib_none().0))
        }
    }

    pub fn to_iso8601_string(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gst_sys::gst_date_time_to_iso8601_string(self.to_glib_none().0))
        }
    }
}

unsafe impl Send for DateTime {}
unsafe impl Sync for DateTime {}
