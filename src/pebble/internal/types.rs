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