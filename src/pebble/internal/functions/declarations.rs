/*
 * Copyright (c) 2019, Andrew Foote. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the copyright holder nor the
      names of its contributors may be used to endorse or promote products
      derived from this software without specific prior written permission.

 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
 * OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
use crate::pebble::internal::types::*;

extern {
    // App
    pub fn app_event_loop();

    // Window
    pub fn window_create() -> *mut Window;
    pub fn window_destroy(window: *mut Window);
    pub fn window_set_click_config_provider(window: *mut Window, func: extern fn(*mut c_void));
    pub fn window_set_click_config_provider_with_context(window: *mut Window, func: extern fn(*mut u8), ctx: *mut u8);
    pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers);
    pub fn window_set_background_color(window: *mut Window, color: GColor);
    pub fn window_set_user_data(window: *mut Window, data: *mut c_void);
    pub fn window_get_user_data(window: *mut Window) -> *mut c_void;
    pub fn window_stack_push(window: *mut Window, animate: u8);
    pub fn window_get_root_layer(window: *mut Window) -> *mut Layer;
    pub fn window_single_click_subscribe(button: u8, func: extern fn(*mut ClickRecognizer, *mut u8));

    // Layer
    pub fn layer_create(bounds: GRect) -> *mut Layer;
    pub fn layer_destroy(layer: *mut Layer);
    pub fn layer_get_frame(layer: *mut Layer) -> GRect;
    pub fn layer_get_bounds(layer: *mut Layer) -> GRect;
    pub fn layer_add_child(layer: *mut Layer, child: *mut Layer);
    pub fn layer_mark_dirty(layer: *mut Layer);
    pub fn layer_set_update_proc(layer: *mut Layer, func: extern fn(*mut Layer, *mut GContext));

    // TextLayer
    pub fn text_layer_create(bounds: GRect) -> *mut TextLayer;
    pub fn text_layer_set_text(layer: *mut TextLayer, text: *const c_char);
    pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer;
    pub fn text_layer_set_font(layer: *mut TextLayer, font: GFont);

    // GBitmap
    pub fn gbitmap_create_with_resource(id: u32) -> *mut GBitmap;
    pub fn gbitmap_destroy(bitmap: *mut GBitmap);

    // BitmapLayer
    pub fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer;
    pub fn bitmap_layer_set_bitmap(layer: *mut BitmapLayer, bitmap: *mut GBitmap);
    pub fn bitmap_layer_set_compositing_mode(layer: *mut BitmapLayer, mode: GCompOp);
    pub fn bitmap_layer_get_layer(layer: *mut BitmapLayer) -> *mut Layer;

    // Graphics
    pub fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor);
    pub fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16);

    // Wall Time
    pub fn clock_copy_time_string(buffer: *mut c_char, size: u8);
    pub fn clock_is_24h_style() -> u8;
    pub fn clock_get_timezone(buffer: *mut c_char, size: usize);

    pub fn tick_timer_service_subscribe(unit: TimeUnits, func: extern fn(*mut tm, TimeUnits));

    // Standard C - Time
    pub fn time(t: *mut usize) -> usize;
    pub fn localtime(now: *const usize) -> *mut tm;
    pub fn gmtime(now: *const usize) -> *mut tm;

    // Standard C - Locale
    pub fn setlocale(category: i32, locale: *const c_char) -> *const c_char;

    // Standard C - Math
    pub fn rand() -> i32;
    pub fn srand(seed: u32) -> i32;

    // Standard C - Strings
    pub fn strcmp(str1: *const c_char, str2: *const c_char) -> i32;
    pub fn strncmp(str1: *const c_char, str2: *const c_char, num_bytes: usize) -> i32;
    pub fn strcpy(destination: *const c_char, source: *const c_char) -> *const c_char;
    pub fn strncpy(destination: *const c_char, source: *const c_char, num_bytes: usize) -> *const c_char;
    pub fn strcat(destination: *const c_char, source: *const c_char) -> *const c_char;
    pub fn strncat(destination: *const c_char, source: *const c_char, num_bytes: usize) -> *const c_char;
    pub fn strlen(str: *const c_char) -> usize;

    // Standard C - Format
    pub fn snprintf(buf: *const c_char, max: usize, fmt: *const c_char, ...) -> usize;

    // Fonts
    pub fn fonts_get_system_font(key: *const c_char) -> GFont;
    pub fn fonts_load_custom_font(res: ResHandle) -> GFont;

    // Resources
    pub fn resource_get_handle(id: u32) -> ResHandle;

    // Dictionary
    pub fn dict_calc_buffer_size(tuple_count: u8) -> u32;
    pub fn dict_size(iter: *mut DictionaryIterator) -> u32;
    pub fn dict_write_begin(iter: *mut DictionaryIterator, buffer: *mut u8, size: u16) -> DictionaryResult;
    pub fn dict_write_data(iter: *mut DictionaryIterator, key: u32, data: *mut u8, size: u16) -> DictionaryResult;
    pub fn dict_write_cstring(iter: *mut DictionaryIterator, key: u32, cstring: *const c_char) -> DictionaryResult;
    pub fn dict_write_int(iter: *mut DictionaryIterator, key: u32, int: *const c_void,
                            len_bytes: u8, signed: bool) -> DictionaryResult;
    pub fn dict_write_end(iter: *mut DictionaryIterator) -> u32;
    pub fn dict_read_begin_from_buffer(iter: *mut DictionaryIterator, buffer: *mut u8, size: u16) -> *mut Tuple;
    pub fn dict_read_next(iter: *mut DictionaryIterator) -> *mut Tuple;
    pub fn dict_read_first(iter: *mut DictionaryIterator) -> *mut Tuple;
    pub fn dict_find(iter: *mut DictionaryIterator, key: u32) -> *mut Tuple;

    // AppMessage
    pub fn app_message_open(size_in: u32, size_out: u32);
    pub fn app_message_register_inbox_received(callback: extern fn(iter: *mut DictionaryIterator,
        ctx: *const c_void));
    pub fn app_message_register_outbox_sent(callback: extern fn(iter: *mut DictionaryIterator,
                                                                   ctx: *const c_void));
    pub fn app_message_register_inbox_dropped(callback: extern fn(reason: AppMessageResult, ctx: *const c_void));
    pub fn app_message_outbox_begin(iter: *mut *mut DictionaryIterator);
    pub fn app_message_outbox_send();

    // EVENTS
    // Battery
    pub fn battery_state_service_subscribe(handler: extern fn(state: BatteryChargeState));
    pub fn battery_state_service_unsubscribe();
    pub fn battery_state_service_peek() -> BatteryChargeState;

    // Connection
    pub fn connection_service_peek_pebble_app_connection() -> bool;
    pub fn connection_service_peek_pebblekit_connection() -> bool;
    pub fn connection_service_unsubscribe();
    pub fn connection_service_subscribe(handlers: ConnectionHandlers);

    // Logging
    pub fn app_log(level: u8, filename: *const c_char, line_num: u32, msg: *const c_char, ...);
}
