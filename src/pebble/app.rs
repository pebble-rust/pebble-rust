use crate::pebble::internal::functions::interface;

#[derive(Copy, Clone)]
pub struct App;

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn run_event_loop(&self) {
        interface::app_event_loop();
    }
}