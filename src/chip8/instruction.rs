
pub struct Arg
{
	pub pos:u8,
	pub size:u8,
	pub register:bool,
}

impl Arg
{
	pub fn new(pos:u8,size:u8,register:bool) -> Arg
	{
		Arg{
			pos,
			size,
			register
		}
	}

	pub fn _23f() -> Arg{ Arg::new(2,3,false) }
	pub fn _21t() -> Arg{ Arg::new(2,1,true ) }
	pub fn _11t() -> Arg{ Arg::new(1,1,true ) }
	pub fn _12t() -> Arg{ Arg::new(1,2,true ) }
	pub fn _12f() -> Arg{ Arg::new(1,2,false) }
	pub fn _01f() -> Arg{ Arg::new(0,1,false) }
}

pub struct Specification
{
	pub bit_mask:u16,
	pub id:u16,
	pub hexa_name:&'static str ,
	pub menmo_name:&'static str,
	pub args: Vec<Arg>
}

impl Specification
{
	pub fn new(	bit_mask:u16, id:u16, hexa_name:&'static str , menmo_name:&'static str, args: Vec<Arg>) -> Specification{
		Specification{
			bit_mask,
			id,
			hexa_name,
			menmo_name,
			args,
		}
	}
}

pub fn get_specifications() ->Vec<Specification> {
	vec!(
		Specification::new(0xFFFF,0x00E0," 00E0 "," CLS  ",vec!()),
		Specification::new(0xFFFF,0x00EE," 00EE "," RET  ",vec!()),
		Specification::new(0xF000,0x0000," 0NNN "," SYS  ",vec!( Arg::_23f() )),
		Specification::new(0xF000,0x1000," 1NNN "," JMP  ",vec!( Arg::_23f() )),
		Specification::new(0xF000,0x2000," 2NNN "," CALL ",vec!( Arg::_23f() )),
		Specification::new(0xF000,0x3000," 3XNN "," SE   ",vec!( Arg::_21t() ,Arg::_12f() )),
		Specification::new(0xF000,0x4000," 4XNN "," SNE  ",vec!( Arg::_21t() ,Arg::_12f() )),
		Specification::new(0xF00F,0x5000," 5XY0 "," SE   ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF000,0x6000," 6XNN "," LD   ",vec!( Arg::_21t() ,Arg::_12f() )),
		Specification::new(0xF000,0x7000," 7XNN "," ADD  ",vec!( Arg::_21t() ,Arg::_12f() )),
		Specification::new(0xF00F,0x8000," 8XY0 "," LD   ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x8001," 8XY1 "," OR   ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x8002," 8XY2 "," AND  ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x8003," 8XY3 "," XOR  ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x8004," 8XY4 "," ADD  ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x8005," 8XY5 "," SUB  ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x8006," 8XY6 "," SHR  ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x8007," 8XY7 "," SUBN ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x800E," 8XYE "," SHL  ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF00F,0x9000," 9XY0 "," SNE  ",vec!( Arg::_21t() ,Arg::_11t() )),
		Specification::new(0xF000,0xA000," ANNN "," LD   ",vec!( Arg::_23f() )),
		Specification::new(0xF000,0xB000," BNNN "," JMP  ",vec!( Arg::_23f() )),
		Specification::new(0xF000,0xC000," CXNN "," RND  ",vec!( Arg::_21t() ,Arg::_12f() )),
		Specification::new(0xF000,0xD000," DXYN "," DRW  ",vec!( Arg::_21t() ,Arg::_11t() , Arg::_01f() )),
		Specification::new(0xF0FF,0xE09E," EX9E "," SKP  ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xE0A1," EXA1 "," SKNP ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF007," FX07 "," LD   ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF00A," FX0A "," LD   ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF015," FX15 "," LD   ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF018," FX18 "," LD   ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF01E," FX1E "," ADD  ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF029," FX29 "," LD   ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF033," FX33 "," LD   ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF055," FX55 "," LD   ",vec!( Arg::_21t() )),
		Specification::new(0xF0FF,0xF065," FX65 "," LD   ",vec!( Arg::_21t() ))
	)
}

