// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Expression;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkObjectExpression")]
    pub struct ObjectExpression(Shared<ffi::GtkObjectExpression>);

    match fn {
        ref => |ptr| ffi::gtk_expression_ref(ptr as *mut ffi::GtkExpression),
        unref => |ptr| ffi::gtk_expression_unref(ptr as *mut ffi::GtkExpression),
    }
}

impl glib::StaticType for ObjectExpression {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_object_expression_get_type()) }
    }
}

impl ObjectExpression {
    #[doc(alias = "gtk_object_expression_new")]
    pub fn new(object: &impl IsA<glib::Object>) -> ObjectExpression {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_object_expression_new(
                object.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_object_expression_get_object")]
    #[doc(alias = "get_object")]
    pub fn object(&self) -> Option<glib::Object> {
        unsafe { from_glib_none(ffi::gtk_object_expression_get_object(self.to_glib_none().0)) }
    }
}

impl fmt::Display for ObjectExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ObjectExpression")
    }
}
