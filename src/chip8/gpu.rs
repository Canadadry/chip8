
mod color{
	pub const WHITE:u32 = 0xffffff;
	pub const BLACK:u32 = 0x000000;
}

pub fn draw_in(buffer:& mut Vec<u32>)
{
	for i in buffer.iter_mut() 
	{
		*i = color::WHITE
	}

}