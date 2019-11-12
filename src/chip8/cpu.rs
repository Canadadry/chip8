use super::instruction::Instruction;

const MEMORY_SIZE:usize    =  4096;
const START_PC:u16         =  0x200;
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
	pc:u16,
	seed:u32,
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
			seed:0u32
		}
	}
	
	pub fn reset(&mut self)
	{
		for i in 0..self.m.len() { self.m[i] = 0u8; }
		for i in 0..self.v.len() { self.v[i] = 0u8; }
		for i in 0..self.s.len() { self.s[i] = 0u16; }
		
		self.i=0;
		self.sp = 0;
		self.dt = 0;
		self.st = 0;
		self.pc= START_PC;
	}

	pub fn load(&mut self,rom:&Vec<u8>)
	{
		self.reset();
		for i in 0..rom.len()
		{
			self.m[i + START_PC as usize] = rom[i];
		}			
	}
	
	pub fn step(&mut self)
	{
		let op = self.fetch();
		let it = Instruction::decode(op);
		self.execute(&it);

		if self.dt>0 { self.dt-=1; }
		if self.st>0 { self.st-=1; }
	}


	fn fetch(&mut self) -> u16
	{
		let msb    = self.m[self.pc as usize    ] as u16;
		let lsb    = self.m[self.pc as usize + 1] as u16;
		let opcode = msb << 8 | lsb;
		self.pc += 2;

		return opcode;
	}
	
	fn execute(&mut self, instruction:&Instruction) 
	{
		match instruction.op_1 {
			1=> {
				self.pc = instruction.op_234;
			},
			2 => {
				if self.sp >= 16 {panic!("Stack Overflow");}
				self.s[self.sp as usize] = self.pc;
				self.sp+=1;
			},
			3 => {
				if self.v[instruction.op_2 as usize] == instruction.op_34 {
					self.pc += 2;
				}
			},
			4 => {
				if self.v[instruction.op_2 as usize] != instruction.op_34 {
					self.pc += 2;
				}
			}, 
			5 => {
				if self.v[instruction.op_2 as usize] == self.v[instruction.op_3 as usize] {
					self.pc += 2;
				}
			}, 
			6 => {
				self.v[instruction.op_2 as usize] = instruction.op_34;
			}, 
			7 => {
				self.v[instruction.op_2 as usize] += instruction.op_34;
			}, 
			8 => {
				match instruction.op_4 {
					0 => {
						self.v[instruction.op_2 as usize] = self.v[instruction.op_3 as usize];
					},
					1 => {
						self.v[instruction.op_2 as usize] |= self.v[instruction.op_3 as usize];
					},
					2 => {
						self.v[instruction.op_2 as usize] &= self.v[instruction.op_3 as usize];
					},
					3 => {
						self.v[instruction.op_2 as usize] = self.v[instruction.op_3 as usize];
					},
					4 => {
						self.v[instruction.op_2 as usize] += self.v[instruction.op_3 as usize];
						// todo overflow and carry in vf
					},
					5 => {
						self.v[instruction.op_2 as usize] -= self.v[instruction.op_3 as usize];
						// todo overflow and carry in vf cf doc not borrow
					},
					6 => {
						self.v[instruction.op_2 as usize] <<= 1;
						// todo overflow and carry in vf
					},
					7 => {
						self.v[instruction.op_2 as usize] = self.v[instruction.op_3 as usize] - self.v[instruction.op_2 as usize];
						// todo overflow and carry in vf
					},
					0xE => {
						self.v[instruction.op_2 as usize] >>= 1;
						// todo overflow and carry in vf
					}
					_ => {
						panic!("Not opcode");
					}
				}
			},
			9 => {
				if self.v[instruction.op_2 as usize] != self.v[instruction.op_3 as usize] {
					self.pc += 2;
				}
			},
			0xA => {
				self.i = instruction.op_234;
			},
			0xB => {
				self.pc = instruction.op_234 + self.v[0] as u16;
			},
			0xC => {
				self.v[instruction.op_2 as usize] = self.rand() & instruction.op_34;
			}
			_ => {
				panic!("Not opcode");
			}
		}
	}

	fn rand(&mut self) -> u8
	{
	    self.seed = (self.seed * 1103515245 + 12345) % std::u32::MAX;
	    return (self.seed & 0xFFu32) as u8;
	}

}