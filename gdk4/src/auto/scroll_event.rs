// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Event;
use crate::ScrollDirection;
use glib::translate::*;
use glib::StaticType;
use std::mem;

glib::wrapper! {
    #[doc(alias = "GdkScrollEvent")]
    pub struct ScrollEvent(Shared<ffi::GdkScrollEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl glib::StaticType for ScrollEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_scroll_event_get_type()) }
    }
}

impl ScrollEvent {
    #[doc(alias = "gdk_scroll_event_get_deltas")]
    #[doc(alias = "get_deltas")]
    pub fn deltas(&self) -> (f64, f64) {
        unsafe {
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            ffi::gdk_scroll_event_get_deltas(
                self.to_glib_none().0,
                delta_x.as_mut_ptr(),
                delta_y.as_mut_ptr(),
            );
            let delta_x = delta_x.assume_init();
            let delta_y = delta_y.assume_init();
            (delta_x, delta_y)
        }
    }

    #[doc(alias = "gdk_scroll_event_get_direction")]
    #[doc(alias = "get_direction")]
    pub fn direction(&self) -> ScrollDirection {
        unsafe { from_glib(ffi::gdk_scroll_event_get_direction(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_scroll_event_is_stop")]
    pub fn is_stop(&self) -> bool {
        unsafe { from_glib(ffi::gdk_scroll_event_is_stop(self.to_glib_none().0)) }
    }
}
