// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Element;

use once_cell::sync::Lazy;

use crate::prelude::*;
use crate::ClockTime;
use crate::ElementFlags;
use crate::Event;
use crate::Format;
use crate::FormattedValue;
use crate::GenericFormattedValue;
use crate::Pad;
use crate::PadTemplate;
use crate::Plugin;
use crate::QueryRef;
use crate::Rank;
use crate::SpecificFormattedValue;
use crate::State;
use crate::StateChange;
use crate::StateChangeError;
use crate::StateChangeReturn;
use crate::StateChangeSuccess;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::translate::FromGlibPtrBorrow;
use glib::translate::{
    from_glib, from_glib_full, from_glib_none, FromGlib, FromGlibPtrContainer, ToGlib, ToGlibPtr,
};

use std::ffi::CStr;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use std::future::Future;
use std::mem;
use std::num::NonZeroU64;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use std::pin::Pin;

impl Element {
    pub fn link_many<E: IsA<Element>>(elements: &[&E]) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        for e in elements.windows(2) {
            unsafe {
                glib::result_from_gboolean!(
                    ffi::gst_element_link(
                        e[0].as_ref().to_glib_none().0,
                        e[1].as_ref().to_glib_none().0,
                    ),
                    "Failed to link elements"
                )?;
            }
        }

        Ok(())
    }

    pub fn unlink_many<E: IsA<Element>>(elements: &[&E]) {
        skip_assert_initialized!();
        for e in elements.windows(2) {
            unsafe {
                ffi::gst_element_unlink(
                    e[0].as_ref().to_glib_none().0,
                    e[1].as_ref().to_glib_none().0,
                );
            }
        }
    }

    pub fn register(
        plugin: Option<&Plugin>,
        name: &str,
        rank: Rank,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_register(
                    plugin.to_glib_none().0,
                    name.to_glib_none().0,
                    rank.to_glib() as u32,
                    type_.to_glib()
                ),
                "Failed to register element factory"
            )
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum ElementMessageType {
    Error,
    Warning,
    Info,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotifyWatchId(NonZeroU64);

impl ToGlib for NotifyWatchId {
    type GlibType = libc::c_ulong;

    fn to_glib(&self) -> libc::c_ulong {
        self.0.get() as libc::c_ulong
    }
}

impl FromGlib<libc::c_ulong> for NotifyWatchId {
    unsafe fn from_glib(val: libc::c_ulong) -> NotifyWatchId {
        skip_assert_initialized!();
        assert_ne!(val, 0);
        NotifyWatchId(NonZeroU64::new_unchecked(val as u64))
    }
}

pub trait ElementExtManual: 'static {
    fn element_class(&self) -> &glib::Class<Element>;

    fn change_state(&self, transition: StateChange)
        -> Result<StateChangeSuccess, StateChangeError>;
    fn continue_state(
        &self,
        ret: StateChangeReturn,
    ) -> Result<StateChangeSuccess, StateChangeError>;

    fn state(
        &self,
        timeout: ClockTime,
    ) -> (Result<StateChangeSuccess, StateChangeError>, State, State);
    fn set_state(&self, state: State) -> Result<StateChangeSuccess, StateChangeError>;

    fn current_state(&self) -> State {
        self.state(ClockTime::from(0)).1
    }

    fn pending_state(&self) -> State {
        self.state(ClockTime::from(0)).2
    }

    fn query(&self, query: &mut QueryRef) -> bool;

    fn send_event(&self, event: Event) -> bool;

    fn metadata<'a>(&self, key: &str) -> Option<&'a str>;

    fn pad_template(&self, name: &str) -> Option<PadTemplate>;
    fn pad_template_list(&self) -> Vec<PadTemplate>;

    #[allow(clippy::too_many_arguments)]
    fn message_full<T: crate::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    );

    fn set_element_flags(&self, flags: ElementFlags);

    fn unset_element_flags(&self, flags: ElementFlags);

    fn element_flags(&self) -> ElementFlags;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[allow(clippy::too_many_arguments)]
    fn message_full_with_details<T: crate::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
        structure: crate::Structure,
    );

    fn post_message(&self, message: crate::Message) -> Result<(), glib::error::BoolError>;
    fn post_error_message(&self, msg: crate::ErrorMessage);

    fn iterate_pads(&self) -> crate::Iterator<Pad>;
    fn iterate_sink_pads(&self) -> crate::Iterator<Pad>;
    fn iterate_src_pads(&self) -> crate::Iterator<Pad>;

    fn pads(&self) -> Vec<Pad>;
    fn sink_pads(&self) -> Vec<Pad>;
    fn src_pads(&self) -> Vec<Pad>;

    fn num_pads(&self) -> u16;
    fn num_sink_pads(&self) -> u16;
    fn num_src_pads(&self) -> u16;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn add_property_deep_notify_watch(
        &self,
        property_name: Option<&str>,
        include_value: bool,
    ) -> NotifyWatchId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn add_property_notify_watch(
        &self,
        property_name: Option<&str>,
        include_value: bool,
    ) -> NotifyWatchId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn remove_property_notify_watch(&self, watch_id: NotifyWatchId);

    fn query_convert<V: Into<GenericFormattedValue>, U: SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U>;
    fn query_convert_generic<V: Into<GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<GenericFormattedValue>;

    fn query_duration<T: SpecificFormattedValue>(&self) -> Option<T>;
    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    fn query_position<T: SpecificFormattedValue>(&self) -> Option<T>;
    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    fn seek<V: Into<GenericFormattedValue>>(
        &self,
        rate: f64,
        flags: crate::SeekFlags,
        start_type: crate::SeekType,
        start: V,
        stop_type: crate::SeekType,
        stop: V,
    ) -> Result<(), glib::error::BoolError>;
    fn seek_simple<V: Into<GenericFormattedValue>>(
        &self,
        seek_flags: crate::SeekFlags,
        seek_pos: V,
    ) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn call_async<F>(&self, func: F)
    where
        F: FnOnce(&Self) + Send + 'static;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn call_async_future<F, T>(&self, func: F) -> Pin<Box<dyn Future<Output = T> + Send + 'static>>
    where
        F: FnOnce(&Self) -> T + Send + 'static,
        T: Send + 'static;

    fn current_running_time(&self) -> crate::ClockTime;
    fn current_clock_time(&self) -> crate::ClockTime;
}

