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

use crate::pebble::internal::functions::interface;

/// Represents the application.
/// # Example
/// ```
/// use pebble::app::App;
///
/// let app = App::new();
/// app.run_event_loop();
/// ```
/// See: [Pebble SDK](https://developer.rebble.io/developer.pebble.com/docs/c/Foundation/App/index.html)
#[derive(Copy, Clone)]
pub struct App;

impl App {
    pub fn new() -> App {
        App {}
    }

    /// Runs the app event loop.
    /// See: [Pebble SDK](https://developer.rebble.io/developer.pebble.com/docs/c/Foundation/App/index.html#app_event_loop)
    pub fn run_event_loop(&self) {
        interface::app_event_loop();
    }
}