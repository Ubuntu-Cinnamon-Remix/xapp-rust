// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::StatusIconInterface;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "XAppObject")]
    pub struct Object(Interface<ffi::XAppObject, ffi::XAppObjectIface>);

    match fn {
        type_ => || ffi::xapp_object_get_type(),
    }
}

impl Object {
        pub const NONE: Option<&'static Object> = None;
    
}

pub trait ObjectExt: 'static {
    #[doc(alias = "xapp_object_get_status_icon_interface")]
    #[doc(alias = "get_status_icon_interface")]
    fn status_icon_interface(&self) -> Option<StatusIconInterface>;

    #[doc(alias = "xapp_object_peek_status_icon_interface")]
    fn peek_status_icon_interface(&self) -> Option<StatusIconInterface>;

    #[doc(alias = "status-icon-interface")]
    fn set_status_icon_interface<P: IsA<StatusIconInterface>>(&self, status_icon_interface: Option<&P>);

    #[doc(alias = "status-icon-interface")]
    fn connect_status_icon_interface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Object>> ObjectExt for O {
    fn status_icon_interface(&self) -> Option<StatusIconInterface> {
        unsafe {
            from_glib_full(ffi::xapp_object_get_status_icon_interface(self.as_ref().to_glib_none().0))
        }
    }

    fn peek_status_icon_interface(&self) -> Option<StatusIconInterface> {
        unsafe {
            from_glib_none(ffi::xapp_object_peek_status_icon_interface(self.as_ref().to_glib_none().0))
        }
    }

    fn set_status_icon_interface<P: IsA<StatusIconInterface>>(&self, status_icon_interface: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(),"status-icon-interface", &status_icon_interface)
    }

    fn connect_status_icon_interface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_status_icon_interface_trampoline<P: IsA<Object>, F: Fn(&P) + 'static>(this: *mut ffi::XAppObject, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::status-icon-interface\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_status_icon_interface_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Object")
    }
}