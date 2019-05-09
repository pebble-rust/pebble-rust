#![allow(unused)]

use crate::pebble::internal::types::*;
use core::intrinsics;

use crate::pebble::internal::functions::declarations;

pub fn app_event_loop() {
    unsafe {
        declarations::app_event_loop();
    }
}

pub fn window_create() -> *mut Window {
    unsafe {
        declarations::window_create()
    }
}

pub fn window_destroy(window: *mut Window) {
    unsafe {
        declarations::window_destroy(window);
    }
}

pub fn window_set_click_config_provider<T>(window: *mut Window, func: extern fn(*mut T)) {
    unsafe {
        declarations::window_set_click_config_provider(window, intrinsics::transmute(func));
    }
}

pub fn window_set_click_config_provider_with_context<T>(window: *mut Window, func: extern fn(*mut T), ctx: *mut T) {
    unsafe {
        declarations::window_set_click_config_provider_with_context(window,
                                                                intrinsics::transmute(func),
                                                                intrinsics::transmute(ctx));
    }
}

pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers) {
    unsafe {
        declarations::window_set_window_handlers(window, handlers);
    }
}

pub fn window_set_background_color(window: *mut Window, color: GColor) {
    unsafe {
        declarations::window_set_background_color(window, color);
    }
}

pub fn window_set_user_data<T>(window: *mut Window, data: *mut T) {
    unsafe {
        declarations::window_set_user_data(window, intrinsics::transmute(data));
    }
}

pub fn window_get_user_data<T>(window: *mut Window) -> *mut T {
    unsafe {
        intrinsics::transmute(declarations::window_get_user_data(window))
    }
}

pub fn window_stack_push(window: *mut Window, animate: bool) {
    unsafe {
        if animate {
            declarations::window_stack_push(window, 1);
        } else {
            declarations::window_stack_push(window, 0);
        }
    }
}

pub fn window_get_root_layer(window: *mut Window) -> *mut Layer {
    unsafe {
        declarations::window_get_root_layer(window)
    }
}

pub fn window_single_click_subscribe<T>(button: u8, func: extern fn(*mut ClickRecognizer, *mut T)) {
    unsafe {
        declarations::window_single_click_subscribe(button, intrinsics::transmute(func));
    }
}

pub fn layer_create(bounds: GRect) -> *mut Layer {
    unsafe {
        declarations::layer_create(bounds)
    }
}

pub fn layer_destroy(layer: *mut Layer) {
    unsafe {
        declarations::layer_destroy(layer);
    }
}

pub fn layer_get_frame(layer: *mut Layer) -> GRect {
    unsafe {
        declarations::layer_get_frame(layer)
    }
}

pub fn layer_get_bounds(layer: *mut Layer) -> GRect {
    unsafe {
        declarations::layer_get_bounds(layer)
    }
}

pub fn layer_add_child(layer: *mut Layer, child: *mut Layer) {
    unsafe {
        declarations::layer_add_child(layer, child);
    }
}

pub fn layer_mark_dirty(layer: *mut Layer) {
    unsafe {
        declarations::layer_mark_dirty(layer);
    }
}

pub fn layer_set_update_proc(layer: *mut Layer, func: extern fn(*mut Layer, *mut GContext)) {
    unsafe {
        declarations::layer_set_update_proc(layer, func);
    }
}

pub fn text_layer_create(bounds: GRect) -> *mut TextLayer {
    unsafe {
        declarations::text_layer_create(bounds)
    }
}

pub fn text_layer_set_text(layer: *mut TextLayer, text: &str) {
    unsafe {
        declarations::text_layer_set_text(layer, text.as_ptr());
    }
}
pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer {
    unsafe {
        declarations::text_layer_get_layer(layer)
    }
}

pub fn gbitmap_create_with_resource(id: u32) -> *mut GBitmap {
    unsafe {
        declarations::gbitmap_create_with_resource(id)
    }
}

pub fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer {
    unsafe {
        declarations::bitmap_layer_create(frame)
    }
}

pub fn bitmap_layer_set_bitmap(layer: *mut BitmapLayer, bitmap: *mut GBitmap) {
    unsafe {
        declarations::bitmap_layer_set_bitmap(layer, bitmap);
    }
}

pub fn bitmap_layer_set_compositing_mode(layer: *mut BitmapLayer, mode: GCompOp) {
    unsafe {
        declarations::bitmap_layer_set_compositing_mode(layer, mode);
    }
}

pub fn bitmap_layer_get_layer(layer: *mut BitmapLayer) -> *mut Layer {
    unsafe {
        declarations::bitmap_layer_get_layer(layer)
    }
}

pub fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor) {
    unsafe {
        declarations::graphics_context_set_fill_color(ctx, color);
    }
}

pub fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16) {
    unsafe {
        declarations::graphics_fill_circle(ctx, center, radius);
    }
}

pub fn clock_is_24h_style() -> bool {
    unsafe {
        let response = declarations::clock_is_24h_style();
        if response == 0 {
            false
        } else {
            true
        }
    }
}

pub fn tick_timer_service_subscribe(unit: TimeUnits, func: extern fn(*mut tm, TimeUnits)) {
    unsafe {
        declarations::tick_timer_service_subscribe(unit, func);
    }
}

pub fn time() -> usize {
    unsafe {
        declarations::time(0 as *mut usize)
    }
}

pub fn localtime(now: usize) -> *mut tm {
    unsafe {
        let now_ptr = &now as *const usize;
        declarations::localtime(now_ptr)
    }
}

pub fn app_log(msg: &str) {
    unsafe {
        declarations::app_log(50, "main\0".as_ptr(), 0, msg.as_ptr());
    }
}
