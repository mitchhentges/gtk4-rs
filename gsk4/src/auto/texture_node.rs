// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::RenderNode;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskTextureNode")]
    pub struct TextureNode(Shared<ffi::GskTextureNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for TextureNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_texture_node_get_type()) }
    }
}

impl TextureNode {
    #[doc(alias = "gsk_texture_node_new")]
    pub fn new(texture: &impl IsA<gdk::Texture>, bounds: &graphene::Rect) -> TextureNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_texture_node_new(
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_texture_node_get_texture")]
    #[doc(alias = "get_texture")]
    pub fn texture(&self) -> Option<gdk::Texture> {
        unsafe { from_glib_none(ffi::gsk_texture_node_get_texture(self.to_glib_none().0)) }
    }
}

impl fmt::Display for TextureNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TextureNode")
    }
}
