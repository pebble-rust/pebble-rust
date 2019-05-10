use crate::pebble::internal::{types, functions::interface};
use crate::pebble::types::GColor;
use crate::pebble::WindowPtr;
use crate::pebble::layer::{Layer, TextLayer, ILayer};
use crate::pebble::internal::types::ClickRecognizer;

pub struct Window {
    internal: *mut types::Window
}

#[derive(Copy, Clone)]
pub struct WindowHandlers {
    pub load: extern fn(WindowPtr),
    pub unload: extern fn(WindowPtr),
    pub appear: extern fn(WindowPtr),
    pub disappear: extern fn(WindowPtr)
}

impl Window {
    pub fn new() -> Window {
        Window {
            internal: interface::window_create()
        }
    }

    pub fn from_raw(ptr: WindowPtr) -> Window {
        Window {
            internal: ptr
        }
    }

    pub fn push(&self, animate: bool) {
        interface::window_stack_push(self.internal, animate);
    }

    pub fn set_handlers(&self, handlers: WindowHandlers) {
        let WindowHandlers {load, unload,
            appear, disappear} = handlers;
        let converted = types::WindowHandlers {
            load, unload, appear, disappear
        };

        interface::window_set_window_handlers(self.internal, converted);
    }

    pub fn set_background_color(&self, color: GColor) {
        interface::window_set_background_color(self.internal, color);
    }

    pub fn get_root_layer(&self) -> Layer {
        let layer_ptr = interface::window_get_root_layer(self.internal);
        Layer::from_raw(layer_ptr)
    }

    pub fn clean_exit(&self) {
        interface::window_destroy(self.internal);
    }
}