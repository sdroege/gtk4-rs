// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use TreeDragSource;
use TreeIter;
use TreeModel;
use TreePath;

glib_wrapper! {
    pub struct TreeModelFilter(Object<gtk_sys::GtkTreeModelFilter, gtk_sys::GtkTreeModelFilterClass, TreeModelFilterClass>) @implements TreeDragSource, TreeModel;

    match fn {
        get_type => || gtk_sys::gtk_tree_model_filter_get_type(),
    }
}

pub const NONE_TREE_MODEL_FILTER: Option<&TreeModelFilter> = None;

pub trait TreeModelFilterExt: 'static {
    fn clear_cache(&self);

    fn convert_child_iter_to_iter(&self, child_iter: &TreeIter) -> Option<TreeIter>;

    fn convert_child_path_to_path(&self, child_path: &TreePath) -> Option<TreePath>;

    fn convert_iter_to_child_iter(&self, filter_iter: &TreeIter) -> TreeIter;

    fn convert_path_to_child_path(&self, filter_path: &TreePath) -> Option<TreePath>;

    fn get_model(&self) -> Option<TreeModel>;

    fn refilter(&self);

    //fn set_modify_func<P: Fn(&TreeModel, &TreeIter, &glib::Value, i32) + 'static>(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }, func: P);

    fn set_visible_column(&self, column: i32);

    fn set_visible_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P);

    fn get_property_child_model(&self) -> Option<TreeModel>;
}

impl<O: IsA<TreeModelFilter>> TreeModelFilterExt for O {
    fn clear_cache(&self) {
        unsafe {
            gtk_sys::gtk_tree_model_filter_clear_cache(self.as_ref().to_glib_none().0);
        }
    }

    fn convert_child_iter_to_iter(&self, child_iter: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut filter_iter = TreeIter::uninitialized();
            let ret = from_glib(gtk_sys::gtk_tree_model_filter_convert_child_iter_to_iter(
                self.as_ref().to_glib_none().0,
                filter_iter.to_glib_none_mut().0,
                mut_override(child_iter.to_glib_none().0),
            ));
            if ret {
                Some(filter_iter)
            } else {
                None
            }
        }
    }

    fn convert_child_path_to_path(&self, child_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_model_filter_convert_child_path_to_path(
                self.as_ref().to_glib_none().0,
                mut_override(child_path.to_glib_none().0),
            ))
        }
    }

    fn convert_iter_to_child_iter(&self, filter_iter: &TreeIter) -> TreeIter {
        unsafe {
            let mut child_iter = TreeIter::uninitialized();
            gtk_sys::gtk_tree_model_filter_convert_iter_to_child_iter(
                self.as_ref().to_glib_none().0,
                child_iter.to_glib_none_mut().0,
                mut_override(filter_iter.to_glib_none().0),
            );
            child_iter
        }
    }

    fn convert_path_to_child_path(&self, filter_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_model_filter_convert_path_to_child_path(
                self.as_ref().to_glib_none().0,
                mut_override(filter_path.to_glib_none().0),
            ))
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tree_model_filter_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn refilter(&self) {
        unsafe {
            gtk_sys::gtk_tree_model_filter_refilter(self.as_ref().to_glib_none().0);
        }
    }

    //fn set_modify_func<P: Fn(&TreeModel, &TreeIter, &glib::Value, i32) + 'static>(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }, func: P) {
    //    unsafe { TODO: call gtk_sys:gtk_tree_model_filter_set_modify_func() }
    //}

    fn set_visible_column(&self, column: i32) {
        unsafe {
            gtk_sys::gtk_tree_model_filter_set_visible_column(
                self.as_ref().to_glib_none().0,
                column,
            );
        }
    }

    fn set_visible_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box::new(func);
        unsafe extern "C" fn func_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            model: *mut gtk_sys::GtkTreeModel,
            iter: *mut gtk_sys::GtkTreeIter,
            data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let model = from_glib_borrow(model);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(data as *mut _);
            let res = (*callback)(&model, &iter);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            data: glib_sys::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            gtk_sys::gtk_tree_model_filter_set_visible_func(
                self.as_ref().to_glib_none().0,
                func,
                Box::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn get_property_child_model(&self) -> Option<TreeModel> {
        unsafe {
            let mut value = Value::from_type(<TreeModel as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"child-model\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `child-model` getter")
        }
    }
}

impl fmt::Display for TreeModelFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeModelFilter")
    }
}
