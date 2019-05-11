use crate::pebble::internal::{types, functions::interface};
use crate::pebble::types::{GRect, Bitmap, GCompOp};

pub struct Layer {
    internal: *mut types::Layer
}

pub struct TextLayer {
    internal: *mut types::TextLayer,
    inner: *mut types::Layer
}

pub struct BitmapLayer {
    internal: *mut types::BitmapLayer,
    inner: *mut types::Layer
}

pub trait ILayer {
    fn get_bounds(&self) -> GRect;
    fn get_frame(&self) -> GRect;
    fn add_child(&self, layer: &ILayer);
    fn mark_dirty(&self);
    fn get_internal(&self) -> *mut types::Layer;
}

impl ILayer for Layer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.internal)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.internal)
    }

    fn add_child(&self, layer: &ILayer) {
        interface::layer_add_child(self.internal, layer.get_internal())
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.internal);
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.internal
    }
}

impl Layer {
    pub fn new(bounds: GRect) -> Layer {
        Layer {
            internal: interface::layer_create(bounds)
        }
    }

    pub fn from_raw(ptr: *mut types::Layer) -> Layer {
        Layer {
            internal: ptr
        }
    }
}

impl ILayer for TextLayer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.inner)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.inner)
    }

    fn add_child(&self, layer: &ILayer) {
        interface::layer_add_child(self.inner, layer.get_internal());
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.inner);
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.inner
    }
}

impl TextLayer {
    pub fn new(bounds: GRect) -> TextLayer {
        let internal = interface::text_layer_create(bounds);
        let inner = interface::text_layer_get_layer(internal);

        TextLayer {
            internal, inner
        }
    }

    pub fn set_text(&self, text: &str) {
        interface::text_layer_set_text(self.internal, text);
    }
}

impl ILayer for BitmapLayer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.inner)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.inner)
    }

    fn add_child(&self, layer: &ILayer) {
        interface::layer_add_child(self.inner, layer.get_internal())
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.inner)
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.inner
    }
}

impl BitmapLayer {
    pub fn new(bounds: GRect) -> BitmapLayer {
        let internal = interface::bitmap_layer_create(bounds);
        let inner = interface::bitmap_layer_get_layer(internal);

        BitmapLayer {
            internal, inner
        }
    }

    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        interface::bitmap_layer_set_bitmap(self.internal, bitmap.internal);
    }

    pub fn set_compositing_mode(&self, mode: GCompOp) {
        interface::bitmap_layer_set_compositing_mode(self.internal, mode);
    }
}