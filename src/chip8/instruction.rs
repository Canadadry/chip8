
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

pub struct Instruction
{
	pub bit_mask:u16,
	pub id:u16,
	pub hexa_name:&'static str ,
	pub menmo_name:&'static str,
	pub args: Vec<Arg>
}

impl Instruction
{
	pub fn new(	bit_mask:u16, id:u16, hexa_name:&'static str , menmo_name:&'static str, args: Vec<Arg>) -> Instruction
	{
		Instruction{
			bit_mask,
			id,
			hexa_name,
			menmo_name,
			args,
		}
	}
}

pub enum InstructionKey
{
	_00E0 =  0,
	_00EE =  1,
	_0NNN =  2,
	_1NNN =  3,
	_2NNN =  4,
	_3XNN =  5,
	_4XNN =  6,
	_5XY0 =  7,
	_6XNN =  8,
	_7XNN =  9,
	_8XY0 = 10,
	_8XY1 = 11,
	_8XY2 = 12,
	_8XY3 = 13,
	_8XY4 = 14,
	_8XY5 = 15,
	_8XY6 = 16,
	_8XY7 = 17,
	_8XYE = 18,
	_9XY0 = 19,
	_ANNN = 20,
	_BNNN = 21,
	_CXNN = 22,
	_DXYN = 23,
	_EX9E = 24,
	_EXA1 = 25,
	_FX07 = 26,
	_FX0A = 27,
	_FX15 = 28,
	_FX18 = 29,
	_FX1E = 30,
	_FX29 = 31,
	_FX33 = 32,
	_FX55 = 33,
	_FX65 = 34
}

pub struct Specification
{
	pub list:Vec<Instruction>
}


impl Specification
{
	pub fn new() -> Specification {
		Specification
		{
			list:vec!(
				Instruction::new(0xFFFF,0x00E0," 00E0 "," CLS  ",vec!()),
				Instruction::new(0xFFFF,0x00EE," 00EE "," RET  ",vec!()),
				Instruction::new(0xF000,0x0000," 0NNN "," SYS  ",vec!( Arg::_23f() )),
				Instruction::new(0xF000,0x1000," 1NNN "," JMP  ",vec!( Arg::_23f() )),
				Instruction::new(0xF000,0x2000," 2NNN "," CALL ",vec!( Arg::_23f() )),
				Instruction::new(0xF000,0x3000," 3XNN "," SE   ",vec!( Arg::_21t() ,Arg::_12f() )),
				Instruction::new(0xF000,0x4000," 4XNN "," SNE  ",vec!( Arg::_21t() ,Arg::_12f() )),
				Instruction::new(0xF00F,0x5000," 5XY0 "," SE   ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF000,0x6000," 6XNN "," LD   ",vec!( Arg::_21t() ,Arg::_12f() )),
				Instruction::new(0xF000,0x7000," 7XNN "," ADD  ",vec!( Arg::_21t() ,Arg::_12f() )),
				Instruction::new(0xF00F,0x8000," 8XY0 "," LD   ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x8001," 8XY1 "," OR   ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x8002," 8XY2 "," AND  ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x8003," 8XY3 "," XOR  ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x8004," 8XY4 "," ADD  ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x8005," 8XY5 "," SUB  ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x8006," 8XY6 "," SHR  ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x8007," 8XY7 "," SUBN ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x800E," 8XYE "," SHL  ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF00F,0x9000," 9XY0 "," SNE  ",vec!( Arg::_21t() ,Arg::_11t() )),
				Instruction::new(0xF000,0xA000," ANNN "," LD   ",vec!( Arg::_23f() )),
				Instruction::new(0xF000,0xB000," BNNN "," JMP  ",vec!( Arg::_23f() )),
				Instruction::new(0xF000,0xC000," CXNN "," RND  ",vec!( Arg::_21t() ,Arg::_12f() )),
				Instruction::new(0xF000,0xD000," DXYN "," DRW  ",vec!( Arg::_21t() ,Arg::_11t() , Arg::_01f() )),
				Instruction::new(0xF0FF,0xE09E," EX9E "," SKP  ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xE0A1," EXA1 "," SKNP ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF007," FX07 "," LD   ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF00A," FX0A "," LD   ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF015," FX15 "," LD   ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF018," FX18 "," LD   ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF01E," FX1E "," ADD  ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF029," FX29 "," LD   ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF033," FX33 "," LD   ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF055," FX55 "," LD   ",vec!( Arg::_21t() )),
				Instruction::new(0xF0FF,0xF065," FX65 "," LD   ",vec!( Arg::_21t() ))
			)
		}
	}

	pub fn opcodeIsAction(&self,opcode:u16, action:InstructionKey) -> bool
	{
		if let Some(i) = self.list.get(action as usize)
		{
			return i.bit_mask & opcode == i.id;
		}
		return false;
	}
	
	pub fn getSubByte(bytes:u16, num:u8) -> u16
	{
		return (bytes & ((0xF)<<(num*4)))>>(num*4);
	}
	
	// pub fn getSubValue(bytes:u16, pos:u8,size:u8) -> &str
	// {
	// 	// String value = "";
	// 	// for(int i=0;i<size;i++)
	// 	// {
	// 	// 	value+= String.format("%x", getSubByte(bytes, pos-i));
	// 	// }
	// 	// return value;
	// }
	
	// pub fn decode(opcode:u16) -> std::string::String
	// {
	// 	boolean opcodefound = false;
	// 	String ligne = "" + String.format("%04x", opcode) + " : ";
	// 	for(int i=0;i<Instruction.lenght;i++)
	// 	{
	// 		if(opcodeIsAction(opcode, i))
	// 		{
	// 			opcodefound = true;
	// 			ligne += instructionSet[i].MenmoName;
	// 			for(int j = 0;j<instructionSet[i].argTab.length;j++)
	// 			{
	// 				if(instructionSet[i].argTab[j].register)
	// 				{
	// 					ligne+="V";
	// 				}
	// 				else
	// 				{
	// 					ligne+="0x";
	// 				}
	// 				ligne += getSubValue(opcode, instructionSet[i].argTab[j].pos,instructionSet[i].argTab[j].size);
	// 				if((j+1)<instructionSet[i].argTab.length)
	// 				{
	// 					ligne += " , ";
	// 				}
	// 			}
	// 		}
	// 	}
		
	// 	if(opcodefound == false)
	// 	{
	// 		ligne += "NOP";
	// 	}

	// 	return ligne;

	// }
}



