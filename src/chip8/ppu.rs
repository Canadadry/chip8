
mod color{
	pub const WHITE:u32 = 0xffffff;
	pub const BLACK:u32 = 0x000000;
}

const SCREEN_WIDTH:usize  = 64;
const SCREEN_HEIGHT:usize = 32;

pub struct Ppu{
	pixels: [bool; SCREEN_WIDTH*SCREEN_HEIGHT]
}

impl Ppu{

	pub fn new() -> Ppu{
		Ppu{
			pixels:[false; SCREEN_WIDTH*SCREEN_HEIGHT]
		}
	}

	pub fn draw_in(&self,buffer:&mut Vec<u32>,width:usize,height:usize)
	{
		for i in 0..buffer.len()
		{
			let pos_x        = i % width;
			let pos_y        = i/width;
			let mapped_pos_x = pos_x / width * SCREEN_WIDTH;
			let mapped_pos_y = pos_y /height * SCREEN_HEIGHT;
			let pos          = mapped_pos_x + mapped_pos_y * SCREEN_HEIGHT;

			buffer[i] = if self.pixels[pos] { color::WHITE } else { color::BLACK };
		}	
	}	

	pub fn clear(&mut self)
	{
		for i in 0..self.pixels.len()
		{
			self.pixels[i] = false;
		}		
	}
}


