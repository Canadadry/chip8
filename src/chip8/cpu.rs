use std::fmt;

use super::instruction::Instruction;
use super::ppu::Ppu;
use super::font;

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

	ppu:Ppu
}

impl Cpu
{
	pub fn new(width:usize,height:usize) -> Cpu
	{	
		let mut cpu = Cpu{
			m:[0;MEMORY_SIZE],
			v:[0;REGISTER_COUNT],
			i:0u16,
			s:[0;STACK_SIZE],
			sp:0u8,
			dt:0u8,
			st:0u8,
			pc:0u16,
			ppu: Ppu::new()
		};
		font::load_at(&mut cpu.m,0);
		cpu.ppu.attach_screen(width,height);
		return cpu;
	}
	
	pub fn reset(&mut self)
	{
		for i in 0..self.m.len() { self.m[i] = 0u8; }
		for i in 0..self.v.len() { self.v[i] = 0u8; }
		for i in 0..self.s.len() { self.s[i] = 0u16; }
		
		font::load_at(&mut self.m,0);

		self.i=0;
		self.sp = 0;
		self.dt = 0;
		self.st = 0;
		self.pc= START_PC;

		self.ppu.clear();
	}

	pub fn load(&mut self,rom:&Vec<u8>)
	{
		self.reset();
		for i in 0..rom.len()
		{
			self.m[i + START_PC as usize] = rom[i];
		}			
	}

	pub fn ppu(&self) -> &Ppu
	{
		return &self.ppu;
	}
	
	pub fn step(&mut self,cycle:usize)
	{
		for _i in 0..cycle
		{
			let op = self.fetch();
			let it = Instruction::decode(op);
			self.execute(&it);

			// println!("executing {}",it);

			if self.dt>0 { self.dt-=1; }
			if self.st>0 { self.st-=1; }			
		}
	}


	fn fetch(&mut self) -> u16
	{
		let msb    = self.m[self.pc as usize    ] as u16;
		let lsb    = self.m[self.pc as usize + 1] as u16;
		let opcode = msb << 8 | lsb;
		self.pc += 2;

		return opcode;
	}
	
