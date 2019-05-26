#![crate_type="staticlib"]
#![no_std]
#![no_builtins]

#[macro_use]
extern crate pebble_rust as pebble;

use pebble::{app, window, layer, WindowPtr};
use pebble::window::WindowHandlers;
use pebble::layer::{ILayer, TextLayer};
use pebble::types::{GRect, GPoint, GSize, GColor};

#[no_mangle]
pub fn main() -> isize {
    pbl_log!("Loading app...");

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

    pbl_log!("Exiting...");
    0
}

extern fn load_handler(window: WindowPtr) {
    let window = window::Window::from_raw(window);
    let root = window.get_root_layer();
    let bounds = root.get_bounds();

    let window_width = bounds.size.w;
    let window_height = bounds.size.h;

    let bounds = GRect {
        origin: GPoint {x: window_width / 4, y: window_height / 2 - 20},
        size: GSize {w: window_width, h: 20}
    };

    // We can print whatever we want.
    pbl_log!("This works like a %s, I can print numbers like %d", nt!("printf").as_ptr(), 25);

    // Or we can use other logging levels.
    pbl_warn!("This is a warning.");
    pbl_err!("Oops, something went wrong.");

    // These logging calls automatically null-terminate the format string, but not the arguments.
    // Hence the nt! call in the pbl_log! call.

    let text = TextLayer::new(bounds);

    // The `nt!` macro appends a null byte at the end of the string.
    text.set_text(nt!("Hello from Rust!"));

    text.set_font(pebble::system::fonts::Font::get_system(nt!("RESOURCE_ID_ROBOTO_CONDENSED_21")));

    root.add_child(&text);
}

extern fn unload_handler(window: WindowPtr) {}
extern fn appear_handler(window: WindowPtr) {}
extern fn disappear_handler(window: WindowPtr) {}