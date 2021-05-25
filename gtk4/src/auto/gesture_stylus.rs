// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::GestureSingle;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct GestureStylus(Object<ffi::GtkGestureStylus, ffi::GtkGestureStylusClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_stylus_get_type(),
    }
}

impl GestureStylus {
    #[doc(alias = "gtk_gesture_stylus_new")]
    pub fn new() -> GestureStylus {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_stylus_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`GestureStylus`]
    /// This method returns an instance of [`GestureStylusBuilder`] which can be used to create a [`GestureStylus`].
    pub fn builder() -> GestureStylusBuilder {
        GestureStylusBuilder::default()
    }

    #[doc(alias = "gtk_gesture_stylus_get_axis")]
    #[doc(alias = "get_axis")]
    pub fn axis(&self, axis: gdk::AxisUse) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_stylus_get_axis(
                self.to_glib_none().0,
                axis.into_glib(),
                value.as_mut_ptr(),
            ));
            let value = value.assume_init();
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_gesture_stylus_get_backlog")]
    #[doc(alias = "get_backlog")]
    pub fn backlog(&self) -> Option<Vec<gdk::TimeCoord>> {
        unsafe {
            let mut backlog = ptr::null_mut();
            let mut n_elems = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_stylus_get_backlog(
                self.to_glib_none().0,
                &mut backlog,
                n_elems.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_full_num(
                    backlog,
                    n_elems.assume_init() as usize,
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_gesture_stylus_get_device_tool")]
    #[doc(alias = "get_device_tool")]
    pub fn device_tool(&self) -> Option<gdk::DeviceTool> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_stylus_get_device_tool(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "down")]
    pub fn connect_down<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn down_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"down\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    down_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "motion")]
    pub fn connect_motion<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "proximity")]
    pub fn connect_proximity<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn proximity_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"proximity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    proximity_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "up")]
    pub fn connect_up<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn up_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureStylus,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"up\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    up_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureStylus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`GestureStylus`].
pub struct GestureStylusBuilder {
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl GestureStylusBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GestureStylusBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureStylus`].
    pub fn build(self) -> GestureStylus {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        glib::Object::new::<GestureStylus>(&properties)
            .expect("Failed to create an instance of GestureStylus")
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for GestureStylus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureStylus")
    }
}
