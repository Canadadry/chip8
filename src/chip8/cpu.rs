use super::instruction::Instruction;

const MEMORY_SIZE:usize =  4096;
const START_PC:u32  =  512;
const REGISTER_COUNT:usize    =  16;
const STACK_SIZE:usize    =  16;

pub struct Cpu 
{	
	memory:[u16; MEMORY_SIZE],
	register_v:[u16; REGISTER_COUNT],
	address_registerI:u32,
	stack:[u32; STACK_SIZE],
	stack_pointer:u16,
	delay_timer:u16,
	sound_timer:u16,
	program_counter:u32,
	// gpu:Gpu,
}

impl Cpu
{
	pub fn new() -> Cpu
	{	
		Cpu{
			memory:[0;MEMORY_SIZE],
			register_v:[0;REGISTER_COUNT],
			address_registerI:0u32,
			stack:[0;STACK_SIZE],
			stack_pointer:0u16,
			delay_timer:0u16,
			sound_timer:0u16,
			program_counter:0u32,
			// gpu:gpu
		}
	}
	
	
	pub fn reset(&mut self)
	{
		for i in 0..self.memory.len()     { self.memory[i]     = 0u16; }
		for i in 0..self.register_v.len() { self.register_v[i] = 0u16; }
		for i in 0..self.stack.len()      { self.stack[i]      = 0u32; }
		
		self.address_registerI=0;
		self.stack_pointer = 0;
		self.delay_timer = 0;
		self.sound_timer = 0;
		self.program_counter= START_PC;

		// self.myScreen.reset();
	}
	
	pub fn step_timer(&mut self)
	{
		if self.delay_timer>0 { self.delay_timer-=1; }
		if self.sound_timer>0 { self.sound_timer-=1; }
	}
	
	// pub fn get_next_instruction(&self) -> Instruction
	// {
	// 	// self.specification.from(self.get_opcode())
	// }
	
	pub fn get_opcode(&self) 
	{
		(self.memory[self.program_counter]<<8) as u32 + self.memory[self.program_counter+1]  as u32;
	}
	
	pub fn load(&mut self,rom:&Vec<u8>)
	{
		self.reset();
		for i in 0..rom.len()/2
		{
			self.memory[i] = rom[2*i] << 8 + rom[2*i+1]
		}			
	}
	
}