impl<O: IsA<Element>> ElementExtManual for O {
    fn element_class(&self) -> &glib::Class<Element> {
        unsafe {
            let klass = (*(self.as_ptr() as *mut glib::gobject_ffi::GTypeInstance)).g_class
                as *const glib::Class<Element>;
            &*klass
        }
    }

    fn change_state(
        &self,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        let ret: StateChangeReturn = unsafe {
            from_glib(ffi::gst_element_change_state(
                self.as_ref().to_glib_none().0,
                transition.to_glib(),
            ))
        };
        ret.into_result()
    }

    fn continue_state(
        &self,
        ret: StateChangeReturn,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        let ret: StateChangeReturn = unsafe {
            from_glib(ffi::gst_element_continue_state(
                self.as_ref().to_glib_none().0,
                ret.to_glib(),
            ))
        };
        ret.into_result()
    }

    fn state(
        &self,
        timeout: ClockTime,
    ) -> (Result<StateChangeSuccess, StateChangeError>, State, State) {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();
            let mut pending = mem::MaybeUninit::uninit();
            let ret: StateChangeReturn = from_glib(ffi::gst_element_get_state(
                self.as_ref().to_glib_none().0,
                state.as_mut_ptr(),
                pending.as_mut_ptr(),
                timeout.to_glib(),
            ));
            (
                ret.into_result(),
                from_glib(state.assume_init()),
                from_glib(pending.assume_init()),
            )
        }
    }

    fn set_state(&self, state: State) -> Result<StateChangeSuccess, StateChangeError> {
        let ret: StateChangeReturn = unsafe {
            from_glib(ffi::gst_element_set_state(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
            ))
        };
        ret.into_result()
    }

    fn query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn send_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(ffi::gst_element_send_event(
                self.as_ref().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn metadata<'a>(&self, key: &str) -> Option<&'a str> {
        self.element_class().metadata(key)
    }

    fn pad_template(&self, name: &str) -> Option<PadTemplate> {
        self.element_class().pad_template(name)
    }

    fn pad_template_list(&self) -> Vec<PadTemplate> {
        self.element_class().pad_template_list()
    }

    fn set_element_flags(&self, flags: ElementFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.to_glib();
        }
    }

    fn unset_element_flags(&self, flags: ElementFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.to_glib();
        }
    }

    fn element_flags(&self) -> ElementFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }

    fn message_full<T: crate::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    ) {
        unsafe {
            let type_ = match type_ {
                ElementMessageType::Error => ffi::GST_MESSAGE_ERROR,
                ElementMessageType::Warning => ffi::GST_MESSAGE_WARNING,
                ElementMessageType::Info => ffi::GST_MESSAGE_INFO,
            };

            ffi::gst_element_message_full(
                self.as_ref().to_glib_none().0,
                type_,
                T::domain().to_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn message_full_with_details<T: crate::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
        structure: crate::Structure,
    ) {
        unsafe {
            let type_ = match type_ {
                ElementMessageType::Error => ffi::GST_MESSAGE_ERROR,
                ElementMessageType::Warning => ffi::GST_MESSAGE_WARNING,
                ElementMessageType::Info => ffi::GST_MESSAGE_INFO,
            };

            ffi::gst_element_message_full_with_details(
                self.as_ref().to_glib_none().0,
                type_,
                T::domain().to_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
                structure.into_ptr(),
            );
        }
    }

    fn post_message(&self, message: crate::Message) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_post_message(self.as_ref().to_glib_none().0, message.into_ptr()),
                "Failed to post message"
            )
        }
    }

    fn post_error_message(&self, msg: crate::ErrorMessage) {
        let crate::ErrorMessage {
            error_domain,
            error_code,
            ref message,
            ref debug,
            filename,
            function,
            line,
        } = msg;

        unsafe {
            ffi::gst_element_message_full(
                self.as_ref().to_glib_none().0,
                ffi::GST_MESSAGE_ERROR,
                error_domain.to_glib(),
                error_code,
                message.to_glib_full(),
                debug.to_glib_full(),
                filename.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            );
        }
    }

    fn iterate_pads(&self) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_sink_pads(&self) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_sink_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_src_pads(&self) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_src_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.pads)
        }
    }

    fn sink_pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.sinkpads)
        }
    }

    fn src_pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.srcpads)
        }
    }

    fn num_pads(&self) -> u16 {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&elt.object.lock);
            elt.numpads
        }
    }

    fn num_sink_pads(&self) -> u16 {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&elt.object.lock);
            elt.numsinkpads
        }
    }

    fn num_src_pads(&self) -> u16 {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&elt.object.lock);
            elt.numsrcpads
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn add_property_deep_notify_watch(
        &self,
        property_name: Option<&str>,
        include_value: bool,
    ) -> NotifyWatchId {
        let property_name = property_name.to_glib_none();
        unsafe {
            from_glib(ffi::gst_element_add_property_deep_notify_watch(
                self.as_ref().to_glib_none().0,
                property_name.0,
                include_value.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn add_property_notify_watch(
        &self,
        property_name: Option<&str>,
        include_value: bool,
    ) -> NotifyWatchId {
        let property_name = property_name.to_glib_none();
        unsafe {
            from_glib(ffi::gst_element_add_property_notify_watch(
                self.as_ref().to_glib_none().0,
                property_name.0,
                include_value.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn remove_property_notify_watch(&self, watch_id: NotifyWatchId) {
        unsafe {
            ffi::gst_element_remove_property_notify_watch(
                self.as_ref().to_glib_none().0,
                watch_id.to_glib(),
            );
        }
    }

    fn query_convert<V: Into<GenericFormattedValue>, U: SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().to_glib(),
                src_val.to_raw_value(),
                U::default_format().to_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(U::from_raw(U::default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_convert_generic<V: Into<GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<GenericFormattedValue> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().to_glib(),
                src_val.value(),
                dest_format.to_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    fn query_duration<T: SpecificFormattedValue>(&self) -> Option<T> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_duration(
                self.as_ref().to_glib_none().0,
                T::default_format().to_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(T::from_raw(T::default_format(), duration.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_duration(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, duration.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_position<T: SpecificFormattedValue>(&self) -> Option<T> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_position(
                self.as_ref().to_glib_none().0,
                T::default_format().to_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(T::from_raw(T::default_format(), cur.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_position(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, cur.assume_init()))
            } else {
                None
            }
        }
    }

    fn seek<V: Into<GenericFormattedValue>>(
        &self,
        rate: f64,
        flags: crate::SeekFlags,
        start_type: crate::SeekType,
        start: V,
        stop_type: crate::SeekType,
        stop: V,
    ) -> Result<(), glib::error::BoolError> {
        let start = start.into();
        let stop = stop.into();

        assert_eq!(stop.format(), start.format());

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_seek(
                    self.as_ref().to_glib_none().0,
                    rate,
                    start.format().to_glib(),
                    flags.to_glib(),
                    start_type.to_glib(),
                    start.value(),
                    stop_type.to_glib(),
                    stop.value(),
                ),
                "Failed to seek",
            )
        }
    }

    fn seek_simple<V: Into<GenericFormattedValue>>(
        &self,
        seek_flags: crate::SeekFlags,
        seek_pos: V,
    ) -> Result<(), glib::error::BoolError> {
        let seek_pos = seek_pos.into();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_seek_simple(
                    self.as_ref().to_glib_none().0,
                    seek_pos.format().to_glib(),
                    seek_flags.to_glib(),
                    seek_pos.value(),
                ),
                "Failed to seek",
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn call_async<F>(&self, func: F)
    where
        F: FnOnce(&Self) + Send + 'static,
    {
        let user_data: Box<Option<F>> = Box::new(Some(func));

        unsafe extern "C" fn trampoline<O: IsA<Element>, F: FnOnce(&O) + Send + 'static>(
            element: *mut ffi::GstElement,
            user_data: glib::ffi::gpointer,
        ) {
            let user_data: &mut Option<F> = &mut *(user_data as *mut _);
            let callback = user_data.take().unwrap();

            callback(&Element::from_glib_borrow(element).unsafe_cast_ref());
        }

        unsafe extern "C" fn free_user_data<O: IsA<Element>, F: FnOnce(&O) + Send + 'static>(
            user_data: glib::ffi::gpointer,
        ) {
            let _: Box<Option<F>> = Box::from_raw(user_data as *mut _);
        }

        unsafe {
            ffi::gst_element_call_async(
                self.as_ref().to_glib_none().0,
                Some(trampoline::<Self, F>),
                Box::into_raw(user_data) as *mut _,
                Some(free_user_data::<Self, F>),
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn call_async_future<F, T>(&self, func: F) -> Pin<Box<dyn Future<Output = T> + Send + 'static>>
    where
        F: FnOnce(&Self) -> T + Send + 'static,
        T: Send + 'static,
    {
        use futures_channel::oneshot;

        let (sender, receiver) = oneshot::channel();

        self.call_async(move |element| {
            let _ = sender.send(func(element));
        });

        Box::pin(async move { receiver.await.expect("sender dropped") })
    }

    fn current_running_time(&self) -> crate::ClockTime {
        let base_time = self.base_time();
        let clock_time = self.current_clock_time();

        clock_time - base_time
    }

    fn current_clock_time(&self) -> crate::ClockTime {
        if let Some(clock) = self.clock() {
            clock.time()
        } else {
            crate::CLOCK_TIME_NONE
        }
    }
}

pub unsafe trait ElementClassExt {
    fn metadata<'a>(&self, key: &str) -> Option<&'a str> {
        unsafe {
            let klass = self as *const _ as *const ffi::GstElementClass;

            let ptr = ffi::gst_element_class_get_metadata(klass as *mut _, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    fn pad_template(&self, name: &str) -> Option<PadTemplate> {
        unsafe {
            let klass = self as *const _ as *const ffi::GstElementClass;

            from_glib_none(ffi::gst_element_class_get_pad_template(
                klass as *mut _,
                name.to_glib_none().0,
            ))
        }
    }

    fn pad_template_list(&self) -> Vec<PadTemplate> {
        unsafe {
            let klass = self as *const _ as *const ffi::GstElementClass;

            FromGlibPtrContainer::from_glib_none(ffi::gst_element_class_get_pad_template_list(
                klass as *mut _,
            ))
        }
    }
}

unsafe impl<T: IsA<Element> + glib::object::IsClass> ElementClassExt for glib::object::Class<T> {}

pub static ELEMENT_METADATA_AUTHOR: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_ELEMENT_METADATA_AUTHOR)
        .to_str()
        .unwrap()
});
pub static ELEMENT_METADATA_DESCRIPTION: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_ELEMENT_METADATA_DESCRIPTION)
        .to_str()
        .unwrap()
});
pub static ELEMENT_METADATA_DOC_URI: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_ELEMENT_METADATA_DOC_URI)
        .to_str()
        .unwrap()
});
pub static ELEMENT_METADATA_ICON_NAME: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_ELEMENT_METADATA_ICON_NAME)
        .to_str()
        .unwrap()
});
pub static ELEMENT_METADATA_KLASS: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_ELEMENT_METADATA_KLASS)
        .to_str()
        .unwrap()
});
pub static ELEMENT_METADATA_LONGNAME: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_ELEMENT_METADATA_LONGNAME)
        .to_str()
        .unwrap()
});

