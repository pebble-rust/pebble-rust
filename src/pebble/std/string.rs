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

pub fn compare_strings(str1: &str, str2: &str) -> i32 {
    unsafe { strcmp(str1.as_ptr(), str2.as_ptr()) }
}

pub fn compare_strings_bytes(str1: &str, str2: &str, max_bytes: usize) -> i32 {
    unsafe { strncmp(str1.as_ptr(), str2.as_ptr(), max_bytes) }
}

pub fn copy_strings<'a>(source: &'a str, dest: &'a str) -> &'a str {
    unsafe {
        ptr_to_str(strcpy(dest.as_ptr(), source.as_ptr()))
    }
}

pub fn copy_strings_bytes<'a>(source: &'a str, dest: &'a str, max_bytes: usize) -> &'a str {
    unsafe {
        ptr_to_str(strncpy(dest.as_ptr(), source.as_ptr(), max_bytes))
    }
}

pub fn concat_strings<'a>(source: &'a str, dest: &'a str) -> &'a str {
    unsafe {
        ptr_to_str(strcat(dest.as_ptr(), source.as_ptr()))
    }
}

pub fn concat_strings_bytes<'a>(source: &'a str, dest: &'a str, max_bytes: usize) -> &'a str {
    unsafe {
        ptr_to_str(strncat(dest.as_ptr(), source.as_ptr(), max_bytes))
    }
}

pub fn string_length(string: &str) -> usize {
    unsafe {
        strlen(string.as_ptr())
    }
}

unsafe fn ptr_to_str<'a>(ptr: *const u8) -> &'a str {
    let slc = core::slice::from_raw_parts(ptr, 5);
    core::str::from_utf8_unchecked(slc)
}