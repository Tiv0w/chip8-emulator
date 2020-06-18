use crate::{HEIGHT, WIDTH};

pub struct Display {
    pub screen: [bool; 2048],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [false; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.screen = [false; WIDTH * HEIGHT];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, state: bool) {
        self.screen[y * WIDTH + x] = state;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.screen[y * WIDTH + x]
    }

    fn destructure_byte_to_bool(byte: u8) -> [bool; 8] {
        let mut bool_array: [bool; 8] = [false; 8];

        //
        for i in 0..8 {
            bool_array[7 - i] = ((byte >> i) & 1) == 1;
        }
        return bool_array;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision_happened = false;
        for (row_idx, &row) in sprite.iter().enumerate() {
            println!("YESSSS: {:#x}, {:#010b}", row, row);
            let bool_row = Self::destructure_byte_to_bool(row);

            for (col_idx, &state) in bool_row.iter().enumerate() {
                let pixel_pos = (y + row_idx) * WIDTH + x + col_idx;

                println!(
                    "Pixel {} position: {}. Screen value: {}. State value: {}",
                    col_idx, pixel_pos, self.screen[pixel_pos], state
                );
                collision_happened = self.screen[pixel_pos] == true && state == true;
                self.screen[pixel_pos] ^= state;
            }
        }

        collision_happened
    }
}

pub static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
