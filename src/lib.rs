#![crate_type="staticlib"]
#![no_std]

// Require the Pebble library
pub mod lib;
pub use lib as pebble;

// Require the application
pub mod app;

#[no_mangle]
pub extern fn main() -> isize {
    app::main();
    0
}