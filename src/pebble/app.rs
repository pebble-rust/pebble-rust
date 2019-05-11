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