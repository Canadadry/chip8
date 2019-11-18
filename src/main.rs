use std::fs::File;
use std::io::Read;
use std::{thread, time};
use std::env;

extern crate minifb;
use minifb::{Key, KeyRepeat, WindowOptions, Window};

extern crate r_tui;
use r_tui::color_table;
use r_tui::screen;
use r_tui::view;


mod chip8;

const WIDTH: usize = 80;
const HEIGHT: usize = 40;
const FONT_SIZE: usize = 16;
const TITLE: &str = "Chip 8 - ESC to exit";
const FACTOR: usize = 6;

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

	let mut screen = screen::Screen::new(WIDTH,HEIGHT,FONT_SIZE,color_table::BLACK);
	let window = Window::new(TITLE,screen.real_width(),screen.real_height(),WindowOptions::default());
    let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

    let mut reg_view  = view::View::new(32,0,48,20);

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
			reg_view.apply(&mut screen);
            println!("{}",cpu);
        }
        if play {
            cpu.step(20);
            if cpu.need_refresh()
            {
	            cpu.refresh(screen.sub(0,64*FACTOR,0,32*FACTOR).unwrap(),screen.buffer_mut(),FACTOR);
            }
			reg_view.apply(&mut screen);
        }

        let delta = time::Instant::now() - start;
        if sixiteen_millis > delta {
            thread::sleep(sixiteen_millis-(delta));
        }


		if screen.is_dirty()
		{
			window.update_with_buffer(screen.buffer()).unwrap();
		}
		else
		{
			window.update();
		}
    }
}