#![crate_type="staticlib"]
#![feature(core_intrinsics, intrinsics)]
#![no_std]

// Require the Pebble library
pub mod pebble;

// Require the application
pub mod app;

#[no_mangle]
pub extern fn main() -> isize {
    app::main();
    0
}