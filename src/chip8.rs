pub use crate::bus::Bus;
pub use crate::cpu;
pub use crate::cpu::Cpu;


pub struct Chip8 {
	cpu: Cpu,
	bus: Bus
}

impl Chip8 {
	pub fn new() -> Chip8 {
		Chip8 {
			cpu: Cpu::new(),
			bus: Bus::new()
		}
	}
	pub fn load_rom(&mut self, data: &Vec<u8>) {
		let offset = 0x200;
		for i in 0..data.len() {
			self.bus.ram_write_byte(cpu::PROGRAM_START + (i as u16), data[i]);
		}
	}

	pub fn run_instruction(&mut self){
		self.cpu.run_instruction(&mut self.bus);
		println!("Cpu state: {:#?}", self.cpu);
	}
}

