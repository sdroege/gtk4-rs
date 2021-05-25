// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ShortcutAction;
use crate::ShortcutTrigger;
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
    pub struct Shortcut(Object<ffi::GtkShortcut, ffi::GtkShortcutClass>);

    match fn {
        type_ => || ffi::gtk_shortcut_get_type(),
    }
}

impl Shortcut {
    #[doc(alias = "gtk_shortcut_new")]
    pub fn new<P: IsA<ShortcutTrigger>, Q: IsA<ShortcutAction>>(
        trigger: Option<&P>,
        action: Option<&Q>,
    ) -> Shortcut {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_shortcut_new(
                trigger.map(|p| p.as_ref()).to_glib_full(),
                action.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }

    //#[doc(alias = "gtk_shortcut_new_with_arguments")]
    //#[doc(alias = "new_with_arguments")]
    //pub fn with_arguments<P: IsA<ShortcutTrigger>, Q: IsA<ShortcutAction>>(trigger: Option<&P>, action: Option<&Q>, format_string: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Shortcut {
    //    unsafe { TODO: call ffi:gtk_shortcut_new_with_arguments() }
    //}

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Shortcut`]
    /// This method returns an instance of [`ShortcutBuilder`] which can be used to create a [`Shortcut`].
    pub fn builder() -> ShortcutBuilder {
        ShortcutBuilder::default()
    }

    #[doc(alias = "gtk_shortcut_get_action")]
    #[doc(alias = "get_action")]
    pub fn action(&self) -> Option<ShortcutAction> {
        unsafe { from_glib_none(ffi::gtk_shortcut_get_action(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_shortcut_get_arguments")]
    #[doc(alias = "get_arguments")]
    pub fn arguments(&self) -> Option<glib::Variant> {
        unsafe { from_glib_none(ffi::gtk_shortcut_get_arguments(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_shortcut_get_trigger")]
    #[doc(alias = "get_trigger")]
    pub fn trigger(&self) -> Option<ShortcutTrigger> {
        unsafe { from_glib_none(ffi::gtk_shortcut_get_trigger(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_shortcut_set_action")]
    pub fn set_action<P: IsA<ShortcutAction>>(&self, action: Option<&P>) {
        unsafe {
            ffi::gtk_shortcut_set_action(
                self.to_glib_none().0,
                action.map(|p| p.as_ref()).to_glib_full(),
            );
        }
    }

    #[doc(alias = "gtk_shortcut_set_arguments")]
    pub fn set_arguments(&self, args: Option<&glib::Variant>) {
        unsafe {
            ffi::gtk_shortcut_set_arguments(self.to_glib_none().0, args.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_shortcut_set_trigger")]
    pub fn set_trigger<P: IsA<ShortcutTrigger>>(&self, trigger: Option<&P>) {
        unsafe {
            ffi::gtk_shortcut_set_trigger(
                self.to_glib_none().0,
                trigger.map(|p| p.as_ref()).to_glib_full(),
            );
        }
    }

    #[doc(alias = "action")]
    pub fn connect_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_trampoline<F: Fn(&Shortcut) + 'static>(
            this: *mut ffi::GtkShortcut,
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
                b"notify::action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "arguments")]
    pub fn connect_arguments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_arguments_trampoline<F: Fn(&Shortcut) + 'static>(
            this: *mut ffi::GtkShortcut,
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
                b"notify::arguments\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_arguments_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "trigger")]
    pub fn connect_trigger_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_trigger_trampoline<F: Fn(&Shortcut) + 'static>(
            this: *mut ffi::GtkShortcut,
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
                b"notify::trigger\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_trigger_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Shortcut`].
pub struct ShortcutBuilder {
    action: Option<ShortcutAction>,
    arguments: Option<glib::Variant>,
    trigger: Option<ShortcutTrigger>,
}

impl ShortcutBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ShortcutBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Shortcut`].
    pub fn build(self) -> Shortcut {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref action) = self.action {
            properties.push(("action", action));
        }
        if let Some(ref arguments) = self.arguments {
            properties.push(("arguments", arguments));
        }
        if let Some(ref trigger) = self.trigger {
            properties.push(("trigger", trigger));
        }
        glib::Object::new::<Shortcut>(&properties)
            .expect("Failed to create an instance of Shortcut")
    }

    pub fn action<P: IsA<ShortcutAction>>(mut self, action: &P) -> Self {
        self.action = Some(action.clone().upcast());
        self
    }

    pub fn arguments(mut self, arguments: &glib::Variant) -> Self {
        self.arguments = Some(arguments.clone());
        self
    }

    pub fn trigger<P: IsA<ShortcutTrigger>>(mut self, trigger: &P) -> Self {
        self.trigger = Some(trigger.clone().upcast());
        self
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Shortcut")
    }
}
