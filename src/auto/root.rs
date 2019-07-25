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
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct Root(Interface<gtk_sys::GtkRoot>) @requires Widget, Buildable;

    match fn {
        get_type => || gtk_sys::gtk_root_get_type(),
    }
}

impl Root {
    pub fn get_for_surface<P: IsA<gdk::Surface>>(surface: &P) -> Option<Widget> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gtk_sys::gtk_root_get_for_surface(
                surface.as_ref().to_glib_none().0,
            ))
        }
    }
}

pub const NONE_ROOT: Option<&Root> = None;

pub trait RootExt: 'static {
    fn get_focus(&self) -> Option<Widget>;

    fn set_focus<P: IsA<Widget>>(&self, focus: Option<&P>);

    fn get_property_focus_widget(&self) -> Option<Widget>;

    fn set_property_focus_widget(&self, focus_widget: Option<&Widget>);

    fn connect_property_focus_widget_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<Root>> RootExt for O {
    fn get_focus(&self) -> Option<Widget> {
        unsafe { from_glib_none(gtk_sys::gtk_root_get_focus(self.as_ref().to_glib_none().0)) }
    }

    fn set_focus<P: IsA<Widget>>(&self, focus: Option<&P>) {
        unsafe {
            gtk_sys::gtk_root_set_focus(
                self.as_ref().to_glib_none().0,
                focus.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn get_property_focus_widget(&self) -> Option<Widget> {
        unsafe {
            let mut value = Value::from_type(<Widget as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"focus-widget\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_focus_widget(&self, focus_widget: Option<&Widget>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"focus-widget\0".as_ptr() as *const _,
                Value::from(focus_widget).to_glib_none().0,
            );
        }
    }

    fn connect_property_focus_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRoot,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Root>,
        {
            let f: &F = &*(f as *const F);
            f(&Root::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::focus-widget\0".as_ptr() as *const _,
                Some(transmute(
                    notify_focus_widget_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root")
    }
}
