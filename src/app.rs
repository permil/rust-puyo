extern crate piston_window;

use piston_window::*;
use field::Field;

pub struct App {
    field: Field
}

impl App {
    pub fn new() -> App {
        App {
            field: Field::new()
        }
    }
    pub fn render<E>(&mut self, window: &mut PistonWindow, e: &E)
        where E: GenericEvent
    {
        window.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            rectangle([0.5, 0.5, 0.5, 1.0],
                    [0.0, 0.0, 32.0 * 6.0, 32.0 * 12.0],
                    c.transform, g);

            ellipse([1.0, 0.0, 0.0, 1.0], // red
                    [(self.field.x * 32) as f64, (self.field.y * 32) as f64, 32.0, 32.0],
                    c.transform, g);
        });
    }
    pub fn key_press(&mut self, args: &Button) {
            use piston_window::Button::Keyboard;
		    use piston_window::Key;
        if *args == Keyboard(Key::Left) {
            self.field.x -= 1;
        }
        if *args == Keyboard(Key::Right) {
            self.field.x += 1;
        }
        if *args == Keyboard(Key::Up) {
            self.field.y -= 1;
        }
        if *args == Keyboard(Key::Down) {
            self.field.y += 1;
        }
    }
}