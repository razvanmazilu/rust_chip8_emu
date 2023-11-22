use crate::bus::Bus;
use std::fmt;

pub const PROGRAM_START: u16 = 0x200;
//#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16,
    ret_stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
            ret_stack: Vec::<u16>::new(),
        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let hi = bus.ram_read_byte(self.pc) as u16;
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;
        println!("instruction read pc:{:#X} {:#X} hi:{:#X} lo:{:#X}", self.pc, instruction, hi, lo);
        //	if lo == 0x00 && hi == 0x00 {
        //		panic!();

        //	}
        let nnn = instruction & 0x0FFF;
        let nn = instruction & 0x00FF;
        let n = instruction & 0x000F;
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        println!("nnn:{}, nn:{}, n:{}, x:{}, y:{}", nnn, nn, n, x, y);

        if (self.prev_pc == self.pc) {
            panic!("Please increment PC");
        }
        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x00 => {
                match nn {
                    0xE0 => {
                        bus.clear_screen();
                        self.pc += 2;
                    }
                    0xEE => {
                        //return from subroutine
                        let addr = self.ret_stack.pop().unwrap();
                        self.pc = addr;
                    }

                    _ => panic!(
                        "unrecognized 0x00** instruction {:#X} : {:#X}",
                        self.pc, instruction
                    ),
                }
            }
            0x1 => {
                //go to nnn;
                self.pc = nnn;
            }
            0x2 => {
                // call subroutine at adress nnn;
                self.ret_stack.push(self.pc + 2);
                self.pc = nnn;
            }
            0x3 => {
                // skip next instruction if vx = nn;
                if (self.read_reg_vx(x) == nn.try_into().unwrap()) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6 => {
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
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                match n {
                    0x0 => {
                        // Vx = Vy
                        self.write_reg_vx(x, y.try_into().unwrap());
                    },

                    0x1 => {
                        //vx = vx|vy
                        self.write_reg_vx(x, vx | vy);
                    },

                    0x2 => {
                        //vx = vx&v
                        self.write_reg_vx(x, vx & vy);
                    },

                    0x3 => {
                        //vx = vx^vy
                        self.write_reg_vx(x, vx ^ vy);
                    },

                    0x4 => {
                        //vx = vx + vy;
                        let sum: u16 = vx as u16 + vy as u16;
                        self.write_reg_vx(x, sum as u8);
                        if (sum > 0xFF) {
                            self.write_reg_vx(0xF, 1);
                        }
                    },

                    0x5 => {
                        //vx = vx - vy;
                        let diff: i8 = vx as i8 - vy as i8;
                        self.write_reg_vx(x, diff as u8);
                        if (diff < 0) {
                            self.write_reg_vx(0xF, 1);
                        }
					},

					0x6 => {
						//vx = vy = vy >> 1
						let bit = vy & 1;
						self.write_reg_vx(0xF, bit);
						self.write_reg_vx(y, vy >> 1);
						self.write_reg_vx(x, vy >> 1);
					},
                    
                    _ => panic!(
                        "unrecognized 0x8XY* instruction {:#X} : {:#X}",
                        self.pc, instruction
                    ),
                };
                self.pc += 2;
            }
			0x9 => {
				//jump next instruction if vx != vy
				let vx = self.read_reg_vx(x);
				let vy = self.read_reg_vx(y);
				if(vx != vy) {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},
            0xD => {
                //draw(x,y,n);
				let x = self.read_reg_vx(x);
				let y = self.read_reg_vx(y);
                self.debug_draw_sprite(bus, x, y, n.try_into().unwrap());
                self.pc += 2;
            },
            0xA => {
                // i = nnn;
                self.i = nnn;
                self.pc += 2;
            },
            0xE => {
                match nn {
					0x9E => {
						//if (key() == vx) skip the next instruction
						let key = self.read_reg_vx(x);
						if bus.key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
					},
                    0xA1 => {
                        // if (key()! = Vx) skip the next instruction
                        let key = self.read_reg_vx(x);
                        if !bus.key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    _ => panic!(
                        "unrecognized 0xEX** instruction {:#X} : {:#X}",
                        self.pc, instruction
                    ),
                }
            }
            0xF => {
				match nn {
                    0x07 => {
                        //set vx with value from delay_timer
                        self.write_reg_vx(x, bus.get_delay_timer());
                        self.pc += 2;
                    },
					0x0A => {
						//vx = get_key_blocking()
						let key = bus.get_key_pressed();
						match key {
							Some(val) => {
								self.write_reg_vx(x, val);
								self.pc += 2;
							}
							None => ()
						}
						
					},
                    0x15 => {
                        //set delay_timer with value at vx
                        bus.set_delay_timer(self.read_reg_vx(x));
                        self.pc += 2;
                    },
                    0x65 => {
                        //fill v0..vx with data from ram starting at address i
                        for index in 0..x+1 {
                            let value = bus.ram_read_byte(self.i + index as u16);
                            self.write_reg_vx(index, value);
                            self.pc += 2;
                        }
                    },
					0x1E => {
                        // i += vx;
						self.i += self.read_reg_vx(x) as u16;
                        self.pc += 2;
					},
					_ => panic!("unrecognized 0xF instruction {:#X} : {:#X}", self.pc, instruction),
				}
                
                
            },

            _ => panic!("unrecognized instruction {:#X} : {:#X}", self.pc, instruction),
        }
    }

    fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        println!("Draw sprite at ({}, {})", x, y);
        let mut should_set_vf = false;
        for mut ii in 0..height {
            let b = bus.ram_read_byte(self.i + ii as u16);
            if bus.debug_draw_byte(b, x, ii) {
                should_set_vf = true;
            }
        }
        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        } else {
            self.write_reg_vx(0xF, 0);
        }
        bus.present_screen();
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
        write!(f, "\npc :{:#X}\n", self.pc);
        write!(f, "vx: ");
        for item in self.vx.iter() {
            write!(f, "{:#X} ", *item);
        }
        write!(f, "\n");
        write!(f, "i: {:#X}\n", self.i)
    }
}
