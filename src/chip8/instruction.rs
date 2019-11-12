use std::fmt;

pub struct Instruction
{
	pub op_1:u8,
	pub op_2:u8,
	pub op_3:u8,
	pub op_4:u8,       
	pub op_234:u16,
	pub op_34:u8,
}

impl Instruction
{

	pub fn decode(opcode:u16) -> Instruction
	{
		Instruction{ 
			op_1   : ( ((opcode & 0xF000u16) >> 12)  as u8 ),
			op_2   : ( ((opcode & 0x0F00u16) >> 8 )  as u8 ),
			op_3   : ( ((opcode & 0x00F0u16) >> 4 )  as u8 ),
			op_4   : (  (opcode & 0x000Fu16)         as u8 ),
			op_234 : (  (opcode & 0x0FFFu16)               ),
			op_34  : (  (opcode & 0x00FFu16)         as u8 )
		}
	}
}


impl fmt::Display for Instruction {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "0x{:x}{:x}{:x}{:x}",  self.op_1,self.op_2,self.op_3,self.op_4)
    }
}
