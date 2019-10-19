use super::cpu::Cpu;

pub struct Instruction
{
	pub bit_mask:u16,
	pub id:u16,
	pub run: fn(&Instruction, &mut Cpu)
}

impl Instruction
{
	pub fn new(	bit_mask:u16, id:u16, run: fn(&Instruction, &mut Cpu)) -> Instruction
	{
		Instruction{
			bit_mask,
			id,
			run,
		}
	}
}

const _00E0_CLS  : Instruction =  Instruction::new(0xFFFF,0x00E0,|i,cpu| {});
const _00EE_RET  : Instruction =  Instruction::new(0xFFFF,0x00EE,|i,cpu| {});
const _0NNN_SYS  : Instruction =  Instruction::new(0xF000,0x0000,|i,cpu| {});
const _1NNN_JMP  : Instruction =  Instruction::new(0xF000,0x1000,|i,cpu| {});
const _2NNN_CALL : Instruction =  Instruction::new(0xF000,0x2000,|i,cpu| {});
const _3XNN_SE   : Instruction =  Instruction::new(0xF000,0x3000,|i,cpu| {});
const _4XNN_SNE  : Instruction =  Instruction::new(0xF000,0x4000,|i,cpu| {});
const _5XY0_SE   : Instruction =  Instruction::new(0xF00F,0x5000,|i,cpu| {});
const _6XNN_LD   : Instruction =  Instruction::new(0xF000,0x6000,|i,cpu| {});
const _7XNN_ADD  : Instruction =  Instruction::new(0xF000,0x7000,|i,cpu| {});
const _8XY0_LD   : Instruction =  Instruction::new(0xF00F,0x8000,|i,cpu| {});
const _8XY1_OR   : Instruction =  Instruction::new(0xF00F,0x8001,|i,cpu| {});
const _8XY2_AND  : Instruction =  Instruction::new(0xF00F,0x8002,|i,cpu| {});
const _8XY3_XOR  : Instruction =  Instruction::new(0xF00F,0x8003,|i,cpu| {});
const _8XY4_ADD  : Instruction =  Instruction::new(0xF00F,0x8004,|i,cpu| {});
const _8XY5_SUB  : Instruction =  Instruction::new(0xF00F,0x8005,|i,cpu| {});
const _8XY6_SHR  : Instruction =  Instruction::new(0xF00F,0x8006,|i,cpu| {});
const _8XY7_SUBN : Instruction =  Instruction::new(0xF00F,0x8007,|i,cpu| {});
const _8XYE_SHL  : Instruction =  Instruction::new(0xF00F,0x800E,|i,cpu| {});
const _9XY0_SNE  : Instruction =  Instruction::new(0xF00F,0x9000,|i,cpu| {});
const _ANNN_LD   : Instruction =  Instruction::new(0xF000,0xA000,|i,cpu| {});
const _BNNN_JMP  : Instruction =  Instruction::new(0xF000,0xB000,|i,cpu| {});
const _CXNN_RND  : Instruction =  Instruction::new(0xF000,0xC000,|i,cpu| {});
const _DXYN_DRW  : Instruction =  Instruction::new(0xF000,0xD000,|i,cpu| {});
const _EX9E_SKP  : Instruction =  Instruction::new(0xF0FF,0xE09E,|i,cpu| {});
const _EXA1_SKNP : Instruction =  Instruction::new(0xF0FF,0xE0A1,|i,cpu| {});
const _FX07_LD   : Instruction =  Instruction::new(0xF0FF,0xF007,|i,cpu| {});
const _FX0A_LD   : Instruction =  Instruction::new(0xF0FF,0xF00A,|i,cpu| {});
const _FX15_LD   : Instruction =  Instruction::new(0xF0FF,0xF015,|i,cpu| {});
const _FX18_LD   : Instruction =  Instruction::new(0xF0FF,0xF018,|i,cpu| {});
const _FX1E_ADD  : Instruction =  Instruction::new(0xF0FF,0xF01E,|i,cpu| {});
const _FX29_LD   : Instruction =  Instruction::new(0xF0FF,0xF029,|i,cpu| {});
const _FX33_LD   : Instruction =  Instruction::new(0xF0FF,0xF033,|i,cpu| {});
const _FX55_LD   : Instruction =  Instruction::new(0xF0FF,0xF055,|i,cpu| {});
const _FX65_LD   : Instruction =  Instruction::new(0xF0FF,0xF065,|i,cpu| {});



