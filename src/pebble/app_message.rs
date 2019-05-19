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

pub use crate::pebble::internal::types::Tuple;

use crate::pebble::internal::types::{self, DictionaryIterator, c_void, AppMessageResult};
use crate::pebble::internal::functions::declarations::*;
use crate::pebble::types::{DictPtr, VoidPtr};

/// Represents a `DictionaryIterator`, essentially a list of `Tuple`s.
/// # Usage
/// ```
/// use pebble_rust::app_message::Dictionary;
///
/// let dictionary = Dictionary::new();
/// let mut buffer: [u8; 256] = unimplemented!();
/// dictionary.init_write(&mut buffer);
///
/// dictionary.write_int(0, 2u32);
/// ```
pub struct Dictionary {
    internal: *mut DictionaryIterator
}

const NULL_TUPLE: *mut Tuple = 0 as *mut Tuple;

impl Dictionary {
    #[allow(clippy::cast_ptr_alignment)]
    pub fn new() -> Self {
        let null_ptr = core::ptr::null_mut();
        let mut iter = DictionaryIterator {
            end: null_ptr,
            cursor: null_ptr as *mut _,
            dict: null_ptr as *mut types::Dictionary
        };

        Self {
            internal: &mut iter as *mut DictionaryIterator
        }
    }

    /// Fetches the underlying dictionary from a raw pointer.
    pub fn from_raw(raw: DictPtr) -> Self {
        Self {
            internal: raw
        }
    }

    /// Prepares the dictionary for reading.
    /// Calling this is **required** after writing, before reading.
    pub fn init_read(&self, buffer: &mut [u8]) -> Option<Tuple> {
        unsafe {
            let ptr = dict_read_begin_from_buffer(self.internal, buffer.as_mut_ptr(),
                                             buffer.len() as u16);
            if ptr == NULL_TUPLE { None } else { Some(*ptr) }
        }
    }

    /// Prepares the dictionary for writing.
    /// You don't need to call this if you use `AppMessage`.
    pub fn init_write(&self, buffer: &mut [u8]) {
        unsafe {
            dict_write_begin(self.internal, buffer.as_mut_ptr(), buffer.len() as u16);
        }
    }

    /// Attempts to read the next `Tuple` in the dictionary.
    pub fn read_next(&self) -> Option<Tuple> {
        unsafe {
            let ptr = dict_read_next(self.internal);
            if ptr == NULL_TUPLE { None } else { Some(*ptr) }
        }
    }

    /// Resets the dictionary, and returns the first `Tuple`, if present.
    pub fn reset(&self) -> Option<Tuple> {
        unsafe {
            let ptr = dict_read_first(self.internal);
            if ptr == NULL_TUPLE { None } else { Some(*ptr) }
        }
    }

    /// Attempts to find a `Tuple` by its key.
    pub fn find(&self, key: u32) -> Option<Tuple> {
        unsafe {
            let ptr = dict_find(self.internal, key);
            if ptr == NULL_TUPLE { None } else { Some(*ptr) }
        }
    }

    pub fn write_string(&self, key: u32, string: &str) {
        unsafe { dict_write_cstring(self.internal, key, string.as_ptr()) };
    }

    pub fn prepare_for_read(&self) {
        unsafe { dict_write_end(self.internal) };
    }

    pub fn write_int<T: Integer>(&self, key: u32, int: T) {
        unsafe {
            let ptr = &int as *const T as *const c_void;
            dict_write_int(self.internal, key, ptr,
                           core::mem::size_of_val(&int) as u8, int.signed());
        }
    }
}

pub trait Integer {
    fn signed(&self) -> bool;
}

macro_rules! impl_signed {
    (for $($t:ty),+) => {
        $(impl Integer for $t {
            fn signed(&self) -> bool {
                true
            }
        })*
    }
}

macro_rules! impl_unsigned {
    (for $($t:ty),+) => {
        $(impl Integer for $t {
            fn signed(&self) -> bool {
                false
            }
        })*
    }
}

impl_signed!(for i32, i64, i8, i16, isize);
impl_unsigned!(for u32, u64, u8, u16, usize);

pub struct AppMessage;

impl AppMessage {
    pub fn open(size_inbound: u32, size_outbound: u32) {
        unsafe {
            app_message_open(size_inbound, size_outbound);
        }
    }

    pub fn register_inbox(callback: extern fn(dict: DictPtr, ctx: VoidPtr)) {
        unsafe {
            app_message_register_inbox_received(callback);
        }
    }

    pub fn register_inbox_drop(callback: extern fn(reason: AppMessageResult, ctx: VoidPtr)) {
        unsafe {
            app_message_register_inbox_dropped(callback);
        }
    }

    pub fn init_write(dictionary: &mut Dictionary) {
        unsafe {
            let internal = dictionary.internal;
            let ptr = internal as *mut *mut DictionaryIterator;
            app_message_outbox_begin(ptr);
            dictionary.internal = *ptr;
        }
    }

    pub fn send() {
        unsafe { app_message_outbox_send(); }
    }
}