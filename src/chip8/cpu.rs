use super::instruction::Instruction;

const MEMORY_SIZE:usize    =  4096;
const START_PC:u32         =  0x200;
const REGISTER_COUNT:usize =  16;
const STACK_SIZE:usize     =  16;

pub struct Cpu 
{	
	m:[u8; MEMORY_SIZE],
	v:[u8; REGISTER_COUNT],
	i:u16,
	s:[u16; STACK_SIZE],
	sp:u8,
	dt:u8,
	st:u8,
	pc:u16
}

impl Cpu
{
	pub fn new() -> Cpu
	{	
		Cpu{
			m:[0;MEMORY_SIZE],
			v:[0;REGISTER_COUNT],
			i:0u16,
			s:[0;STACK_SIZE],
			sp:0u8,
			dt:0u8,
			st:0u8,
			pc:0u16,
		}
	}
	
	pub fn reset(&mut self)
	{
		for i in 0..self.m.len() { self.m[i] = 0u16; }
		for i in 0..self.c.len() { self.c[i] = 0u16; }
		for i in 0..self.s.len() { self.s[i] = 0u32; }
		
		self.i=0;
		self.sp = 0;
		self.dt = 0;
		self.st = 0;
		self.pc= START_PC;
	}
	
	pub fn step_timer(&mut self)
	{
		if self.dt>0 { self.dt-=1; }
		if self.st>0 { self.st-=1; }
	}
	
	pub fn get_opcode(&self) -> u32
	{
		return  ((self.m[self.pc as usize    ] as u32 ) << 16) 
		       | (self.m[self.pc as usize + 1]  as u32);
	}
	
	pub fn load(&mut self,rom:&Vec<u8>)
	{
		self.reset();
		for i in 0..rom.len()/2
		{
			self.m[i] = (rom[2*i] as u16) << 8 | rom[2*i+1] as u16;
		}			
	}

	pub fn fetch() -> u16
	{

	}

	pub fn decode(opcode:u32) -> Instruction
	{
		Instruction{ 
			op_1   : (opcode & 0xF000) >> 12,
			op_2   : (opcode & 0x0F00) >> 8,
			op_3   : (opcode & 0x00F0) >> 4,
			op_4   :  opcode & 0x000F,
			op_234 :  opcode & 0x0FFF,
			op_34  : (opcode & 0x00FF)
    	}
	}
	
}