#![crate_type="staticlib"]
#![no_std]
#![no_builtins]

extern crate pebble_rust as pebble;

use pebble::{app, window, layer, WindowPtr};
use pebble::window::WindowHandlers;
use pebble::layer::{ILayer, BitmapLayer};
use pebble::types::{GRect, GPoint, GSize, GColor, Bitmap, GCompOp};

#[no_mangle]
pub fn main() -> isize {
    let app = app::App::new();
    let window = window::Window::new();
    let handlers = WindowHandlers {
        load: load_handler,
        unload: unload_handler,
        appear: appear_handler,
        disappear: disappear_handler
    };
    window.set_handlers(handlers);

    window.push(false);
    app.run_event_loop();
    window.clean_exit();
    0
}

extern fn load_handler(window: WindowPtr) {
    let window = window::Window::from_raw(window);
    let root = window.get_root_layer();
    let bounds = root.get_bounds();

    let bitmap = Bitmap::new(1);
    let bitmap_layer = BitmapLayer::new(bounds);
    bitmap_layer.set_bitmap(&bitmap);
    bitmap_layer.set_compositing_mode(GCompOp::GCompOpSet);

    root.add_child(&bitmap_layer);
}

extern fn unload_handler(window: WindowPtr) {}
extern fn appear_handler(window: WindowPtr) {}
extern fn disappear_handler(window: WindowPtr) {}