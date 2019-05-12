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

use crate::pebble::internal::functions::declarations::setlocale;

pub fn set_locale(category: i32, locale: &str) {
    unsafe {
        setlocale(category, locale.as_ptr());
    }
}

pub fn get_locale<'a>(category: i32) -> &'a str {
    unsafe {
        let ptr = setlocale(category, 0 as *const u8);
        let slc = core::slice::from_raw_parts(ptr, 5);
        core::str::from_utf8_unchecked(slc)
    }
}