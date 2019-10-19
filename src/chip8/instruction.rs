use std::collections::HashMap;
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

	pub fn load() -> HashMap<&'static str,Instruction>
	{
		let mut map = HashMap::new();
		map.insert("00E0_CLS"  , Instruction::new(0xFFFF,0x00E0,|i,cpu| {}) );
		map.insert("00EE_RET"  , Instruction::new(0xFFFF,0x00EE,|i,cpu| {}) );
		map.insert("0NNN_SYS"  , Instruction::new(0xF000,0x0000,|i,cpu| {}) );
		map.insert("1NNN_JMP"  , Instruction::new(0xF000,0x1000,|i,cpu| {}) );
		map.insert("2NNN_CALL" , Instruction::new(0xF000,0x2000,|i,cpu| {}) );
		map.insert("3XNN_SE"   , Instruction::new(0xF000,0x3000,|i,cpu| {}) );
		map.insert("4XNN_SNE"  , Instruction::new(0xF000,0x4000,|i,cpu| {}) );
		map.insert("5XY0_SE"   , Instruction::new(0xF00F,0x5000,|i,cpu| {}) );
		map.insert("6XNN_LD"   , Instruction::new(0xF000,0x6000,|i,cpu| {}) );
		map.insert("7XNN_ADD"  , Instruction::new(0xF000,0x7000,|i,cpu| {}) );
		map.insert("8XY0_LD"   , Instruction::new(0xF00F,0x8000,|i,cpu| {}) );
		map.insert("8XY1_OR"   , Instruction::new(0xF00F,0x8001,|i,cpu| {}) );
		map.insert("8XY2_AND"  , Instruction::new(0xF00F,0x8002,|i,cpu| {}) );
		map.insert("8XY3_XOR"  , Instruction::new(0xF00F,0x8003,|i,cpu| {}) );
		map.insert("8XY4_ADD"  , Instruction::new(0xF00F,0x8004,|i,cpu| {}) );
		map.insert("8XY5_SUB"  , Instruction::new(0xF00F,0x8005,|i,cpu| {}) );
		map.insert("8XY6_SHR"  , Instruction::new(0xF00F,0x8006,|i,cpu| {}) );
		map.insert("8XY7_SUBN" , Instruction::new(0xF00F,0x8007,|i,cpu| {}) );
		map.insert("8XYE_SHL"  , Instruction::new(0xF00F,0x800E,|i,cpu| {}) );
		map.insert("9XY0_SNE"  , Instruction::new(0xF00F,0x9000,|i,cpu| {}) );
		map.insert("ANNN_LD"   , Instruction::new(0xF000,0xA000,|i,cpu| {}) );
		map.insert("BNNN_JMP"  , Instruction::new(0xF000,0xB000,|i,cpu| {}) );
		map.insert("CXNN_RND"  , Instruction::new(0xF000,0xC000,|i,cpu| {}) );
		map.insert("DXYN_DRW"  , Instruction::new(0xF000,0xD000,|i,cpu| {}) );
		map.insert("EX9E_SKP"  , Instruction::new(0xF0FF,0xE09E,|i,cpu| {}) );
		map.insert("EXA1_SKNP" , Instruction::new(0xF0FF,0xE0A1,|i,cpu| {}) );
		map.insert("FX07_LD"   , Instruction::new(0xF0FF,0xF007,|i,cpu| {}) );
		map.insert("FX0A_LD"   , Instruction::new(0xF0FF,0xF00A,|i,cpu| {}) );
		map.insert("FX15_LD"   , Instruction::new(0xF0FF,0xF015,|i,cpu| {}) );
		map.insert("FX18_LD"   , Instruction::new(0xF0FF,0xF018,|i,cpu| {}) );
		map.insert("FX1E_ADD"  , Instruction::new(0xF0FF,0xF01E,|i,cpu| {}) );
		map.insert("FX29_LD"   , Instruction::new(0xF0FF,0xF029,|i,cpu| {}) );
		map.insert("FX33_LD"   , Instruction::new(0xF0FF,0xF033,|i,cpu| {}) );
		map.insert("FX55_LD"   , Instruction::new(0xF0FF,0xF055,|i,cpu| {}) );
		map.insert("FX65_LD"   , Instruction::new(0xF0FF,0xF065,|i,cpu| {}) );

		return map; 
	}
}





