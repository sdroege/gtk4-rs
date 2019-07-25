// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct EntryBuffer(Object<gtk_sys::GtkEntryBuffer, gtk_sys::GtkEntryBufferClass, EntryBufferClass>);

    match fn {
        get_type => || gtk_sys::gtk_entry_buffer_get_type(),
    }
}

pub const NONE_ENTRY_BUFFER: Option<&EntryBuffer> = None;

pub trait EntryBufferExt: 'static {
    fn emit_deleted_text(&self, position: u32, n_chars: u32);

    fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32);

    fn get_property_length(&self) -> u32;

    fn get_property_max_length(&self) -> i32;

    fn set_property_max_length(&self, max_length: i32);

    fn get_property_text(&self) -> Option<GString>;

    fn set_property_text(&self, text: Option<&str>);

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EntryBuffer>> EntryBufferExt for O {
    fn emit_deleted_text(&self, position: u32, n_chars: u32) {
        unsafe {
            gtk_sys::gtk_entry_buffer_emit_deleted_text(
                self.as_ref().to_glib_none().0,
                position,
                n_chars,
            );
        }
    }

    fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32) {
        unsafe {
            gtk_sys::gtk_entry_buffer_emit_inserted_text(
                self.as_ref().to_glib_none().0,
                position,
                chars.to_glib_none().0,
                n_chars,
            );
        }
    }

    fn get_property_length(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"length\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn get_property_max_length(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-length\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn set_property_max_length(&self, max_length: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-length\0".as_ptr() as *const _,
                Value::from(&max_length).to_glib_none().0,
            );
        }
    }

    fn get_property_text(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"text\0".as_ptr() as *const _,
                Value::from(text).to_glib_none().0,
            );
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkEntryBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<EntryBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute(notify_length_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkEntryBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<EntryBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-length\0".as_ptr() as *const _,
                Some(transmute(notify_max_length_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkEntryBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<EntryBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EntryBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EntryBuffer")
    }
}
