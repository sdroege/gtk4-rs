// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Overflow;
use crate::Widget;
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

glib::wrapper! {
    pub struct Calendar(Object<ffi::GtkCalendar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_calendar_get_type(),
    }
}

impl Calendar {
    #[doc(alias = "gtk_calendar_new")]
    pub fn new() -> Calendar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_calendar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Calendar`]
    /// This method returns an instance of [`CalendarBuilder`] which can be used to create a [`Calendar`].
    pub fn builder() -> CalendarBuilder {
        CalendarBuilder::default()
    }

    #[doc(alias = "gtk_calendar_clear_marks")]
    pub fn clear_marks(&self) {
        unsafe {
            ffi::gtk_calendar_clear_marks(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_calendar_get_date")]
    #[doc(alias = "get_date")]
    pub fn date(&self) -> glib::DateTime {
        unsafe { from_glib_full(ffi::gtk_calendar_get_date(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_calendar_get_day_is_marked")]
    #[doc(alias = "get_day_is_marked")]
    pub fn day_is_marked(&self, day: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_calendar_get_day_is_marked(
                self.to_glib_none().0,
                day,
            ))
        }
    }

    #[doc(alias = "gtk_calendar_get_show_day_names")]
    #[doc(alias = "get_show_day_names")]
    pub fn shows_day_names(&self) -> bool {
        unsafe { from_glib(ffi::gtk_calendar_get_show_day_names(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_calendar_get_show_heading")]
    #[doc(alias = "get_show_heading")]
    pub fn shows_heading(&self) -> bool {
        unsafe { from_glib(ffi::gtk_calendar_get_show_heading(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_calendar_get_show_week_numbers")]
    #[doc(alias = "get_show_week_numbers")]
    pub fn shows_week_numbers(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_calendar_get_show_week_numbers(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_calendar_mark_day")]
    pub fn mark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_mark_day(self.to_glib_none().0, day);
        }
    }

    #[doc(alias = "gtk_calendar_select_day")]
    pub fn select_day(&self, date: &glib::DateTime) {
        unsafe {
            ffi::gtk_calendar_select_day(self.to_glib_none().0, date.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_calendar_set_show_day_names")]
    pub fn set_show_day_names(&self, value: bool) {
        unsafe {
            ffi::gtk_calendar_set_show_day_names(self.to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "gtk_calendar_set_show_heading")]
    pub fn set_show_heading(&self, value: bool) {
        unsafe {
            ffi::gtk_calendar_set_show_heading(self.to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "gtk_calendar_set_show_week_numbers")]
    pub fn set_show_week_numbers(&self, value: bool) {
        unsafe {
            ffi::gtk_calendar_set_show_week_numbers(self.to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "gtk_calendar_unmark_day")]
    pub fn unmark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_unmark_day(self.to_glib_none().0, day);
        }
    }

    pub fn day(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"day\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `day` getter")
        }
    }

    pub fn set_day(&self, day: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"day\0".as_ptr() as *const _,
                day.to_value().to_glib_none().0,
            );
        }
    }

    pub fn month(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"month\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `month` getter")
        }
    }

    pub fn set_month(&self, month: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"month\0".as_ptr() as *const _,
                month.to_value().to_glib_none().0,
            );
        }
    }

    pub fn year(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"year\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `year` getter")
        }
    }

    pub fn set_year(&self, year: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"year\0".as_ptr() as *const _,
                year.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "day-selected")]
    pub fn connect_day_selected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn day_selected_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"day-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    day_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "next-month")]
    pub fn connect_next_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "next-year")]
    pub fn connect_next_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "prev-month")]
    pub fn connect_prev_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prev_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "prev-year")]
    pub fn connect_prev_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prev_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "day")]
    pub fn connect_day_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_day_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::day\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_day_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "month")]
    pub fn connect_month_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-day-names")]
    pub fn connect_show_day_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_day_names_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-day-names\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_day_names_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-heading")]
    pub fn connect_show_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_heading_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-heading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_heading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-week-numbers")]
    pub fn connect_show_week_numbers_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_week_numbers_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-week-numbers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_week_numbers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "year")]
    pub fn connect_year_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Calendar`].
pub struct CalendarBuilder {
    day: Option<i32>,
    month: Option<i32>,
    show_day_names: Option<bool>,
    show_heading: Option<bool>,
    show_week_numbers: Option<bool>,
    year: Option<i32>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
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
    accessible_role: Option<AccessibleRole>,
}

impl CalendarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CalendarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Calendar`].
    pub fn build(self) -> Calendar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref day) = self.day {
            properties.push(("day", day));
        }
        if let Some(ref month) = self.month {
            properties.push(("month", month));
        }
        if let Some(ref show_day_names) = self.show_day_names {
            properties.push(("show-day-names", show_day_names));
        }
        if let Some(ref show_heading) = self.show_heading {
            properties.push(("show-heading", show_heading));
        }
        if let Some(ref show_week_numbers) = self.show_week_numbers {
            properties.push(("show-week-numbers", show_week_numbers));
        }
        if let Some(ref year) = self.year {
            properties.push(("year", year));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
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
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
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
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        glib::Object::new::<Calendar>(&properties)
            .expect("Failed to create an instance of Calendar")
    }

    pub fn day(mut self, day: i32) -> Self {
        self.day = Some(day);
        self
    }

    pub fn month(mut self, month: i32) -> Self {
        self.month = Some(month);
        self
    }

    pub fn show_day_names(mut self, show_day_names: bool) -> Self {
        self.show_day_names = Some(show_day_names);
        self
    }

    pub fn show_heading(mut self, show_heading: bool) -> Self {
        self.show_heading = Some(show_heading);
        self
    }

    pub fn show_week_numbers(mut self, show_week_numbers: bool) -> Self {
        self.show_week_numbers = Some(show_week_numbers);
        self
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
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

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
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

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
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

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
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

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Calendar")
    }
}
