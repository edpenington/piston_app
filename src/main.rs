// Imports
extern crate piston;
extern crate piston_window;

use piston_window::*;
use piston::*;

pub type Vec2 = (f64, f64);

pub struct App {
    window: PistonWindow,
    should_quit: bool,
}

impl App {
    //Constructors
    fn new() -> App {
        App {
            window:  WindowSettings::new("Hello Piston!", (640, 480))
                .exit_on_esc(true)
                .build()
                .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) }),
            should_quit: false,
        }
    }

    fn run_loop(&mut self) -> () {
        while let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |_c, g| {
                clear([0.5, 1.0, 0.5, 1.0], g);
            });
        }
    }
}

fn main() {
    let mut app: App = App::new();
    app.run_loop();
}
