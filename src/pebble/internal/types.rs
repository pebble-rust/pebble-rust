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

#![allow(non_camel_case_types)]

pub enum Window {}
pub enum Layer {}
pub enum TextLayer {}
pub enum ClickRecognizer {}
pub enum GBitmap {}
pub enum GContext {}
pub enum BitmapLayer {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm {
    pub tm_sec: u32,
    pub tm_min: u32,
    pub tm_hour: u32,
    pub tm_mday: u32,
    pub tm_mon: u32,
    pub tm_year: u32,
    pub tm_wday: u32,
    pub tm_yday: u32,
    pub tm_isdst: u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPoint {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSize {
    pub w: u16,
    pub h: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GRect {
    pub origin: GPoint,
    pub size: GSize,
}

#[repr(C)]
pub struct WindowHandlers {
    pub load: extern fn(*mut Window),
    pub appear: extern fn(*mut Window),
    pub disappear: extern fn(*mut Window),
    pub unload: extern fn(*mut Window),
}

#[repr(C)]
pub enum GCompOp {
    GCompOpAssign,
    GCompOpAssignInverted,
    GCompOpOr,
    GCompOpAnd,
    GCompOpClear,
    GCompOpSet
}

#[repr(C)]
pub enum GColor {
    GColorClear = -1,
    GColorBlack = 0,
    GColorWhite = 1
}

#[repr(C)]
pub enum TimeUnits {
    SECOND_UNIT=1,
    MINUTE_UNIT,
    HOUR_UNIT,
    DAY_UNIT,
    MONTH_UNIT,
    YEAR_UNIT
}

pub type ResHandle = c_void;

#[repr(C)]
pub struct FontInfo;

pub type GFont = *mut FontInfo;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

#[allow(non_camel_case_types)]
pub type c_char = u8;

#[repr(C, align(1))]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct Tuple {
    pub key: u32,
    #[bitfield(name = "t_type", ty = "u8", bits = "32..=39")]
    #[bitfield(name = "length", ty = "u16", bits = "40..=55")]
    pub t_type: [u8; 2],
    value: TupleValue
}

impl Tuple {
    unsafe fn read(&self) -> Option<TupleValue> {
        let ptr = (&self.key as *const u32) as usize;
        let value_ptr = ptr + 7;
        let t = self.t_type[0];
        match t {
            0 => {
                Some(TupleValue {
                    data: core::slice::from_raw_parts(value_ptr as *const u8,
                                                      self.t_type[1] as usize)
                })
            },
            1 => {
                Some(TupleValue {
                    cstring: core::slice::from_raw_parts(value_ptr as *const u8,
                                                         self.t_type[1] as usize)
                })
            },
            2 => {
                let value_ptr = value_ptr as *const u32;
                Some(TupleValue {
                    uint32: *value_ptr
                })
            },
            3 => {
                let value_ptr = value_ptr as *const i32;
                Some(TupleValue {
                    int32: *value_ptr
                })
            },
            _ => {None}
        }
    }

    pub fn get_string(&self) -> Option<&'static str> {
        unsafe {
            let opt = self.get_value();
            if let Some(opt) = opt {
                let cstr= opt.cstring;
                let str = core::str::from_utf8_unchecked(cstr);
                Some(str)
            } else {
                None
            }
        }
    }

    pub fn get_value(&self) -> Option<TupleValue> {
        unsafe {self.read()}
    }

}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union TupleValue {
    data: &'static [u8],
    cstring: &'static [u8],
    pub uint32: u32,
    pub int32: i32,

    // Unions are as large as the largest item.
    // No space is wasted though.
    placeholder: [u8; u8::max_value() as usize + 325usize]
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TupleType {
    BYTE_ARRAY, CSTRING, UINT, INT
}

#[repr(C)]
pub struct Dictionary;

#[repr(C)]
pub struct DictionaryIterator {
    pub dict: *mut Dictionary,
    pub end: *const c_void,
    pub cursor: *mut Tuple
}

#[repr(u8)]
pub enum DictionaryResult {
    DICT_OK, DICT_NOT_ENOUGH_STORAGE, DICT_INVALID_ARGS, DICT_INTERNAL_INCONSISTENCY,
    DICT_MALLOC_FAILED
}

#[repr(u8)]
pub enum AppMessageResult {
    OK, SEND_TIMEOUT, SEND_REJECTED, NOT_CONNECTED, NOT_RUNNING, INVALID_ARGS, BUSY, BUFFER_OVERFLOW,
    ALREADY_RELEASED, CALLBACK_ALREADY_REGISTERED, CALLBACK_NOT_REGISTERED, OUT_OF_MEMORY, CLOSED,
    INTERNAL_ERROR, INVALID_STATE
}

#[repr(C)]
pub struct BatteryChargeState {
    pub charge_percent: u8,
    pub is_charging: bool,
    pub is_plugged: bool
}

#[repr(C)]
pub struct ConnectionHandlers {
    pub app: extern fn(bool),
    pub pebblekit: extern fn(bool)
}