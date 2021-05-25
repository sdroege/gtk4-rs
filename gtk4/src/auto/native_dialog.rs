// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ResponseType;
use crate::Window;
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
    pub struct NativeDialog(Object<ffi::GtkNativeDialog, ffi::GtkNativeDialogClass>);

    match fn {
        type_ => || ffi::gtk_native_dialog_get_type(),
    }
}

pub const NONE_NATIVE_DIALOG: Option<&NativeDialog> = None;

pub trait NativeDialogExt: 'static {
    #[doc(alias = "gtk_native_dialog_destroy")]
    fn destroy(&self);

    #[doc(alias = "gtk_native_dialog_get_modal")]
    #[doc(alias = "get_modal")]
    fn is_modal(&self) -> bool;

    #[doc(alias = "gtk_native_dialog_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_native_dialog_get_transient_for")]
    #[doc(alias = "get_transient_for")]
    fn transient_for(&self) -> Option<Window>;

    #[doc(alias = "gtk_native_dialog_get_visible")]
    #[doc(alias = "get_visible")]
    fn is_visible(&self) -> bool;

    #[doc(alias = "gtk_native_dialog_hide")]
    fn hide(&self);

    #[doc(alias = "gtk_native_dialog_set_modal")]
    fn set_modal(&self, modal: bool);

    #[doc(alias = "gtk_native_dialog_set_title")]
    fn set_title(&self, title: &str);

    #[doc(alias = "gtk_native_dialog_set_transient_for")]
    fn set_transient_for<P: IsA<Window>>(&self, parent: Option<&P>);

    #[doc(alias = "gtk_native_dialog_show")]
    fn show(&self);

    fn set_visible(&self, visible: bool);

    #[doc(alias = "response")]
    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "modal")]
    fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "transient-for")]
    fn connect_transient_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible")]
    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NativeDialog>> NativeDialogExt for O {
    fn destroy(&self) {
        unsafe {
            ffi::gtk_native_dialog_destroy(self.as_ref().to_glib_none().0);
        }
    }

    fn is_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_native_dialog_get_modal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_native_dialog_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn transient_for(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_native_dialog_get_transient_for(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_native_dialog_get_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hide(&self) {
        unsafe {
            ffi::gtk_native_dialog_hide(self.as_ref().to_glib_none().0);
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_native_dialog_set_modal(self.as_ref().to_glib_none().0, modal.into_glib());
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_native_dialog_set_title(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn set_transient_for<P: IsA<Window>>(&self, parent: Option<&P>) {
        unsafe {
            ffi::gtk_native_dialog_set_transient_for(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn show(&self) {
        unsafe {
            ffi::gtk_native_dialog_show(self.as_ref().to_glib_none().0);
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"visible\0".as_ptr() as *const _,
                visible.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "response")]
    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn response_trampoline<
            P: IsA<NativeDialog>,
            F: Fn(&P, ResponseType) + 'static,
        >(
            this: *mut ffi::GtkNativeDialog,
            response_id: ffi::GtkResponseType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &NativeDialog::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(response_id),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"response\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modal")]
    fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<P: IsA<NativeDialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkNativeDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<NativeDialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkNativeDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transient-for")]
    fn connect_transient_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transient_for_trampoline<
            P: IsA<NativeDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkNativeDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transient-for\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transient_for_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible")]
    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<
            P: IsA<NativeDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkNativeDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for NativeDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NativeDialog")
    }
}
