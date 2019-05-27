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

use crate::pebble::internal::functions::declarations::*;

pub fn get_time_string() -> alloc::string::String {
    const max_size: usize = 8usize; // 00:00 AM
    let mut buf = [0u8; max_size];

    unsafe {
        clock_copy_time_string(buf.as_mut_ptr(), max_size as u8);
        alloc::string::String::from_utf8_unchecked(buf.to_vec())
    }
}

pub fn is_24h() -> bool {
    unsafe {
        clock_is_24h_style() != 0
    }
}

pub fn get_timezone() -> alloc::string::String {
    let mut buf = [0u8; 32];

    unsafe {
        clock_get_timezone(buf.as_mut_ptr(), 32);
        alloc::string::String::from_utf8_unchecked(buf.to_vec())
    }
}