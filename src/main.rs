#![deny(clippy::all)]
extern crate minifb;

mod display;
mod vector;
mod world;

use display::Display;
use minifb::{Key, Window, WindowOptions};
use world::World;

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;

fn main() {
    let mut window = Window::new(
        "Test - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Create a new display that contains a buffer of pixels
    let mut display = Display::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).unwrap();
    // Create a new world
    let mut world = World::new().unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the world
        world.update();
        // Render the world
        world.render(&mut display);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(display.color_buffer(), WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
