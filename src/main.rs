extern crate minifb;
use minifb::{Key, WindowOptions, Window};

mod chip8;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const TITLE: &str = "Chip 8 - ESC to exit";

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let window = Window::new(TITLE,WIDTH,HEIGHT,WindowOptions::default());
    let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

    while window.is_open() && !window.is_key_down(Key::Escape)
    {
        chip8::gpu::draw_in(&mut buffer);
        window.update_with_buffer(&buffer).unwrap();
    }
}