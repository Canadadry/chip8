
mod color{
	pub const WHITE:u32 = 0xffffff;
	pub const BLACK:u32 = 0x000000;
}

const SCREEN_WIDTH:usize  = 64;
const SCREEN_HEIGHT:usize = 32;

pub struct Ppu{
	pixels: [bool; SCREEN_WIDTH*SCREEN_HEIGHT],
	real_width:usize,
	real_height:usize,
	pixel_width:usize,
	pixel_height:usize,
	buffer:Vec<u32>
}

impl Ppu{

	pub fn new() -> Ppu{
		Ppu{
			pixels:[false; SCREEN_WIDTH*SCREEN_HEIGHT],
			real_width:0,
			real_height:0,
			pixel_width:0,
			pixel_height:0,
			buffer: vec![]
		}
	}

	pub fn draw_byte_at(&mut self,byte:u8,x:usize,y:usize) -> bool
	{
		if y>=SCREEN_HEIGHT { return false; }
		let pos:usize = x+y*SCREEN_WIDTH;
		let mut ret:bool = false;

		for i in 0..8
		{
			if (x+i)>=SCREEN_WIDTH {return false};
			let mask = 0x1<<(7-i);
			let last = self.pixels[pos+i];
			let new  = last ^ ((byte &  mask)==mask);
			self.pixels[pos+i] = new;
			self.update_pixel(x,y,new);

			ret = ret || ( last && new );
		}

		return ret;
	}

	pub fn clear(&mut self)
	{
		for i in 0..self.pixels.len()
		{
			self.pixels[i] = false;
		}		
		for i in 0..self.buffer.len()
		{
			self.buffer[i] = color::BLACK;
		}
	}

	pub fn attach_screen(&mut self,width:usize,height:usize)
	{
		self.real_width   = width;
		self.real_height  = height;
		self.pixel_width  = width  / SCREEN_WIDTH;
		self.pixel_height = height / SCREEN_HEIGHT;

		self.buffer.resize(width*height,color::BLACK);
	}

	pub fn buffer(&self) -> &Vec<u32>
	{
		return &self.buffer;
	}

	fn update_pixel(&mut self,x:usize,y:usize,pixel:bool)
	{
		let start_x = x*self.pixel_width;
		let start_y = y*self.pixel_height;

		for i in 0..self.pixel_width
		{
			for j in 0..self.pixel_width
			{
				let pos_x = i + start_x;
				let pos_y = y + start_y;
				let pos   = pos_x + pos_y * self.real_width;

				self.buffer[pos] = if pixel { color::WHITE } else { color::BLACK };
			}
		}	
	}	

}


