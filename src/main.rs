extern crate minifb;

use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use display::Display;
use minifb::{KeyRepeat, Window, WindowOptions, Key};

pub mod ram;
pub mod cpu;
pub mod chip8;
pub mod bus;
pub mod display;
pub mod keyboard;

pub fn get_chip8_key_code_for(key: Option<Key>) -> Option<u8> {
    match key {
        Some(Key::Key1) => Some(0x1),
        Some(Key::Key2) => Some(0x2),
        Some(Key::Key3) => Some(0x3),
        Some(Key::Key4) => Some(0xC),

        Some(Key::Q) => Some(0x4),
        Some(Key::W) => Some(0x5),
        Some(Key::E) => Some(0x6),
        Some(Key::R) => Some(0xD),

        Some(Key::A) => Some(0x7),
        Some(Key::S) => Some(0x8),
        Some(Key::D) => Some(0x9),
        Some(Key::F) => Some(0xE),

        Some(Key::Z) => Some(0xA),
        Some(Key::X) => Some(0x0),
        Some(Key::C) => Some(0xB),
        Some(Key::V) => Some(0xF),

        _=> None
    }

}

fn main() {
    let mut file = File::open("games/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);
    
   
    
    const WIDTH: usize = 640;
    const HEIGHT: usize = 320;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    for i in buffer.iter_mut() {
        *i = 0xffff0000;
    }
    let mut window = Window::new(
        "Rust - chip8 emulator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let key_pressed = window.get_keys_pressed(KeyRepeat:: No);
        let key = match key_pressed {
            Some(keys) => Some(keys[0]),
            None => None
        };
        let chip8_key = get_chip8_key_code_for(key);

        chip8.run_instruction();
        let chip8_buffer = chip8.get_display_buffer();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = Display::get_index_from_coord(x/10, y/10);
                let pixel = chip8_buffer[index];
                let color_pixel = match pixel {
                    0 => 0x0,
                    1 => 0xffffffff,
                    _ => unreachable!()
                };
                buffer[y*WIDTH + x] = color_pixel;
            }
        }
        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
