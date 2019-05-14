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

use crate::pebble::internal::types::{GFont, ResHandle};
use crate::pebble::internal::functions::declarations::*;

const FONT_KEY_GOTHIC_18_BOLD: &'static str = "RESOURCE_ID_GOTHIC_18_BOLD";
const FONT_KEY_GOTHIC_24: &'static str = "RESOURCE_ID_GOTHIC_24";
const FONT_KEY_GOTHIC_09: &'static str = "RESOURCE_ID_GOTHIC_09";
const FONT_KEY_GOTHIC_14: &'static str = "RESOURCE_ID_GOTHIC_14";
const FONT_KEY_GOTHIC_14_BOLD: &'static str = "RESOURCE_ID_GOTHIC_14_BOLD";
const FONT_KEY_GOTHIC_18: &'static str = "RESOURCE_ID_GOTHIC_18";
const FONT_KEY_GOTHIC_24_BOLD: &'static str = "RESOURCE_ID_GOTHIC_24_BOLD";
const FONT_KEY_GOTHIC_28: &'static str = "RESOURCE_ID_GOTHIC_28";
const FONT_KEY_GOTHIC_28_BOLD: &'static str = "RESOURCE_ID_GOTHIC_28_BOLD";
const FONT_KEY_BITHAM_30_BLACK: &'static str = "RESOURCE_ID_BITHAM_30_BLACK";
const FONT_KEY_BITHAM_42_BOLD: &'static str = "RESOURCE_ID_BITHAM_42_BOLD";
const FONT_KEY_BITHAM_42_LIGHT: &'static str = "RESOURCE_ID_BITHAM_42_LIGHT";
const FONT_KEY_BITHAM_42_MEDIUM_NUMBERS: &'static str = "RESOURCE_ID_BITHAM_42_MEDIUM_NUMBERS";
const FONT_KEY_BITHAM_34_MEDIUM_NUMBERS: &'static str = "RESOURCE_ID_BITHAM_34_MEDIUM_NUMBERS";
const FONT_KEY_BITHAM_34_LIGHT_SUBSET: &'static str = "RESOURCE_ID_BITHAM_34_LIGHT_SUBSET";
const FONT_KEY_BITHAM_18_LIGHT_SUBSET: &'static str = "RESOURCE_ID_BITHAM_18_LIGHT_SUBSET";
const FONT_KEY_DROID_SERIF_28_BOLD: &'static str = "RESOURCE_ID_DROID_SERIF_28_BOLD";
const FONT_KEY_LECO_20_BOLD_NUMBERS: &'static str = "RESOURCE_ID_LECO_20_BOLD_NUMBERS";
const FONT_KEY_LECO_26_BOLD_NUMBERS_AM_PM: &'static str = "RESOURCE_ID_LECO_26_BOLD_NUMBERS_AM_PM";
const FONT_KEY_LECO_32_BOLD_NUMBERS: &'static str = "RESOURCE_ID_LECO_32_BOLD_NUMBERS";
const FONT_KEY_LECO_36_BOLD_NUMBERS: &'static str = "RESOURCE_ID_LECO_36_BOLD_NUMBERS";
const FONT_KEY_LECO_38_BOLD_NUMBERS: &'static str = "RESOURCE_ID_LECO_38_BOLD_NUMBERS";
const FONT_KEY_LECO_42_BOLD_NUMBERS: &'static str = "RESOURCE_ID_LECO_42_BOLD_NUMBERS";
const FONT_KEY_LECO_28_LIGHT_NUMBERS: &'static str = "RESOURCE_ID_LECO_28_LIGHT_NUMBERS";


pub struct Font {
    pub internal: GFont
}

impl Font {
    pub fn get_system(resource_id: &str) -> Self {
        unsafe {
            let internal = fonts_get_system_font(resource_id.as_ptr());
            Self {internal}
        }
    }

    pub fn get_custom_from_handle(res_handle: ResHandle) -> Self {
        unsafe {
            let internal = fonts_load_custom_font(res_handle);
            Self {internal}
        }
    }

    pub fn get_custom(resource_id: u32) -> Self {
        unsafe {
            let res_handle = resource_get_handle(resource_id);
            Self::get_custom_from_handle(res_handle)
        }
    }
}
