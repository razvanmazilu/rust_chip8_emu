pub use crate::minifb::Key;

pub struct Keyboard {
	key_pressed: Option<u8>
}

impl Keyboard {

	pub fn new() -> Keyboard {
		Keyboard {
			key_pressed: None,
		}
	}
	//todo implement proper key handling
	pub fn key_pressed(&self, key_code: u8) -> bool {
		true
	}

	pub fn set_keys_pressed(&mut self, key: Option<u8>) {
		self.key_pressed = key;
	}
	pub fn get_key_pressed(&self) -> Option<u8> {
		self.key_pressed
	}
}