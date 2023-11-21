extern crate minifb;

use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use minifb::Key;
use display::Display;
use minifb::{Window, WindowOptions};

pub mod ram;
pub mod cpu;
pub mod chip8;
pub mod bus;
pub mod display;
pub mod keyboard;

fn main() {
    let mut file = File::open("games/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);
    
    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
    
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

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {


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
