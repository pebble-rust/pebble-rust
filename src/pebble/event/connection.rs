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

use crate::pebble::event::Event;
use crate::pebble::internal::functions::declarations::*;
pub use crate::pebble::internal::types::ConnectionHandlers;

pub struct ConnectionEvent;

impl ConnectionEvent {
    pub fn peek_app() -> Result<bool, i32> {
        unsafe {
            Ok(connection_service_peek_pebble_app_connection())
        }
    }

    pub fn peek_pebblekit() -> Result<bool, i32> {
        unsafe {
            Ok(connection_service_peek_pebblekit_connection())
        }
    }

    pub fn subscribe(handlers: ConnectionHandlers) {
        unsafe {
            connection_service_subscribe(handlers);
        }
    }
}

impl Event<bool> for ConnectionEvent {

    /// Do **NOT** use this. Use ConnectionEvent#subscribe instead.
    fn subscribe(handler: extern fn(bool)) {
        unimplemented!()
    }

    fn unsubscribe() {
        unsafe { connection_service_unsubscribe(); }
    }

    fn peek() -> Result<bool, i32> {
        ConnectionEvent::peek_app()
    }
}
