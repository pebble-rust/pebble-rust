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

use crate::pebble::internal::functions::declarations;

pub fn rand() -> i32 {
    unsafe {
        declarations::rand()
    }
}

pub fn seed_rand(seed: u32) -> i32 {
    unsafe {
        declarations::srand(seed)
    }
}

pub fn le_to_be_u32(num: u32) -> u32 {
    let b0 = num & 0x000000FF;
    let b1 = (num & 0x0000FF00) >> 8;
    let b2 = (num & 0x00FF0000) >> 16;
    let b3 = (num & 0xFF000000) >> 24;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

pub fn le_to_be_u16(num: u16) -> u16 {
    let b0 = num & 0x00FF;
    let b1 = (num & 0xFF00) >> 8;
    (b0 << 8) | b1
}

