extern crate minifb;
use minifb::{Key, WindowOptions, Window};

use std::fs::File;
use std::io::Read;

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
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let window = Window::new(TITLE,WIDTH,HEIGHT,WindowOptions::default());
    let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

    let mut cpu = chip8::cpu::Cpu::new();
    cpu.reset();
    cpu.load(&load_rom("rom/MAZE.ch8").unwrap());

    while window.is_open() && !window.is_key_down(Key::Escape)
    {
    	cpu.step(4);
        cpu.ppu().draw_in(&mut buffer,WIDTH,HEIGHT);
        window.update_with_buffer(&buffer).unwrap();
    }
}