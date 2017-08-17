// This file was generated by gir (651da6e) from gir-files (???)
// DO NOT EDIT

use ClockTime;
use Error;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Object(Object<ffi::GstObject>);

    match fn {
        get_type => || ffi::gst_object_get_type(),
    }
}

impl Object {
    pub fn check_uniqueness(list: &[Object], name: &str) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_object_check_uniqueness(list.to_glib_none().0, name.to_glib_none().0))
        }
    }

    //pub fn default_deep_notify<P: IsA<glib::Object>, Q: IsA<Object>, R: IsA</*Ignored*/glib::ParamSpec>>(object: &P, orig: &Q, pspec: &R, excluded_props: &[&str]) {
    //    unsafe { TODO: call ffi::gst_object_default_deep_notify() }
    //}

    //pub fn ref_sink<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(object: P) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gst_object_ref_sink() }
    //}

    //pub fn replace<'a, 'b, P: IsA<Object> + 'a, Q: Into<Option<&'a P>>, R: IsA<Object> + 'b, S: Into<Option<&'b R>>>(oldobj: Q, newobj: S) -> bool {
    //    unsafe { TODO: call ffi::gst_object_replace() }
    //}
}

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

pub trait GstObjectExt {
    //fn add_control_binding(&self, binding: /*Ignored*/&ControlBinding) -> bool;

    fn default_error<'a, P: Into<Option<&'a str>>>(&self, error: &Error, debug: P);

    //fn get_control_binding(&self, property_name: &str) -> /*Ignored*/Option<ControlBinding>;

    fn get_control_rate(&self) -> ClockTime;

    //fn get_g_value_array(&self, property_name: &str, timestamp: ClockTime, interval: ClockTime, values: /*Ignored*/&[&glib::Value]) -> bool;

    fn get_name(&self) -> String;

    fn get_parent(&self) -> Option<Object>;

    fn get_path_string(&self) -> String;

    //fn get_value(&self, property_name: &str, timestamp: ClockTime) -> /*Ignored*/Option<glib::Value>;

    //fn get_value_array<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, property_name: &str, timestamp: ClockTime, interval: ClockTime, n_values: u32, values: P) -> bool;

    fn has_active_control_bindings(&self) -> bool;

    fn has_ancestor<P: IsA<Object>>(&self, ancestor: &P) -> bool;

    fn has_as_ancestor<P: IsA<Object>>(&self, ancestor: &P) -> bool;

    fn has_as_parent<P: IsA<Object>>(&self, parent: &P) -> bool;

    //fn remove_control_binding(&self, binding: /*Ignored*/&ControlBinding) -> bool;

    fn set_control_binding_disabled(&self, property_name: &str, disabled: bool);

    fn set_control_bindings_disabled(&self, disabled: bool);

    fn set_control_rate(&self, control_rate: ClockTime);

    fn set_name<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Result<(), glib::error::BoolError>;

    fn set_parent<P: IsA<Object>>(&self, parent: &P) -> Result<(), glib::error::BoolError>;

    fn suggest_next_sync(&self) -> ClockTime;

    fn sync_values(&self, timestamp: ClockTime) -> Result<(), glib::error::BoolError>;

    fn unparent(&self);

    //fn connect_deep_notify<Unsupported or ignored types>(&self, f: F) -> u64;
}

impl<O: IsA<Object>> GstObjectExt for O {
    //fn add_control_binding(&self, binding: /*Ignored*/&ControlBinding) -> bool {
    //    unsafe { TODO: call ffi::gst_object_add_control_binding() }
    //}

    fn default_error<'a, P: Into<Option<&'a str>>>(&self, error: &Error, debug: P) {
        let debug = debug.into();
        let debug = debug.to_glib_none();
        unsafe {
            ffi::gst_object_default_error(self.to_glib_none().0, error.to_glib_none().0, debug.0);
        }
    }

    //fn get_control_binding(&self, property_name: &str) -> /*Ignored*/Option<ControlBinding> {
    //    unsafe { TODO: call ffi::gst_object_get_control_binding() }
    //}

    fn get_control_rate(&self) -> ClockTime {
        unsafe {
            ffi::gst_object_get_control_rate(self.to_glib_none().0)
        }
    }

    //fn get_g_value_array(&self, property_name: &str, timestamp: ClockTime, interval: ClockTime, values: /*Ignored*/&[&glib::Value]) -> bool {
    //    unsafe { TODO: call ffi::gst_object_get_g_value_array() }
    //}

    fn get_name(&self) -> String {
        unsafe {
            from_glib_full(ffi::gst_object_get_name(self.to_glib_none().0))
        }
    }

    fn get_parent(&self) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::gst_object_get_parent(self.to_glib_none().0))
        }
    }

    fn get_path_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gst_object_get_path_string(self.to_glib_none().0))
        }
    }

    //fn get_value(&self, property_name: &str, timestamp: ClockTime) -> /*Ignored*/Option<glib::Value> {
    //    unsafe { TODO: call ffi::gst_object_get_value() }
    //}

    //fn get_value_array<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, property_name: &str, timestamp: ClockTime, interval: ClockTime, n_values: u32, values: P) -> bool {
    //    unsafe { TODO: call ffi::gst_object_get_value_array() }
    //}

    fn has_active_control_bindings(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_active_control_bindings(self.to_glib_none().0))
        }
    }

    fn has_ancestor<P: IsA<Object>>(&self, ancestor: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_ancestor(self.to_glib_none().0, ancestor.to_glib_none().0))
        }
    }

    fn has_as_ancestor<P: IsA<Object>>(&self, ancestor: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_as_ancestor(self.to_glib_none().0, ancestor.to_glib_none().0))
        }
    }

    fn has_as_parent<P: IsA<Object>>(&self, parent: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_as_parent(self.to_glib_none().0, parent.to_glib_none().0))
        }
    }

    //fn remove_control_binding(&self, binding: /*Ignored*/&ControlBinding) -> bool {
    //    unsafe { TODO: call ffi::gst_object_remove_control_binding() }
    //}

    fn set_control_binding_disabled(&self, property_name: &str, disabled: bool) {
        unsafe {
            ffi::gst_object_set_control_binding_disabled(self.to_glib_none().0, property_name.to_glib_none().0, disabled.to_glib());
        }
    }

    fn set_control_bindings_disabled(&self, disabled: bool) {
        unsafe {
            ffi::gst_object_set_control_bindings_disabled(self.to_glib_none().0, disabled.to_glib());
        }
    }

    fn set_control_rate(&self, control_rate: ClockTime) {
        unsafe {
            ffi::gst_object_set_control_rate(self.to_glib_none().0, control_rate);
        }
    }

    fn set_name<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Result<(), glib::error::BoolError> {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_object_set_name(self.to_glib_none().0, name.0), "Failed to set object name")
        }
    }

    fn set_parent<P: IsA<Object>>(&self, parent: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_object_set_parent(self.to_glib_none().0, parent.to_glib_none().0), "Failed to set parent object")
        }
    }

    fn suggest_next_sync(&self) -> ClockTime {
        unsafe {
            ffi::gst_object_suggest_next_sync(self.to_glib_none().0)
        }
    }

    fn sync_values(&self, timestamp: ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_object_sync_values(self.to_glib_none().0, timestamp), "Failed to sync values")
        }
    }

    fn unparent(&self) {
        unsafe {
            ffi::gst_object_unparent(self.to_glib_none().0);
        }
    }

    //fn connect_deep_notify<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored prop: GObject.ParamSpec
    //}
}
