// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::IconChooserDialog;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

#[cfg(any(feature = "gtk_v3_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_4")))]
glib::wrapper! {
    #[doc(alias = "XAppIconChooserButton")]
    pub struct IconChooserButton(Object<ffi::XAppIconChooserButton, ffi::XAppIconChooserButtonClass>) @extends gtk::Button, gtk::Bin, gtk::Container, gtk::Widget, @implements gtk::Buildable, gtk::Actionable;

    match fn {
        type_ => || ffi::xapp_icon_chooser_button_get_type(),
    }
}

#[cfg(not(any(feature = "gtk_v3_4", feature = "dox")))]
glib::wrapper! {
    #[doc(alias = "XAppIconChooserButton")]
    pub struct IconChooserButton(Object<ffi::XAppIconChooserButton, ffi::XAppIconChooserButtonClass>) @extends gtk::Button, gtk::Bin, gtk::Container, gtk::Widget, @implements gtk::Buildable;

    match fn {
        type_ => || ffi::xapp_icon_chooser_button_get_type(),
    }
}

impl IconChooserButton {
    #[doc(alias = "xapp_icon_chooser_button_new")]
    pub fn new() -> IconChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::xapp_icon_chooser_button_new())
        }
    }

    #[doc(alias = "xapp_icon_chooser_button_new_with_size")]
    #[doc(alias = "new_with_size")]
    pub fn with_size(icon_size: gtk::IconSize) -> IconChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::xapp_icon_chooser_button_new_with_size(icon_size.into_glib()))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`IconChooserButton`] objects.
            ///
            /// This method returns an instance of [`IconChooserButtonBuilder`](crate::builders::IconChooserButtonBuilder) which can be used to create [`IconChooserButton`] objects.
            pub fn builder() -> IconChooserButtonBuilder {
                IconChooserButtonBuilder::default()
            }
        

    #[doc(alias = "xapp_icon_chooser_button_get_dialog")]
    #[doc(alias = "get_dialog")]
    pub fn dialog(&self) -> Option<IconChooserDialog> {
        unsafe {
            from_glib_none(ffi::xapp_icon_chooser_button_get_dialog(self.to_glib_none().0))
        }
    }

    #[doc(alias = "xapp_icon_chooser_button_get_icon")]
    #[doc(alias = "get_icon")]
    pub fn icon(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::xapp_icon_chooser_button_get_icon(self.to_glib_none().0))
        }
    }

    #[doc(alias = "xapp_icon_chooser_button_set_default_category")]
    pub fn set_default_category(&self, category: Option<&str>) {
        unsafe {
            ffi::xapp_icon_chooser_button_set_default_category(self.to_glib_none().0, category.to_glib_none().0);
        }
    }

    #[doc(alias = "xapp_icon_chooser_button_set_icon")]
    pub fn set_icon(&self, icon: Option<&str>) {
        unsafe {
            ffi::xapp_icon_chooser_button_set_icon(self.to_glib_none().0, icon.to_glib_none().0);
        }
    }

    #[doc(alias = "xapp_icon_chooser_button_set_icon_size")]
    pub fn set_icon_size(&self, icon_size: gtk::IconSize) {
        unsafe {
            ffi::xapp_icon_chooser_button_set_icon_size(self.to_glib_none().0, icon_size.into_glib());
        }
    }

    pub fn category(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "category")
    }

    pub fn set_category(&self, category: Option<&str>) {
        glib::ObjectExt::set_property(self,"category", &category)
    }

    #[doc(alias = "icon-size")]
    pub fn icon_size(&self) -> gtk::IconSize {
        glib::ObjectExt::property(self, "icon-size")
    }

    #[doc(alias = "category")]
    pub fn connect_category_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_category_trampoline<F: Fn(&IconChooserButton) + 'static>(this: *mut ffi::XAppIconChooserButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::category\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_category_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon")]
    pub fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<F: Fn(&IconChooserButton) + 'static>(this: *mut ffi::XAppIconChooserButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_icon_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon-size")]
    pub fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<F: Fn(&IconChooserButton) + 'static>(this: *mut ffi::XAppIconChooserButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_icon_size_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for IconChooserButton {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`IconChooserButton`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct IconChooserButtonBuilder {
    category: Option<String>,
    icon: Option<String>,
    icon_size: Option<gtk::IconSize>,
    #[cfg(any(feature = "gtk_v3_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_6")))]
    always_show_image: Option<bool>,
    #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_6")))]
    image: Option<gtk::Widget>,
    //image-position: /*Unknown type*/,
    label: Option<String>,
    //relief: /*Unknown type*/,
    #[cfg_attr(feature = "v3_10", deprecated = "Since 3.10")]
    use_stock: Option<bool>,
    use_underline: Option<bool>,
    #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_4")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    xalign: Option<f32>,
    #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_4")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    yalign: Option<f32>,
    border_width: Option<u32>,
    child: Option<gtk::Widget>,
    //resize-mode: /*Unknown type*/,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    #[cfg(any(feature = "gtk_v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_18")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    double_buffered: Option<bool>,
    //events: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    expand: Option<bool>,
    #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_20")))]
    focus_on_click: Option<bool>,
    //halign: /*Unknown type*/,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    hexpand: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin_bottom: Option<i32>,
    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    margin_end: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    margin_left: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    margin_right: Option<i32>,
    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    margin_start: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_8")))]
    opacity: Option<f64>,
    parent: Option<gtk::Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    tooltip_markup: Option<String>,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    tooltip_text: Option<String>,
    //valign: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    vexpand: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    action_name: Option<String>,
    //action-target: /*Unknown type*/,
}

impl IconChooserButtonBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`IconChooserButtonBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`IconChooserButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> IconChooserButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
if let Some(ref category) = self.category {
                properties.push(("category", category));
            }
