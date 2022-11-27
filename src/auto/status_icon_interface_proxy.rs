// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::StatusIconInterface;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

#[cfg(any(feature = "gio_v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
glib::wrapper! {
    #[doc(alias = "XAppStatusIconInterfaceProxy")]
    pub struct StatusIconInterfaceProxy(Object<ffi::XAppStatusIconInterfaceProxy, ffi::XAppStatusIconInterfaceProxyClass>) @extends gio::DBusProxy, @implements gio::AsyncInitable, gio::DBusInterface, gio::Initable, StatusIconInterface;

    match fn {
        type_ => || ffi::xapp_status_icon_interface_proxy_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_26", feature = "dox")))]
#[cfg(any(feature = "gio_v2_22", feature = "dox"))]
glib::wrapper! {
    #[doc(alias = "XAppStatusIconInterfaceProxy")]
    pub struct StatusIconInterfaceProxy(Object<ffi::XAppStatusIconInterfaceProxy, ffi::XAppStatusIconInterfaceProxyClass>) @implements gio::AsyncInitable, gio::DBusInterface, gio::Initable, StatusIconInterface;

    match fn {
        type_ => || ffi::xapp_status_icon_interface_proxy_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_22", feature = "dox")))]
#[cfg(any(feature = "gio_v2_30", feature = "dox"))]
glib::wrapper! {
    #[doc(alias = "XAppStatusIconInterfaceProxy")]
    pub struct StatusIconInterfaceProxy(Object<ffi::XAppStatusIconInterfaceProxy, ffi::XAppStatusIconInterfaceProxyClass>) @implements gio::DBusInterface, gio::Initable, StatusIconInterface;

    match fn {
        type_ => || ffi::xapp_status_icon_interface_proxy_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_30", feature = "dox")))]
#[cfg(any(feature = "gio_v2_22", feature = "dox"))]
glib::wrapper! {
    #[doc(alias = "XAppStatusIconInterfaceProxy")]
    pub struct StatusIconInterfaceProxy(Object<ffi::XAppStatusIconInterfaceProxy, ffi::XAppStatusIconInterfaceProxyClass>) @implements gio::Initable, StatusIconInterface;

    match fn {
        type_ => || ffi::xapp_status_icon_interface_proxy_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_22", feature = "dox")))]
glib::wrapper! {
    #[doc(alias = "XAppStatusIconInterfaceProxy")]
    pub struct StatusIconInterfaceProxy(Object<ffi::XAppStatusIconInterfaceProxy, ffi::XAppStatusIconInterfaceProxyClass>) @implements StatusIconInterface;

    match fn {
        type_ => || ffi::xapp_status_icon_interface_proxy_get_type(),
    }
}

impl StatusIconInterfaceProxy {
        pub const NONE: Option<&'static StatusIconInterfaceProxy> = None;
    

    #[doc(alias = "xapp_status_icon_interface_proxy_new_for_bus_sync")]
    #[doc(alias = "new_for_bus_sync")]
    pub fn for_bus_sync(bus_type: gio::BusType, flags: gio::DBusProxyFlags, name: &str, object_path: &str, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<StatusIconInterfaceProxy, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::xapp_status_icon_interface_proxy_new_for_bus_sync(bus_type.into_glib(), flags.into_glib(), name.to_glib_none().0, object_path.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "xapp_status_icon_interface_proxy_new_sync")]
    pub fn new_sync(connection: &gio::DBusConnection, flags: gio::DBusProxyFlags, name: Option<&str>, object_path: &str, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<StatusIconInterfaceProxy, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::xapp_status_icon_interface_proxy_new_sync(connection.to_glib_none().0, flags.into_glib(), name.to_glib_none().0, object_path.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`StatusIconInterfaceProxy`] objects.
            ///
            /// This method returns an instance of [`StatusIconInterfaceProxyBuilder`](crate::builders::StatusIconInterfaceProxyBuilder) which can be used to create [`StatusIconInterfaceProxy`] objects.
            pub fn builder() -> StatusIconInterfaceProxyBuilder {
                StatusIconInterfaceProxyBuilder::default()
            }
        

    #[doc(alias = "xapp_status_icon_interface_proxy_new")]
    pub fn new<P: FnOnce(Result<StatusIconInterfaceProxy, glib::Error>) + 'static>(connection: &gio::DBusConnection, flags: gio::DBusProxyFlags, name: Option<&str>, object_path: &str, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        assert_initialized_main_thread!();
        
                let main_context = glib::MainContext::ref_thread_default();
                let is_main_context_owner = main_context.is_owner();
                let has_acquired_main_context = (!is_main_context_owner)
                    .then(|| main_context.acquire().ok())
                    .flatten();
                assert!(
                    is_main_context_owner || has_acquired_main_context.is_some(),
                    "Async operations only allowed if the thread is owning the MainContext"
                );
        
        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn new_trampoline<P: FnOnce(Result<StatusIconInterfaceProxy, glib::Error>) + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::xapp_status_icon_interface_proxy_new_finish(res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = new_trampoline::<P>;
        unsafe {
            ffi::xapp_status_icon_interface_proxy_new(connection.to_glib_none().0, flags.into_glib(), name.to_glib_none().0, object_path.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    pub fn new_future(connection: &gio::DBusConnection, flags: gio::DBusProxyFlags, name: Option<&str>, object_path: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<StatusIconInterfaceProxy, glib::Error>> + 'static>> {

        skip_assert_initialized!();
        let connection = connection.clone();
        let name = name.map(ToOwned::to_owned);
        let object_path = String::from(object_path);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::new(
                &connection,
                flags,
                name.as_ref().map(::std::borrow::Borrow::borrow),
                &object_path,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "xapp_status_icon_interface_proxy_new_for_bus")]
    pub fn new_for_bus<P: FnOnce(Result<StatusIconInterfaceProxy, glib::Error>) + 'static>(bus_type: gio::BusType, flags: gio::DBusProxyFlags, name: &str, object_path: &str, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        assert_initialized_main_thread!();
        
                let main_context = glib::MainContext::ref_thread_default();
                let is_main_context_owner = main_context.is_owner();
                let has_acquired_main_context = (!is_main_context_owner)
                    .then(|| main_context.acquire().ok())
                    .flatten();
                assert!(
                    is_main_context_owner || has_acquired_main_context.is_some(),
                    "Async operations only allowed if the thread is owning the MainContext"
                );
        
        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn new_for_bus_trampoline<P: FnOnce(Result<StatusIconInterfaceProxy, glib::Error>) + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::xapp_status_icon_interface_proxy_new_for_bus_finish(res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = new_for_bus_trampoline::<P>;
        unsafe {
            ffi::xapp_status_icon_interface_proxy_new_for_bus(bus_type.into_glib(), flags.into_glib(), name.to_glib_none().0, object_path.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    pub fn new_for_bus_future(bus_type: gio::BusType, flags: gio::DBusProxyFlags, name: &str, object_path: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<StatusIconInterfaceProxy, glib::Error>> + 'static>> {

        skip_assert_initialized!();
        let name = String::from(name);
        let object_path = String::from(object_path);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::new_for_bus(
                bus_type,
                flags,
                &name,
                &object_path,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`StatusIconInterfaceProxy`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StatusIconInterfaceProxyBuilder {
    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    g_bus_type: Option<gio::BusType>,
    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    g_connection: Option<gio::DBusConnection>,
    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    g_default_timeout: Option<i32>,
    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    g_flags: Option<gio::DBusProxyFlags>,
    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    g_interface_info: Option<gio::DBusInterfaceInfo>,
    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    g_interface_name: Option<String>,
    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    g_name: Option<String>,
    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    g_object_path: Option<String>,
    icon_name: Option<String>,
    icon_size: Option<i32>,
    label: Option<String>,
    metadata: Option<String>,
    name: Option<String>,
    primary_menu_is_open: Option<bool>,
    secondary_menu_is_open: Option<bool>,
    tooltip_text: Option<String>,
    visible: Option<bool>,
}

impl StatusIconInterfaceProxyBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`StatusIconInterfaceProxyBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`StatusIconInterfaceProxy`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> StatusIconInterfaceProxy {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
if let Some(ref g_bus_type) = self.g_bus_type {
                properties.push(("g-bus-type", g_bus_type));
            }
        #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
if let Some(ref g_connection) = self.g_connection {
                properties.push(("g-connection", g_connection));
            }
        #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
if let Some(ref g_default_timeout) = self.g_default_timeout {
                properties.push(("g-default-timeout", g_default_timeout));
            }
        #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
if let Some(ref g_flags) = self.g_flags {
                properties.push(("g-flags", g_flags));
            }
        #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
if let Some(ref g_interface_info) = self.g_interface_info {
                properties.push(("g-interface-info", g_interface_info));
            }
        #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
if let Some(ref g_interface_name) = self.g_interface_name {
                properties.push(("g-interface-name", g_interface_name));
            }
        #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
if let Some(ref g_name) = self.g_name {
                properties.push(("g-name", g_name));
            }
        #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
if let Some(ref g_object_path) = self.g_object_path {
                properties.push(("g-object-path", g_object_path));
            }
if let Some(ref icon_name) = self.icon_name {
                properties.push(("icon-name", icon_name));
            }
if let Some(ref icon_size) = self.icon_size {
                properties.push(("icon-size", icon_size));
            }
if let Some(ref label) = self.label {
                properties.push(("label", label));
            }
if let Some(ref metadata) = self.metadata {
                properties.push(("metadata", metadata));
            }
if let Some(ref name) = self.name {
                properties.push(("name", name));
            }
if let Some(ref primary_menu_is_open) = self.primary_menu_is_open {
                properties.push(("primary-menu-is-open", primary_menu_is_open));
            }
if let Some(ref secondary_menu_is_open) = self.secondary_menu_is_open {
                properties.push(("secondary-menu-is-open", secondary_menu_is_open));
            }
if let Some(ref tooltip_text) = self.tooltip_text {
                properties.push(("tooltip-text", tooltip_text));
            }
if let Some(ref visible) = self.visible {
                properties.push(("visible", visible));
            }
        glib::Object::new::<StatusIconInterfaceProxy>(&properties)

    }

    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_bus_type(mut self, g_bus_type: gio::BusType) -> Self {
        self.g_bus_type = Some(g_bus_type);
        self
    }

    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_connection(mut self, g_connection: &gio::DBusConnection) -> Self {
        self.g_connection = Some(g_connection.clone());
        self
    }

    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_default_timeout(mut self, g_default_timeout: i32) -> Self {
        self.g_default_timeout = Some(g_default_timeout);
        self
    }

    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_flags(mut self, g_flags: gio::DBusProxyFlags) -> Self {
        self.g_flags = Some(g_flags);
        self
    }

    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_interface_info(mut self, g_interface_info: &gio::DBusInterfaceInfo) -> Self {
        self.g_interface_info = Some(g_interface_info.clone());
        self
    }

    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_interface_name(mut self, g_interface_name: &str) -> Self {
        self.g_interface_name = Some(g_interface_name.to_string());
        self
    }

    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_name(mut self, g_name: &str) -> Self {
        self.g_name = Some(g_name.to_string());
        self
    }

    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_object_path(mut self, g_object_path: &str) -> Self {
        self.g_object_path = Some(g_object_path.to_string());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn icon_size(mut self, icon_size: i32) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn metadata(mut self, metadata: &str) -> Self {
        self.metadata = Some(metadata.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn primary_menu_is_open(mut self, primary_menu_is_open: bool) -> Self {
        self.primary_menu_is_open = Some(primary_menu_is_open);
        self
    }

    pub fn secondary_menu_is_open(mut self, secondary_menu_is_open: bool) -> Self {
        self.secondary_menu_is_open = Some(secondary_menu_is_open);
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }
}

impl fmt::Display for StatusIconInterfaceProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StatusIconInterfaceProxy")
    }
}
