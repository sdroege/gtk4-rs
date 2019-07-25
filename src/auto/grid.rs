// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
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
use std::mem;
use std::mem::transmute;
use Align;
use BaselinePosition;
use Buildable;
use Container;
use LayoutManager;
use Orientable;
use Overflow;
use PositionType;
use Widget;

glib_wrapper! {
    pub struct Grid(Object<gtk_sys::GtkGrid, gtk_sys::GtkGridClass, GridClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_grid_get_type(),
    }
}

impl Grid {
    pub fn new() -> Grid {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_grid_new()).unsafe_cast() }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GridBuilder {
    baseline_row: Option<i32>,
    column_homogeneous: Option<bool>,
    column_spacing: Option<i32>,
    row_homogeneous: Option<bool>,
    row_spacing: Option<i32>,
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

impl GridBuilder {
    pub fn new() -> Self {
        Self {
            baseline_row: None,
            column_homogeneous: None,
            column_spacing: None,
            row_homogeneous: None,
            row_spacing: None,
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

    pub fn build(self) -> Grid {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref baseline_row) = self.baseline_row {
            properties.push(("baseline-row", baseline_row));
        }
        if let Some(ref column_homogeneous) = self.column_homogeneous {
            properties.push(("column-homogeneous", column_homogeneous));
        }
        if let Some(ref column_spacing) = self.column_spacing {
            properties.push(("column-spacing", column_spacing));
        }
        if let Some(ref row_homogeneous) = self.row_homogeneous {
            properties.push(("row-homogeneous", row_homogeneous));
        }
        if let Some(ref row_spacing) = self.row_spacing {
            properties.push(("row-spacing", row_spacing));
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
        glib::Object::new(Grid::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn baseline_row(mut self, baseline_row: i32) -> Self {
        self.baseline_row = Some(baseline_row);
        self
    }

    pub fn column_homogeneous(mut self, column_homogeneous: bool) -> Self {
        self.column_homogeneous = Some(column_homogeneous);
        self
    }

    pub fn column_spacing(mut self, column_spacing: i32) -> Self {
        self.column_spacing = Some(column_spacing);
        self
    }

    pub fn row_homogeneous(mut self, row_homogeneous: bool) -> Self {
        self.row_homogeneous = Some(row_homogeneous);
        self
    }

    pub fn row_spacing(mut self, row_spacing: i32) -> Self {
        self.row_spacing = Some(row_spacing);
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

pub const NONE_GRID: Option<&Grid> = None;

pub trait GridExt: 'static {
    fn attach<P: IsA<Widget>>(&self, child: &P, left: i32, top: i32, width: i32, height: i32);

    fn attach_next_to<P: IsA<Widget>, Q: IsA<Widget>>(
        &self,
        child: &P,
        sibling: Option<&Q>,
        side: PositionType,
        width: i32,
        height: i32,
    );

    fn get_baseline_row(&self) -> i32;

    fn get_child_at(&self, left: i32, top: i32) -> Option<Widget>;

    fn get_column_homogeneous(&self) -> bool;

    fn get_column_spacing(&self) -> u32;

    fn get_row_baseline_position(&self, row: i32) -> BaselinePosition;

    fn get_row_homogeneous(&self) -> bool;

    fn get_row_spacing(&self) -> u32;

    fn insert_column(&self, position: i32);

    fn insert_next_to<P: IsA<Widget>>(&self, sibling: &P, side: PositionType);

    fn insert_row(&self, position: i32);

    fn query_child<P: IsA<Widget>>(&self, child: &P) -> (i32, i32, i32, i32);

    fn remove_column(&self, position: i32);

    fn remove_row(&self, position: i32);

    fn set_baseline_row(&self, row: i32);

    fn set_column_homogeneous(&self, homogeneous: bool);

    fn set_column_spacing(&self, spacing: u32);

    fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition);

    fn set_row_homogeneous(&self, homogeneous: bool);

    fn set_row_spacing(&self, spacing: u32);

    fn connect_property_baseline_row_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_column_homogeneous_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_row_homogeneous_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Grid>> GridExt for O {
    fn attach<P: IsA<Widget>>(&self, child: &P, left: i32, top: i32, width: i32, height: i32) {
        unsafe {
            gtk_sys::gtk_grid_attach(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                left,
                top,
                width,
                height,
            );
        }
    }

    fn attach_next_to<P: IsA<Widget>, Q: IsA<Widget>>(
        &self,
        child: &P,
        sibling: Option<&Q>,
        side: PositionType,
        width: i32,
        height: i32,
    ) {
        unsafe {
            gtk_sys::gtk_grid_attach_next_to(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                sibling.map(|p| p.as_ref()).to_glib_none().0,
                side.to_glib(),
                width,
                height,
            );
        }
    }

    fn get_baseline_row(&self) -> i32 {
        unsafe { gtk_sys::gtk_grid_get_baseline_row(self.as_ref().to_glib_none().0) }
    }

    fn get_child_at(&self, left: i32, top: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_grid_get_child_at(
                self.as_ref().to_glib_none().0,
                left,
                top,
            ))
        }
    }

    fn get_column_homogeneous(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_grid_get_column_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_column_spacing(&self) -> u32 {
        unsafe { gtk_sys::gtk_grid_get_column_spacing(self.as_ref().to_glib_none().0) }
    }

    fn get_row_baseline_position(&self, row: i32) -> BaselinePosition {
        unsafe {
            from_glib(gtk_sys::gtk_grid_get_row_baseline_position(
                self.as_ref().to_glib_none().0,
                row,
            ))
        }
    }

    fn get_row_homogeneous(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_grid_get_row_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_row_spacing(&self) -> u32 {
        unsafe { gtk_sys::gtk_grid_get_row_spacing(self.as_ref().to_glib_none().0) }
    }

    fn insert_column(&self, position: i32) {
        unsafe {
            gtk_sys::gtk_grid_insert_column(self.as_ref().to_glib_none().0, position);
        }
    }

    fn insert_next_to<P: IsA<Widget>>(&self, sibling: &P, side: PositionType) {
        unsafe {
            gtk_sys::gtk_grid_insert_next_to(
                self.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
                side.to_glib(),
            );
        }
    }

    fn insert_row(&self, position: i32) {
        unsafe {
            gtk_sys::gtk_grid_insert_row(self.as_ref().to_glib_none().0, position);
        }
    }

    fn query_child<P: IsA<Widget>>(&self, child: &P) -> (i32, i32, i32, i32) {
        unsafe {
            let mut left = mem::uninitialized();
            let mut top = mem::uninitialized();
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            gtk_sys::gtk_grid_query_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                &mut left,
                &mut top,
                &mut width,
                &mut height,
            );
            (left, top, width, height)
        }
    }

    fn remove_column(&self, position: i32) {
        unsafe {
            gtk_sys::gtk_grid_remove_column(self.as_ref().to_glib_none().0, position);
        }
    }

    fn remove_row(&self, position: i32) {
        unsafe {
            gtk_sys::gtk_grid_remove_row(self.as_ref().to_glib_none().0, position);
        }
    }

    fn set_baseline_row(&self, row: i32) {
        unsafe {
            gtk_sys::gtk_grid_set_baseline_row(self.as_ref().to_glib_none().0, row);
        }
    }

    fn set_column_homogeneous(&self, homogeneous: bool) {
        unsafe {
            gtk_sys::gtk_grid_set_column_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.to_glib(),
            );
        }
    }

    fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            gtk_sys::gtk_grid_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition) {
        unsafe {
            gtk_sys::gtk_grid_set_row_baseline_position(
                self.as_ref().to_glib_none().0,
                row,
                pos.to_glib(),
            );
        }
    }

    fn set_row_homogeneous(&self, homogeneous: bool) {
        unsafe {
            gtk_sys::gtk_grid_set_row_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.to_glib(),
            );
        }
    }

    fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            gtk_sys::gtk_grid_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn connect_property_baseline_row_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_row_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkGrid,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::baseline-row\0".as_ptr() as *const _,
                Some(transmute(
                    notify_baseline_row_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_column_homogeneous_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_homogeneous_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkGrid,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column-homogeneous\0".as_ptr() as *const _,
                Some(transmute(
                    notify_column_homogeneous_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkGrid,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column-spacing\0".as_ptr() as *const _,
                Some(transmute(
                    notify_column_spacing_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_row_homogeneous_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_homogeneous_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkGrid,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row-homogeneous\0".as_ptr() as *const _,
                Some(transmute(
                    notify_row_homogeneous_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkGrid,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Grid>,
        {
            let f: &F = &*(f as *const F);
            f(&Grid::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row-spacing\0".as_ptr() as *const _,
                Some(transmute(notify_row_spacing_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid")
    }
}