if let Some(ref icon) = self.icon {
                properties.push(("icon", icon));
            }
if let Some(ref icon_size) = self.icon_size {
                properties.push(("icon-size", icon_size));
            }
        #[cfg(any(feature = "gtk_v3_6", feature = "dox"))]
if let Some(ref always_show_image) = self.always_show_image {
                properties.push(("always-show-image", always_show_image));
            }
        #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
if let Some(ref image) = self.image {
                properties.push(("image", image));
            }
if let Some(ref label) = self.label {
                properties.push(("label", label));
            }
if let Some(ref use_stock) = self.use_stock {
                properties.push(("use-stock", use_stock));
            }
if let Some(ref use_underline) = self.use_underline {
                properties.push(("use-underline", use_underline));
            }
        #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
if let Some(ref xalign) = self.xalign {
                properties.push(("xalign", xalign));
            }
        #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
if let Some(ref yalign) = self.yalign {
                properties.push(("yalign", yalign));
            }
if let Some(ref border_width) = self.border_width {
                properties.push(("border-width", border_width));
            }
if let Some(ref child) = self.child {
                properties.push(("child", child));
            }
if let Some(ref app_paintable) = self.app_paintable {
                properties.push(("app-paintable", app_paintable));
            }
if let Some(ref can_default) = self.can_default {
                properties.push(("can-default", can_default));
            }
if let Some(ref can_focus) = self.can_focus {
                properties.push(("can-focus", can_focus));
            }
        #[cfg(any(feature = "gtk_v2_18", feature = "dox"))]
if let Some(ref double_buffered) = self.double_buffered {
                properties.push(("double-buffered", double_buffered));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref expand) = self.expand {
                properties.push(("expand", expand));
            }
        #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
if let Some(ref has_default) = self.has_default {
                properties.push(("has-default", has_default));
            }
if let Some(ref has_focus) = self.has_focus {
                properties.push(("has-focus", has_focus));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref has_tooltip) = self.has_tooltip {
                properties.push(("has-tooltip", has_tooltip));
            }
if let Some(ref height_request) = self.height_request {
                properties.push(("height-request", height_request));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref hexpand) = self.hexpand {
                properties.push(("hexpand", hexpand));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref hexpand_set) = self.hexpand_set {
                properties.push(("hexpand-set", hexpand_set));
            }
if let Some(ref is_focus) = self.is_focus {
                properties.push(("is-focus", is_focus));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin) = self.margin {
                properties.push(("margin", margin));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_bottom) = self.margin_bottom {
                properties.push(("margin-bottom", margin_bottom));
            }
        #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
if let Some(ref margin_end) = self.margin_end {
                properties.push(("margin-end", margin_end));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_left) = self.margin_left {
                properties.push(("margin-left", margin_left));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_right) = self.margin_right {
                properties.push(("margin-right", margin_right));
            }
        #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
if let Some(ref margin_start) = self.margin_start {
                properties.push(("margin-start", margin_start));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_top) = self.margin_top {
                properties.push(("margin-top", margin_top));
            }
if let Some(ref name) = self.name {
                properties.push(("name", name));
            }
if let Some(ref no_show_all) = self.no_show_all {
                properties.push(("no-show-all", no_show_all));
            }
        #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
if let Some(ref opacity) = self.opacity {
                properties.push(("opacity", opacity));
            }
if let Some(ref parent) = self.parent {
                properties.push(("parent", parent));
            }
if let Some(ref receives_default) = self.receives_default {
                properties.push(("receives-default", receives_default));
            }
if let Some(ref sensitive) = self.sensitive {
                properties.push(("sensitive", sensitive));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref tooltip_markup) = self.tooltip_markup {
                properties.push(("tooltip-markup", tooltip_markup));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref tooltip_text) = self.tooltip_text {
                properties.push(("tooltip-text", tooltip_text));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref vexpand) = self.vexpand {
                properties.push(("vexpand", vexpand));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref vexpand_set) = self.vexpand_set {
                properties.push(("vexpand-set", vexpand_set));
            }
if let Some(ref visible) = self.visible {
                properties.push(("visible", visible));
            }
if let Some(ref width_request) = self.width_request {
                properties.push(("width-request", width_request));
            }
if let Some(ref action_name) = self.action_name {
                properties.push(("action-name", action_name));
            }
        glib::Object::new::<IconChooserButton>(&properties)

    }

    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn icon_size(mut self, icon_size: gtk::IconSize) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    #[cfg(any(feature = "gtk_v3_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_6")))]
    pub fn always_show_image(mut self, always_show_image: bool) -> Self {
        self.always_show_image = Some(always_show_image);
        self
    }

    #[cfg(any(feature = "gtk_v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_6")))]
    pub fn image(mut self, image: &impl IsA<gtk::Widget>) -> Self {
        self.image = Some(image.clone().upcast());
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    #[cfg_attr(feature = "v3_10", deprecated = "Since 3.10")]
    pub fn use_stock(mut self, use_stock: bool) -> Self {
        self.use_stock = Some(use_stock);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_4")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    #[cfg(any(feature = "gtk_v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_4")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &impl IsA<gtk::Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    #[cfg(any(feature = "gtk_v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_18")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn double_buffered(mut self, double_buffered: bool) -> Self {
        self.double_buffered = Some(double_buffered);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_left(mut self, margin_left: i32) -> Self {
        self.margin_left = Some(margin_left);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_right(mut self, margin_right: i32) -> Self {
        self.margin_right = Some(margin_right);
        self
    }

    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_8")))]
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &impl IsA<gtk::Container>) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }
}

impl fmt::Display for IconChooserButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IconChooserButton")
    }
}
