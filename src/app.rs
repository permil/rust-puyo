extern crate piston_window;

use piston_window::*;
use field::Field;

pub struct App {
    field: Field,
    dt: f64
}

impl App {
    pub fn new() -> App {
        App {
            field: Field::new(),
            dt: 0.0
        }
    }
    pub fn render<E>(&mut self, window: &mut PistonWindow, e: &E)
        where E: GenericEvent
    {
        window.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            self.field.render(c, g);
        });
    }
    pub fn key_press(&mut self, args: &Button) {
            use piston_window::Button::Keyboard;
		    use piston_window::Key;
        if *args == Keyboard(Key::Left) {
            self.field.left();
        }
        if *args == Keyboard(Key::Right) {
            self.field.right();
        }
        if *args == Keyboard(Key::Down) {
            self.field.down();
        }
    }
    pub fn update(&mut self, args: &UpdateArgs) {
        self.dt += args.dt;
        if self.dt >= 1.0 / 60.0 {
            self.dt -= 1.0 / 60.0;
            self.field.step();
        }
    }
}
