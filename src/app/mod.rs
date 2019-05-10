use crate::pebble::{app, window, layer, WindowPtr};
use crate::pebble::window::WindowHandlers;
use crate::pebble::layer::{ILayer, TextLayer};
use crate::pebble::types::{GRect, GPoint, GSize, GColor};

pub fn main() {
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
}

extern fn load_handler(window: WindowPtr) {
    let window = window::Window::from_raw(window);
    let root = window.get_root_layer();
    let bounds = root.get_bounds();
    
    let w = bounds.size.w;

    let bounds = GRect {
        origin: GPoint {x: 50, y: 50},
        size: GSize {w, h: 20}
    };

    let text = TextLayer::new(bounds);
    text.set_text("Hello from Rust!\0");

    root.add_child(&text);
}

extern fn unload_handler(window: WindowPtr) {}
extern fn appear_handler(window: WindowPtr) {}
extern fn disappear_handler(window: WindowPtr) {}