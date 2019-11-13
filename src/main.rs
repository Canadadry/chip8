use std::fs::File;
use std::io::Read;
use std::{thread, time};
use std::env;

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
    let args: Vec<String> = env::args().collect();
    let romfile = args.get(1).unwrap_or_else(|| {panic!("You must specify a rom file tu load");});

    let mut cpu = chip8::cpu::Cpu::new(WIDTH,HEIGHT);
    cpu.reset();
    cpu.load(&load_rom(romfile).unwrap());

    let window = Window::new(TITLE,WIDTH,HEIGHT,WindowOptions::default());
    let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

    let mut play = false;
    let sixiteen_millis = time::Duration::from_millis(16);
    while window.is_open() && !window.is_key_down(Key::Escape)
    {
        let start = time::Instant::now();
        if window.is_key_pressed(Key::Enter,KeyRepeat::No) {
            play = !play;
        }
        if window.is_key_pressed(Key::Space,KeyRepeat::No) && !play {
            cpu.step(1);
            println!("{}",cpu);
        }
        if play {
            cpu.step(20);
        }
        let delta = time::Instant::now() - start;
        if sixiteen_millis > delta {
            thread::sleep(sixiteen_millis-(delta));
        }

        window.update_with_buffer(cpu.ppu().buffer()).unwrap();
    }
}