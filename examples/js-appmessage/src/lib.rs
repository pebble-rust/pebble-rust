#![crate_type="staticlib"]
#![no_std]
#![no_builtins]
#![feature(alloc, alloc_error_handler)]

extern crate alloc;
extern crate pebble_rust as pebble;

use pebble::{app, window, WindowPtr};
use pebble::app_message::*;
use pebble::layer::{ILayer, TextLayer};
use pebble::types::{GPoint, GRect, GSize};
use pebble::window::WindowHandlers;

const MESSAGE_KEY_EXAMPLE: u32 = 1768777472;

#[global_allocator]
static ALLOC: pebble::alloc::Allocator = pebble::alloc::Allocator;

static mut TEXT_LAYER: Option<TextLayer> = None;

#[alloc_error_handler]
pub fn error_handler(layout: core::alloc::Layout) -> ! {
    loop {}
}

#[no_mangle]
pub fn main() -> isize {
    AppMessage::register_inbox(message_received);

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

extern fn message_received(dict_ptr: pebble::types::DictPtr, ctx: pebble::types::VoidPtr) {
    let dict = Dictionary::from_raw(dict_ptr);
    let tuple = dict.find(MESSAGE_KEY_EXAMPLE);
    unsafe {
        if let Some(tuple) = tuple {
            if let Some(layer) = &TEXT_LAYER {

                // Get the string value from the tuple
                // Note: you should probably implement proper error handling
                let str = tuple.get_string().unwrap();

                // Allocate the string on the heap so that it's available
                // when TextLayer tries to draw it
                let str2 = alloc::string::String::from(str);

                layer.set_text(str2.as_str());
            }
        }
    }
}

extern fn load_handler(window: WindowPtr) {
    let window = window::Window::from_raw(window);
    let root = window.get_root_layer();
    let bounds = root.get_bounds();

    let window_width = bounds.size.w;
    let window_height = bounds.size.h;

    let bounds = GRect {
        origin: GPoint {x: window_width / 9, y: window_height / 2 - 20},
        size: GSize {w: window_width, h: 20}
    };

    unsafe {
        let text = TextLayer::new(bounds);
        text.set_text("Loading...\0");
        root.add_child(&text);
        TEXT_LAYER = Some(text);
    }
    AppMessage::open(200, 200);
}

extern fn unload_handler(_window: WindowPtr) {}
extern fn appear_handler(_window: WindowPtr) {}
extern fn disappear_handler(_window: WindowPtr) {}