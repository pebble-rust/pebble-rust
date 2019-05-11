pub use crate::pebble::internal::types::{GColor, GRect, GPoint, GSize, tm, TimeUnits, GCompOp};
use crate::pebble::internal::types::GBitmap;
use crate::pebble::internal::functions::{interface, declarations};

pub struct Bitmap {
    pub internal: *mut GBitmap
}

impl Bitmap {
    pub fn new(resource_id: u32) -> Bitmap {
        let internal = interface::gbitmap_create_with_resource(resource_id);
        Bitmap {internal}
    }

    pub fn clean(self) {
        unsafe {
            declarations::gbitmap_destroy(self.internal);
        }
        drop(self);
    }
}