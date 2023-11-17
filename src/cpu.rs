use crate::bus::Bus;
use std::fmt;

pub const PROGRAM_START: u16 = 0x200;
//#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
	prev_pc: u16,
	ret_stack: Vec<u16>
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
			prev_pc: 0,
			ret_stack: Vec::<u16>::new()
        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
		let hi = bus.ram_read_byte(self.pc) as u16;
		let lo = bus.ram_read_byte(self.pc+1) as u16;
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
				//go to nnn;
				self.pc = nnn;
			},
			0x2 => {
				// call subroutine at adress nnn;
				self.ret_stack.push(self.pc + 2);
				self.pc = nnn;
			},
			0x3 => {
				// skip next instruction if vx = nn;
				if(self.read_reg_vx(x) == nn.try_into().unwrap()) {
					self.pc += 4;
				} else {
					self.pc += 2;
				}

			},
			0x6 =>  {
				//vx = nn;
				self.write_reg_vx(x, nn.try_into().unwrap());
				self.pc += 2;
			},
			0x7 => {
				//vx = vx + nn;
				let vx = self.read_reg_vx(x);
				self.write_reg_vx(x, vx.wrapping_add(nn.try_into().unwrap()));
				self.pc += 2;
			},
			0x8 => {
				match n {

					0x0 => {
						// Vx = Vy
						let vy = self.read_reg_vx(y);
						self.write_reg_vx(x,y.try_into().unwrap());
						self.pc += 2;
					},
					_ =>panic!("unrecognized 0x8XY* instruction {:#X} : {:#X}", self.pc, instruction)
				};
			},
			0xD => {
				//draw(x,y,n);
				self.debug_draw_sprite(bus, x.try_into().unwrap(), y.try_into().unwrap(),n.try_into().unwrap());
				self.pc += 2;
			},
			0xA => {
				// i = nnn;
				self.i = nnn;
				self.pc += 2;
			},
			0xE => {
				match nn {
					0xA1 => {
						// if (key()! = Vx) skip the next instruction
						let key = self.read_reg_vx(x);
						if bus.key_pressed(key) {
							self.pc += 2;
						} else
						{
							self.pc += 4;
						}
					}
					_ =>panic!("unrecognized 0xEX** instruction {:#X} : {:#X}", self.pc, instruction)
				}
			},
			0xF => {
				// i +=vx;
				self.i += self.read_reg_vx(x) as u16;
				self.pc += 2;
			}

			_ =>panic!("unrecognized instruction {:#X} : {:#X}", self.pc, instruction)
		}

    }

	fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
		println!("Draw sprite at ({}, {})", x, y);
		for y in 0..height {
			let b = bus.ram_read_byte(self.i + y as u16);
			if bus.debug_draw_byte(b, x, y) {
				self.write_reg_vx(0xF, 1);
			}
			y += 1
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