// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::IsA;
use glib::translate::*;
use graphene;
use gsk_sys;
use std::fmt;
use RenderNode;

glib_wrapper! {
    pub struct TextureNode(Object<gsk_sys::GskTextureNode, TextureNodeClass>) @extends RenderNode;

    match fn {
        get_type => || gsk_sys::gsk_texture_node_get_type(),
    }
}

impl TextureNode {
    pub fn new<P: IsA<gdk::Texture>>(texture: &P, bounds: &graphene::Rect) -> TextureNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_texture_node_new(
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    pub fn get_texture(&self) -> Option<gdk::Texture> {
        unsafe { from_glib_none(gsk_sys::gsk_texture_node_get_texture(self.to_glib_none().0)) }
    }
}

impl fmt::Display for TextureNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureNode")
    }
}
