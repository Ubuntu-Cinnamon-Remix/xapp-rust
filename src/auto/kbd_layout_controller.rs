// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "XAppKbdLayoutController")]
    pub struct KbdLayoutController(Object<ffi::XAppKbdLayoutController, ffi::XAppKbdLayoutControllerClass>);

    match fn {
        type_ => || ffi::xapp_kbd_layout_controller_get_type(),
    }
}

impl KbdLayoutController {
        pub const NONE: Option<&'static KbdLayoutController> = None;
    

    #[doc(alias = "xapp_kbd_layout_controller_new")]
    pub fn new() -> KbdLayoutController {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::xapp_kbd_layout_controller_new())
        }
    }
}

impl Default for KbdLayoutController {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait KbdLayoutControllerExt: 'static {
    #[doc(alias = "xapp_kbd_layout_controller_get_all_names")]
    #[doc(alias = "get_all_names")]
    fn all_names(&self) -> Vec<glib::GString>;

    #[doc(alias = "xapp_kbd_layout_controller_get_current_flag_id")]
    #[doc(alias = "get_current_flag_id")]
    fn current_flag_id(&self) -> i32;

    #[doc(alias = "xapp_kbd_layout_controller_get_current_group")]
    #[doc(alias = "get_current_group")]
    fn current_group(&self) -> u32;

    #[doc(alias = "xapp_kbd_layout_controller_get_current_icon_name")]
    #[doc(alias = "get_current_icon_name")]
    fn current_icon_name(&self) -> Option<glib::GString>;

    #[doc(alias = "xapp_kbd_layout_controller_get_current_name")]
    #[doc(alias = "get_current_name")]
    fn current_name(&self) -> Option<glib::GString>;

    #[doc(alias = "xapp_kbd_layout_controller_get_current_short_group_label")]
    #[doc(alias = "get_current_short_group_label")]
    fn current_short_group_label(&self) -> Option<glib::GString>;

    #[doc(alias = "xapp_kbd_layout_controller_get_current_variant_label")]
    #[doc(alias = "get_current_variant_label")]
    fn current_variant_label(&self) -> Option<glib::GString>;

    #[doc(alias = "xapp_kbd_layout_controller_get_enabled")]
    #[doc(alias = "get_enabled")]
    fn is_enabled(&self) -> bool;

    #[doc(alias = "xapp_kbd_layout_controller_get_flag_id_for_group")]
    #[doc(alias = "get_flag_id_for_group")]
    fn flag_id_for_group(&self, group: u32) -> i32;

    #[doc(alias = "xapp_kbd_layout_controller_get_icon_name_for_group")]
    #[doc(alias = "get_icon_name_for_group")]
    fn icon_name_for_group(&self, group: u32) -> Option<glib::GString>;

    #[doc(alias = "xapp_kbd_layout_controller_get_short_group_label_for_group")]
    #[doc(alias = "get_short_group_label_for_group")]
    fn short_group_label_for_group(&self, group: u32) -> Option<glib::GString>;

    #[doc(alias = "xapp_kbd_layout_controller_get_variant_label_for_group")]
    #[doc(alias = "get_variant_label_for_group")]
    fn variant_label_for_group(&self, group: u32) -> Option<glib::GString>;

    #[doc(alias = "xapp_kbd_layout_controller_next_group")]
    fn next_group(&self);

    #[doc(alias = "xapp_kbd_layout_controller_previous_group")]
    fn previous_group(&self);

    #[doc(alias = "xapp_kbd_layout_controller_set_current_group")]
    fn set_current_group(&self, group: u32);

    #[doc(alias = "config-changed")]
    fn connect_config_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "layout-changed")]
    fn connect_layout_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "enabled")]
    fn connect_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<KbdLayoutController>> KbdLayoutControllerExt for O {
    fn all_names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::xapp_kbd_layout_controller_get_all_names(self.as_ref().to_glib_none().0))
        }
    }

    fn current_flag_id(&self) -> i32 {
        unsafe {
            ffi::xapp_kbd_layout_controller_get_current_flag_id(self.as_ref().to_glib_none().0)
        }
    }

    fn current_group(&self) -> u32 {
        unsafe {
            ffi::xapp_kbd_layout_controller_get_current_group(self.as_ref().to_glib_none().0)
        }
    }

    fn current_icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_kbd_layout_controller_get_current_icon_name(self.as_ref().to_glib_none().0))
        }
    }

    fn current_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_kbd_layout_controller_get_current_name(self.as_ref().to_glib_none().0))
        }
    }

    fn current_short_group_label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_kbd_layout_controller_get_current_short_group_label(self.as_ref().to_glib_none().0))
        }
    }

    fn current_variant_label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_kbd_layout_controller_get_current_variant_label(self.as_ref().to_glib_none().0))
        }
    }

    fn is_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::xapp_kbd_layout_controller_get_enabled(self.as_ref().to_glib_none().0))
        }
    }

    fn flag_id_for_group(&self, group: u32) -> i32 {
        unsafe {
            ffi::xapp_kbd_layout_controller_get_flag_id_for_group(self.as_ref().to_glib_none().0, group)
        }
    }

    fn icon_name_for_group(&self, group: u32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_kbd_layout_controller_get_icon_name_for_group(self.as_ref().to_glib_none().0, group))
        }
    }

    fn short_group_label_for_group(&self, group: u32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_kbd_layout_controller_get_short_group_label_for_group(self.as_ref().to_glib_none().0, group))
        }
    }

    fn variant_label_for_group(&self, group: u32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::xapp_kbd_layout_controller_get_variant_label_for_group(self.as_ref().to_glib_none().0, group))
        }
    }

    fn next_group(&self) {
        unsafe {
            ffi::xapp_kbd_layout_controller_next_group(self.as_ref().to_glib_none().0);
        }
    }

    fn previous_group(&self) {
        unsafe {
            ffi::xapp_kbd_layout_controller_previous_group(self.as_ref().to_glib_none().0);
        }
    }

    fn set_current_group(&self, group: u32) {
        unsafe {
            ffi::xapp_kbd_layout_controller_set_current_group(self.as_ref().to_glib_none().0, group);
        }
    }

    fn connect_config_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn config_changed_trampoline<P: IsA<KbdLayoutController>, F: Fn(&P) + 'static>(this: *mut ffi::XAppKbdLayoutController, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(KbdLayoutController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"config-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(config_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_layout_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn layout_changed_trampoline<P: IsA<KbdLayoutController>, F: Fn(&P, u32) + 'static>(this: *mut ffi::XAppKbdLayoutController, object: libc::c_uint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(KbdLayoutController::from_glib_borrow(this).unsafe_cast_ref(), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"layout-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(layout_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<P: IsA<KbdLayoutController>, F: Fn(&P) + 'static>(this: *mut ffi::XAppKbdLayoutController, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(KbdLayoutController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_enabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for KbdLayoutController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("KbdLayoutController")
    }
}
