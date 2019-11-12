use std::fs::File;
use std::io::Read;
use std::{thread, time};

extern crate minifb;
use minifb::{Key, KeyRepeat, WindowOptions, Window};

mod chip8;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const TITLE: &str = "Chip 8 - ESC to exit";

fn load_rom (filename: &str) -> std::io::Result<Vec<u8>>
{
	let mut file = try!(File::open(filename));

	let mut data = Vec::new();
	try!(file.read_to_end(&mut data));

	return Ok(data);
}

fn main() {
    let window = Window::new(TITLE,WIDTH,HEIGHT,WindowOptions::default());
    let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

    let mut cpu = chip8::cpu::Cpu::new(WIDTH,HEIGHT);
    cpu.reset();
    cpu.load(&load_rom("rom/MAZE.ch8").unwrap());

    let sixiteen_millis = time::Duration::from_millis(16);
    while window.is_open() && !window.is_key_down(Key::Escape)
    {
        let start = time::Instant::now();
        if window.is_key_pressed(Key::Space,KeyRepeat::No) {
            cpu.step(1);
        }
        let end = time::Instant::now();
        thread::sleep(sixiteen_millis-(end-start));
        
        window.update_with_buffer(cpu.ppu().buffer()).unwrap();
    }
}