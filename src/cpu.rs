use crate::ram::Ram;
use std::fmt;

pub const PROGRAM_START: u16 = 0x200;
//#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
	prev_pc: u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
			prev_pc: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {
		let hi = ram.read_byte(self.pc) as u16;
		let lo = ram.read_byte(self.pc+1) as u16;
		let instruction : u16 = (hi << 8) | lo;
		println!("instruction read{:#X} hi:{:#X} lo:{:#X}", instruction, hi, lo);
	//	if lo == 0x00 && hi == 0x00 {
	//		panic!();
		
	//	}
		let nnn = instruction & 0x0FFF;
		let nn  = instruction & 0x00FF;
		let n   = instruction & 0x000F;
		let x   = (instruction & 0x0F00) >> 8;
		let y 	= (instruction & 0x00F0) >> 4;
		println!("nnn:{:#X}, nn{:#X}, n{:#X}, x{:#X}, y{:#X}", nnn, nn, n, x, y);

		if (self.prev_pc == self.pc)
		{
			panic!("Please increment PC");
		}
		self.prev_pc = self.pc;

		match (instruction & 0xF000) >> 12{
			0x1 =>{
				//go to nnn
				self.pc = nnn;
			}
			0x6 =>{
				//vx = nn;
				self.write_reg_vx(x, nn.try_into().unwrap());
				self.pc += 2;

			}

			_=>panic!("unrecognized instruction {:#X} : {:#X}", self.pc, instruction)
		}

    }

	pub fn write_reg_vx(&mut self, index: u16, value: u8) {
		self.vx[index as usize] = value;
	}

	pub fn read_reg_vx(&mut self, index: u16) -> u8 {
		self.vx[index as usize]
	}
}



impl fmt::Debug for Cpu {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		write!(f, "pc :{:#X}\n", self.pc);
		write!(f, "vx: ");
		for item in self.vx.iter() {
			write!(f, "{:#X} ", *item);
		}
		write!(f, "\n");
		write!(f, "i: {:#X}\n", self.i)
	}
}