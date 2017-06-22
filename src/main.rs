extern crate piston_window;

use piston_window::*;

struct Field {
}
impl Field {
    fn new() -> Field {
        Field {
        }
    }
}

struct App {
    field: Field,
    x: isize,
    y: isize
}
impl App {
    fn new() -> App {
        App {
            field: Field::new(),
            x: 2,
            y: 0
        }
    }
    fn render<E>(&mut self, window: &mut PistonWindow, e: &E)
        where E: GenericEvent
    {
        window.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            rectangle([0.5, 0.5, 0.5, 1.0],
                    [0.0, 0.0, 32.0 * 6.0, 32.0 * 12.0],
                    c.transform, g);

            ellipse([1.0, 0.0, 0.0, 1.0], // red
                    [(self.x * 32) as f64, (self.y * 32) as f64, 32.0, 32.0],
                    c.transform, g);
        });
    }
    fn key_press(&mut self, args: &Button) {
            use piston_window::Button::Keyboard;
		    use piston_window::Key;
        if *args == Keyboard(Key::Left) {
            self.x -= 1;
        }
        if *args == Keyboard(Key::Right) {
            self.x += 1;
        }
        if *args == Keyboard(Key::Up) {
            self.y -= 1;
        }
        if *args == Keyboard(Key::Down) {
            self.y += 1;
        }
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Puyo", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut app = App::new();

    while let Some(e) = window.next() {
        if let Some(ref args) = e.render_args() {
            app.render(&mut window, &e);
        }

        if let Some(ref args) = e.update_args() {
           // TODO: only update if necessary
           // println!("update");
        }

        if let Some(ref args) = e.press_args() {
            // println!("press");
            app.key_press(args);
        }
    }
}