	fn execute(&mut self, inst:&Instruction) 
	{
		match inst.op_1 {
			0=>{
				match inst.op_234 {
					0x0E0 => {
						self.ppu.clear();
					}
					_ => {
						panic!("Not opcode {}",inst);
					}
				}
			},
			1=> {
				self.pc = inst.op_234;
			},
			2 => {
				if self.sp >= 16 {panic!("Stack Overflow");}
				self.s[self.sp as usize] = self.pc;
				self.sp+=1;
			},
			3 => {
				if self.v[inst.op_2 as usize] == inst.op_34 {
					self.pc += 2;
				}
			},
			4 => {
				if self.v[inst.op_2 as usize] != inst.op_34 {
					self.pc += 2;
				}
			}, 
			5 => {
				if self.v[inst.op_2 as usize] == self.v[inst.op_3 as usize] {
					self.pc += 2;
				}
			}, 
			6 => {
				self.v[inst.op_2 as usize] = inst.op_34;
			}, 
			7 => {
				self.v[inst.op_2 as usize] += inst.op_34;
			}, 
			8 => {
				match inst.op_4 {
					0 => {
						self.v[inst.op_2 as usize] = self.v[inst.op_3 as usize];
					},
					1 => {
						self.v[inst.op_2 as usize] |= self.v[inst.op_3 as usize];
					},
					2 => {
						self.v[inst.op_2 as usize] &= self.v[inst.op_3 as usize];
					},
					3 => {
						self.v[inst.op_2 as usize] = self.v[inst.op_3 as usize];
					},
					4 => {
						let vx =  self.v[inst.op_2 as usize] as u16;
						let vy =  self.v[inst.op_3 as usize] as u16; 
						let result = vx + vy;
						self.v[inst.op_2 as usize] = result as u8;
						self.v[0xF] =  if result > 0xFF { 1 } else { 0 };
					},
					5 => {
						self.v[0xF] = if self.v[inst.op_2 as usize] > self.v[inst.op_3 as usize] { 1 } else { 0 };
						self.v[inst.op_2 as usize] = self.v[inst.op_2 as usize].wrapping_sub(self.v[inst.op_3 as usize]);
					},
					6 => {
						let vx = self.v[inst.op_2 as usize];
						self.v[0xF] =  if vx & 0x1 == 0x1 { 1 } else { 0 };
						self.v[inst.op_2 as usize] = vx >> 1;
					},
					7 => {
						self.v[0xF] = if self.v[inst.op_2 as usize] < self.v[inst.op_3 as usize] { 1 } else { 0 };
						self.v[inst.op_2 as usize] = self.v[inst.op_3 as usize].wrapping_sub(self.v[inst.op_2 as usize]);
					},
					0xE => {

						let vx = self.v[inst.op_2 as usize] as u16;
						let result  = vx << 1; 
						self.v[inst.op_2 as usize] = result as u8;
						self.v[0xF] =  if result > 0xFF { 1 } else { 0 };
					}
					_ => {
						panic!("Not opcode {}",inst);
					}
				}
			},
			9 => {
				if self.v[inst.op_2 as usize] != self.v[inst.op_3 as usize] {
					self.pc += 2;
				}
			},
			0xA => {
				self.i = inst.op_234;
			},
			0xB => {
				self.pc = inst.op_234 + self.v[0] as u16;
			},
			0xC => {
				self.v[inst.op_2 as usize] = rand::random::<u8>() & inst.op_34;
			},
			0xD => {
				let x = self.v[inst.op_2 as usize] as usize;
				let y = self.v[inst.op_3 as usize] as usize;
				let mut collide:bool = false;
				for i in 0..inst.op_4
				{
					let byte:u8 = self.m[(self.i+i as u16) as usize];
					collide = collide || self.ppu.draw_byte_at(byte,x,y+(i as usize));
				}
				self.v[0xF as usize] = if collide { 1u8 }  else { 0u8 };
			},
			0xF => {
				match inst.op_34 {
					0x29 => {
						self.i = (self.v[inst.op_2 as usize] as u16) * 5;
					},
					_ => {
						panic!("Not opcode {}",inst);
					}
				}
			},
			_ => {
				panic!("Not opcode {}",inst);
			}
		}
	}
}


impl fmt::Display for Cpu {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "V0:0x{:x} V1:0x{:x} V2:0x{:x} V3:0x{:x} \n", self.v[0] ,self.v[1] ,self.v[2] ,self.v[3] ).unwrap();
        write!(f, "V4:0x{:x} V5:0x{:x} V6:0x{:x} V7:0x{:x} \n", self.v[4] ,self.v[5] ,self.v[6] ,self.v[7] ).unwrap();
        write!(f, "V8:0x{:x} V9:0x{:x} VA:0x{:x} VB:0x{:x} \n", self.v[8] ,self.v[9] ,self.v[10],self.v[8] ).unwrap();
        write!(f, "VC:0x{:x} VD:0x{:x} VE:0x{:x} VF:0x{:x} \n", self.v[12],self.v[13],self.v[14],self.v[15]).unwrap();

        write!(f, "I:0x{:x} DT:{:x} ST:{:x} PC:0x{:x} SP:{:x} \n", self.i,self.dt,self.st, self.pc,self.sp).unwrap();

        write!(f, "S0:0x{:x} S1:0x{:x} S2:0x{:x} S3:0x{:x} \n", self.s[0] ,self.s[1] ,self.s[2] ,self.s[3] ).unwrap();
        write!(f, "S4:0x{:x} S5:0x{:x} S6:0x{:x} S7:0x{:x} \n", self.s[4] ,self.s[5] ,self.s[6] ,self.s[7] ).unwrap();
        write!(f, "S8:0x{:x} S9:0x{:x} SA:0x{:x} SB:0x{:x} \n", self.s[8] ,self.s[9] ,self.s[10],self.s[8] ).unwrap();
        write!(f, "SC:0x{:x} SD:0x{:x} SE:0x{:x} SF:0x{:x} \n", self.s[12],self.s[13],self.s[14],self.s[15]).unwrap();

        return Ok(());
    }
}
