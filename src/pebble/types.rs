/*
 * This file is part of pebble-rust.
 * Copyright (c) 2019 RoccoDev
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

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