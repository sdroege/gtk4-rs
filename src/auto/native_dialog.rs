// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use ResponseType;
use Window;

glib_wrapper! {
    pub struct NativeDialog(Object<gtk_sys::GtkNativeDialog, gtk_sys::GtkNativeDialogClass, NativeDialogClass>);

    match fn {
        get_type => || gtk_sys::gtk_native_dialog_get_type(),
    }
}

pub const NONE_NATIVE_DIALOG: Option<&NativeDialog> = None;

pub trait NativeDialogExt: 'static {
    fn destroy(&self);

    fn get_modal(&self) -> bool;

    fn get_title(&self) -> Option<GString>;

    fn get_transient_for(&self) -> Option<Window>;

    fn get_visible(&self) -> bool;

    fn hide(&self);

    fn run(&self) -> i32;

    fn set_modal(&self, modal: bool);

    fn set_title(&self, title: &str);

    fn set_transient_for<P: IsA<Window>>(&self, parent: Option<&P>);

    fn show(&self);

    fn set_property_visible(&self, visible: bool);

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transient_for_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NativeDialog>> NativeDialogExt for O {
    fn destroy(&self) {
        unsafe {
            gtk_sys::gtk_native_dialog_destroy(self.as_ref().to_glib_none().0);
        }
    }

    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_native_dialog_get_modal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_native_dialog_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_transient_for(&self) -> Option<Window> {
        unsafe {
            from_glib_none(gtk_sys::gtk_native_dialog_get_transient_for(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_native_dialog_get_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hide(&self) {
        unsafe {
            gtk_sys::gtk_native_dialog_hide(self.as_ref().to_glib_none().0);
        }
    }

    fn run(&self) -> i32 {
        unsafe { gtk_sys::gtk_native_dialog_run(self.as_ref().to_glib_none().0) }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            gtk_sys::gtk_native_dialog_set_modal(self.as_ref().to_glib_none().0, modal.to_glib());
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            gtk_sys::gtk_native_dialog_set_title(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn set_transient_for<P: IsA<Window>>(&self, parent: Option<&P>) {
        unsafe {
            gtk_sys::gtk_native_dialog_set_transient_for(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn show(&self) {
        unsafe {
            gtk_sys::gtk_native_dialog_show(self.as_ref().to_glib_none().0);
        }
    }

    fn set_property_visible(&self, visible: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"visible\0".as_ptr() as *const _,
                Value::from(&visible).to_glib_none().0,
            );
        }
    }

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn response_trampoline<P, F: Fn(&P, ResponseType) + 'static>(
            this: *mut gtk_sys::GtkNativeDialog,
            response_id: gtk_sys::GtkResponseType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(
                &NativeDialog::from_glib_borrow(this).unsafe_cast(),
                from_glib(response_id),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"response\0".as_ptr() as *const _,
                Some(transmute(response_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkNativeDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast())
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

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkNativeDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute(notify_title_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_transient_for_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transient_for_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkNativeDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transient-for\0".as_ptr() as *const _,
                Some(transmute(
                    notify_transient_for_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkNativeDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute(notify_visible_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for NativeDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NativeDialog")
    }
}