#[macro_export]
macro_rules! element_error(
    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
);

#[macro_export]
macro_rules! element_warning(
    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
);

#[macro_export]
macro_rules! element_info(
    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
);

#[cfg(test)]
mod tests {
    use super::*;
    use glib::GString;
    #[cfg(feature = "v1_10")]
    use std::sync::mpsc::channel;

    #[test]
    fn test_get_pads() {
        crate::init().unwrap();

        let identity = crate::ElementFactory::make("identity", None).unwrap();

        let mut pad_names = identity
            .pads()
            .iter()
            .map(|p| p.name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink"), String::from("src")]);

        let mut pad_names = identity
            .sink_pads()
            .iter()
            .map(|p| p.name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink")]);

        let mut pad_names = identity
            .src_pads()
            .iter()
            .map(|p| p.name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("src")]);
    }

    #[test]
    #[cfg(feature = "v1_14")]
    fn test_foreach_pad() {
        crate::init().unwrap();

        let identity = crate::ElementFactory::make("identity", None).unwrap();

        let mut pad_names = Vec::new();
        identity.foreach_pad(|_element, pad| {
            pad_names.push(pad.name());

            true
        });
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink"), String::from("src")]);
    }

    #[cfg(feature = "v1_10")]
    #[test]
    fn test_call_async() {
        crate::init().unwrap();

        let identity = crate::ElementFactory::make("identity", None).unwrap();
        let (sender, receiver) = channel();

        identity.call_async(move |_| {
            sender.send(()).unwrap();
        });

        assert_eq!(receiver.recv(), Ok(()));
    }
}
