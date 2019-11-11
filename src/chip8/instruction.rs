use super::cpu::Cpu;

pub struct Instruction
{
	op_1:u8,
	op_2:u8,
	op_3:u8,
	op_4:u8,       
	op_234:u16,
	op_34:u8,
}

impl Instruction{

	fn from_opcode(opcode:u16) -> Instruction
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
	fn runOn(&self, cpu:&mut CPU) 
	{
		match self.op_1 {
			1=> {
				cpu.pc = op_234;
			},
			2 => {
				if cpu.sp >= 16 {panic!("Stack Overflow");}
				cpu.s[cpu.sp] = cpu.pc;
				cpu.sp++;
			},
			3 => {
				if cpu.v[self.op_2] == self.op_34 {
					cpi.pc += 2;
				}
			},
			4 => {
				if cpu.v[self.op_2] != self.op_34 {
					cpi.pc += 2;
				}
			}, 
			5 => {
				if cpu.v[self.op_2] == cpu.v[self.op_3] {
					cpi.pc += 2;
				}
			}, 
			6 => {
				cpu.v[self.op_2] = self.op_34;
			}, 
			7 => {
				cpu.v[self.op_2] += self.op_34;
			}, 
			8 => {
				match self.op_4 {
					0 => {
						cpu.v[self.op_2] = cpu.v[self.op_3];
					},
					1 => {
						cpu.v[self.op_2] |= cpu.v[self.op_3];
					},
					2 => {
						cpu.v[self.op_2] &= cpu.v[self.op_3];
					},
					3 => {
						cpu.v[self.op_2] = cpu.v[self.op_3];
					},
					4 => {
						cpu.v[self.op_2] += cpu.v[self.op_3];
						// todo overflow and carry in vf
					},
					5 => {
						cpu.v[self.op_2] -= cpu.v[self.op_3];
						// todo overflow and carry in vf cf doc not borrow
					},
					6 => {
						cpu.v[self.op_2] <<= 1;
						// todo overflow and carry in vf
					},
					7 => {
						cpu.v[self.op_2] = cpu.v[self.op_3] - cpu.v[self.op_2];
						// todo overflow and carry in vf
					},
					0xE => {
						cpu.v[self.op_2] >>= 1;
						// todo overflow and carry in vf
					},
				},
				9 => {
					if cpu.v[self.op_2] != cpu.v[self.op_3] {
						cpi.pc += 2;
					}
				},
				0xA => {
					cpu.i = self.op_234;
				},
				0xB => {
					cpu.pc = self.op_234 +  cpu.v[0];
				},
				0xC => {
					cpu.v[self.op_2] = 0;
					// implement rand
				}
			}, 
		}
	}
}