use crate::ram::Ram;
use crate::cpu::Cpu;

pub struct Chip8 {
	ram: Ram,
	cpu: Cpu
}

impl Chip8 {
	pub fn new() -> Chip8 {
		Chip8 {
			ram: Ram::new(),

		}
	}
	pub fn load_rom(&mut self, data: &Vec<u8>) {
		let offset = 0x200;
		for i in 0..data.len() {
			self.ram.write_byte((offset + i) as u16, data[i]);
		}
	}

	pub fn run_instruction(&mut self){
		cpu.run_instruction(&mut self.ram);
	}
}