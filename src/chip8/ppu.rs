use r_tui::sub_screen;
use r_tui::color_table;

const SCREEN_WIDTH:usize  = 64;
const SCREEN_HEIGHT:usize = 32;

pub struct Ppu{
	pixels: [bool; SCREEN_WIDTH*SCREEN_HEIGHT],
	pub updates: Vec<(usize,bool)>
}

impl Ppu{

	pub fn new() -> Ppu{
		Ppu{
			pixels:[false; SCREEN_WIDTH*SCREEN_HEIGHT],
			updates: vec![]
		}
	}

	pub fn draw_byte_at(&mut self,byte:u8,x:usize,y:usize) -> bool
	{
		if y>=SCREEN_HEIGHT { return false; }
		let pos:usize = x+y*SCREEN_WIDTH;
		let mut ret:bool = false;

		for i in 0..4
		{
			if (x+i)>=SCREEN_WIDTH {return false};
			let mask = 0x1<<(7-i);
			let last = self.pixels[pos+i];
			let new  = last ^ ((byte &  mask)==mask);
			self.pixels[pos+i] = new;
			self.update_pixel(x+(i as usize),y,new);

			ret = ret || ( last && new );
		}

		return ret;
	}

	pub fn clear(&mut self)
	{
		self.updates.clear();
		for i in 0..self.pixels.len()
		{
			self.pixels[i] = false;
			self.updates.push((i,false));	
		}	
	}

	fn update_pixel(&mut self,x:usize,y:usize,pixel:bool)
	{
		self.updates.push((x+y*SCREEN_WIDTH,pixel));	
	}	

	pub fn refresh(&mut self, sub:sub_screen::SubScreen, buf:&mut Vec<u32>,factor:usize)
	{
		for p in sub
		{
			let pos = p.0/factor+p.1/factor*SCREEN_WIDTH;
			buf[p.2] = if self.pixels[pos] {color_table::WHITE} else {color_table::BLACK}
		}

	}

}


