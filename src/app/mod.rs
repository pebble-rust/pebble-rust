use crate::pebble::app;

pub fn main() {
    let app = app::App::new();
    app.run_event_loop();
}