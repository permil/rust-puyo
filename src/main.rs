extern crate piston_window;

use piston_window::*;

mod app;
mod field;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Puyo", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut app = app::App::new();

    while let Some(e) = window.next() {
        if let Some(ref args) = e.render_args() {
            app.render(&mut window, &e);
        }

        if let Some(ref args) = e.update_args() {
           // TODO: only update if necessary
           // println!("update");
            app.update(args);
        }

        if let Some(ref args) = e.press_args() {
            // println!("press");
            app.key_press(args);
        }
    }
}