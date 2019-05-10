mod internal;

pub mod app;
pub mod window;
pub mod types;
pub mod layer;

pub use pebble::internal::types::Window as RawWindow;
pub type WindowPtr = *mut RawWindow;

pub use pebble::internal::functions::interface::app_log as println;

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}