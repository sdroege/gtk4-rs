// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Bin;
use Buildable;
use Container;
use LayoutManager;
use Overflow;
use PopoverConstraint;
use PositionType;
use Widget;

glib_wrapper! {
    pub struct Popover(Object<gtk_sys::GtkPopover, gtk_sys::GtkPopoverClass, PopoverClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_popover_get_type(),
    }
}

impl Popover {
    pub fn new<P: IsA<Widget>>(relative_to: Option<&P>) -> Popover {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_popover_new(
                relative_to.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn new_from_model<P: IsA<Widget>, Q: IsA<gio::MenuModel>>(
        relative_to: Option<&P>,
        model: &Q,
    ) -> Popover {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_popover_new_from_model(
                relative_to.map(|p| p.as_ref()).to_glib_none().0,
                model.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

pub struct PopoverBuilder {
    constrain_to: Option<PopoverConstraint>,
    default_widget: Option<Widget>,
    modal: Option<bool>,
    pointing_to: Option<gdk::Rectangle>,
    position: Option<PositionType>,
    relative_to: Option<Widget>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl PopoverBuilder {
    pub fn new() -> Self {
        Self {
            constrain_to: None,
            default_widget: None,
            modal: None,
            pointing_to: None,
            position: None,
            relative_to: None,
            can_focus: None,
            can_target: None,
            css_name: None,
            cursor: None,
            expand: None,
            focus_on_click: None,
            halign: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            layout_manager: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            opacity: None,
            overflow: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
        }
    }

    pub fn build(self) -> Popover {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref constrain_to) = self.constrain_to {
            properties.push(("constrain-to", constrain_to));
        }
        if let Some(ref default_widget) = self.default_widget {
            properties.push(("default-widget", default_widget));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref pointing_to) = self.pointing_to {
            properties.push(("pointing-to", pointing_to));
        }
        if let Some(ref position) = self.position {
            properties.push(("position", position));
        }
        if let Some(ref relative_to) = self.relative_to {
            properties.push(("relative-to", relative_to));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(Popover::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn constrain_to(mut self, constrain_to: PopoverConstraint) -> Self {
        self.constrain_to = Some(constrain_to);
        self
    }

    pub fn default_widget(mut self, default_widget: &Widget) -> Self {
        self.default_widget = Some(default_widget.clone());
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn pointing_to(mut self, pointing_to: &gdk::Rectangle) -> Self {
        self.pointing_to = Some(pointing_to.clone());
        self
    }

    pub fn position(mut self, position: PositionType) -> Self {
        self.position = Some(position);
        self
    }

    pub fn relative_to(mut self, relative_to: &Widget) -> Self {
        self.relative_to = Some(relative_to.clone());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &LayoutManager) -> Self {
        self.layout_manager = Some(layout_manager.clone());
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
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

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

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
}

pub const NONE_POPOVER: Option<&Popover> = None;

pub trait PopoverExt: 'static {
    fn bind_model<P: IsA<gio::MenuModel>>(&self, model: Option<&P>, action_namespace: Option<&str>);

    fn get_constrain_to(&self) -> PopoverConstraint;

    fn get_default_widget(&self) -> Option<Widget>;

    fn get_modal(&self) -> bool;

    fn get_pointing_to(&self) -> Option<gdk::Rectangle>;

    fn get_position(&self) -> PositionType;

    fn get_relative_to(&self) -> Option<Widget>;

    fn popdown(&self);

    fn popup(&self);

    fn set_constrain_to(&self, constraint: PopoverConstraint);

    fn set_default_widget<P: IsA<Widget>>(&self, widget: Option<&P>);

    fn set_modal(&self, modal: bool);

    fn set_pointing_to(&self, rect: &gdk::Rectangle);

    fn set_position(&self, position: PositionType);

    fn set_relative_to<P: IsA<Widget>>(&self, relative_to: Option<&P>);

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_constrain_to_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_default_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_relative_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Popover>> PopoverExt for O {
    fn bind_model<P: IsA<gio::MenuModel>>(
        &self,
        model: Option<&P>,
        action_namespace: Option<&str>,
    ) {
        unsafe {
            gtk_sys::gtk_popover_bind_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                action_namespace.to_glib_none().0,
            );
        }
    }

    fn get_constrain_to(&self) -> PopoverConstraint {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_constrain_to(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_default_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_popover_get_default_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_modal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pointing_to(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(gtk_sys::gtk_popover_get_pointing_to(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
            ));
            if ret {
                Some(rect)
            } else {
                None
            }
        }
    }

    fn get_position(&self) -> PositionType {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_position(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_relative_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_popover_get_relative_to(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn popdown(&self) {
        unsafe {
            gtk_sys::gtk_popover_popdown(self.as_ref().to_glib_none().0);
        }
    }

    fn popup(&self) {
        unsafe {
            gtk_sys::gtk_popover_popup(self.as_ref().to_glib_none().0);
        }
    }

    fn set_constrain_to(&self, constraint: PopoverConstraint) {
        unsafe {
            gtk_sys::gtk_popover_set_constrain_to(
                self.as_ref().to_glib_none().0,
                constraint.to_glib(),
            );
        }
    }

    fn set_default_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_popover_set_default_widget(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            gtk_sys::gtk_popover_set_modal(self.as_ref().to_glib_none().0, modal.to_glib());
        }
    }

    fn set_pointing_to(&self, rect: &gdk::Rectangle) {
        unsafe {
            gtk_sys::gtk_popover_set_pointing_to(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none().0,
            );
        }
    }

    fn set_position(&self, position: PositionType) {
        unsafe {
            gtk_sys::gtk_popover_set_position(self.as_ref().to_glib_none().0, position.to_glib());
        }
    }

    fn set_relative_to<P: IsA<Widget>>(&self, relative_to: Option<&P>) {
        unsafe {
            gtk_sys::gtk_popover_set_relative_to(
                self.as_ref().to_glib_none().0,
                relative_to.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(transmute(closed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_constrain_to_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_constrain_to_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::constrain-to\0".as_ptr() as *const _,
                Some(transmute(
                    notify_constrain_to_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_default_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-widget\0".as_ptr() as *const _,
                Some(transmute(
                    notify_default_widget_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute(notify_modal_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pointing_to_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pointing-to\0".as_ptr() as *const _,
                Some(transmute(notify_pointing_to_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute(notify_position_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_relative_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_relative_to_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::relative-to\0".as_ptr() as *const _,
                Some(transmute(notify_relative_to_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Popover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Popover")
    }
}
