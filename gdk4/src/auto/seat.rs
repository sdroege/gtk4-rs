// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Device;
use crate::DeviceTool;
use crate::Display;
use crate::SeatCapabilities;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Seat(Object<ffi::GdkSeat>);

    match fn {
        type_ => || ffi::gdk_seat_get_type(),
    }
}

impl Seat {
    #[doc(alias = "gdk_seat_get_capabilities")]
    #[doc(alias = "get_capabilities")]
    pub fn capabilities(&self) -> SeatCapabilities {
        unsafe { from_glib(ffi::gdk_seat_get_capabilities(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_seat_get_devices")]
    #[doc(alias = "get_devices")]
    pub fn devices(&self, capabilities: SeatCapabilities) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_seat_get_devices(
                self.to_glib_none().0,
                capabilities.into_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_seat_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_seat_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_seat_get_keyboard")]
    #[doc(alias = "get_keyboard")]
    pub fn keyboard(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_seat_get_keyboard(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_seat_get_pointer")]
    #[doc(alias = "get_pointer")]
    pub fn pointer(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_seat_get_pointer(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_seat_get_tools")]
    #[doc(alias = "get_tools")]
    pub fn tools(&self) -> Vec<DeviceTool> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_seat_get_tools(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "device-added")]
    pub fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<F: Fn(&Seat, &Device) + 'static>(
            this: *mut ffi::GdkSeat,
            device: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "device-removed")]
    pub fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<F: Fn(&Seat, &Device) + 'static>(
            this: *mut ffi::GdkSeat,
            device: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tool-added")]
    pub fn connect_tool_added<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tool_added_trampoline<F: Fn(&Seat, &DeviceTool) + 'static>(
            this: *mut ffi::GdkSeat,
            tool: *mut ffi::GdkDeviceTool,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tool-removed")]
    pub fn connect_tool_removed<F: Fn(&Self, &DeviceTool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn tool_removed_trampoline<F: Fn(&Seat, &DeviceTool) + 'static>(
            this: *mut ffi::GdkSeat,
            tool: *mut ffi::GdkDeviceTool,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Seat")
    }
}
