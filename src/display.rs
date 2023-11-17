struct Display{

}

impl Display{
	pub fn new() -> Display {
		Display {}
	}

	pub fn debug_draw_byte(mut byte: u8, x: u8, y: u8) {
			for _ in 0..8 {
				match (byte & 0b1000_0000)  >> 7{
					0 => print!(" "),
					1 => print!("#"),
					_ => unreachable!(),
				}
				byte = byte << 1;
			}
			print!("\n");
	}
}