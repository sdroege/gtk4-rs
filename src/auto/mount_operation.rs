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
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Window;

glib_wrapper! {
    pub struct MountOperation(Object<gtk_sys::GtkMountOperation, gtk_sys::GtkMountOperationClass, MountOperationClass>) @extends gio::MountOperation;

    match fn {
        get_type => || gtk_sys::gtk_mount_operation_get_type(),
    }
}

impl MountOperation {
    pub fn new<P: IsA<Window>>(parent: Option<&P>) -> MountOperation {
        assert_initialized_main_thread!();
        unsafe {
            gio::MountOperation::from_glib_full(gtk_sys::gtk_mount_operation_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

pub const NONE_MOUNT_OPERATION: Option<&MountOperation> = None;

pub trait MountOperationExt: 'static {
    fn get_display(&self) -> Option<gdk::Display>;

    fn get_parent(&self) -> Option<Window>;

    fn is_showing(&self) -> bool;

    fn set_display(&self, display: &gdk::Display);

    fn set_parent<P: IsA<Window>>(&self, parent: Option<&P>);

    fn get_property_is_showing(&self) -> bool;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MountOperation>> MountOperationExt for O {
    fn get_display(&self) -> Option<gdk::Display> {
        unsafe {
            from_glib_none(gtk_sys::gtk_mount_operation_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_parent(&self) -> Option<Window> {
        unsafe {
            from_glib_none(gtk_sys::gtk_mount_operation_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_showing(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_mount_operation_is_showing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_display(&self, display: &gdk::Display) {
        unsafe {
            gtk_sys::gtk_mount_operation_set_display(
                self.as_ref().to_glib_none().0,
                display.to_glib_none().0,
            );
        }
    }

    fn set_parent<P: IsA<Window>>(&self, parent: Option<&P>) {
        unsafe {
            gtk_sys::gtk_mount_operation_set_parent(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn get_property_is_showing(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"is-showing\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute(notify_display_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_showing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-showing\0".as_ptr() as *const _,
                Some(transmute(notify_is_showing_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute(notify_parent_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MountOperation")
    }
}
