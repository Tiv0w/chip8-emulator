use crate::{HEIGHT, WIDTH};

type Coordinates = (usize, usize);

pub struct Display {
    pub screen: [[bool; HEIGHT]; WIDTH],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[false; HEIGHT]; WIDTH],
        }
    }

    pub fn clear(&mut self) {
        self.screen = [[false; HEIGHT]; WIDTH];
    }

    pub fn set_pixel(&mut self, (x, y): Coordinates, new_state: bool) {
        self.screen[x][y] ^= new_state;
    }

    pub fn get_pixel(&self, (x, y): Coordinates) -> bool {
        self.screen[x][y]
    }

    fn destructure_byte_to_bool(byte: u8) -> [bool; 8] {
        let mut bool_array: [bool; 8] = [false; 8];

        // Convert each bit of the byte to a bool and put it in the bool_array
        for i in 0..8 {
            bool_array[7 - i] = ((byte >> i) & 1) == 1;
        }

        bool_array
    }

    pub fn draw(&mut self, (x, y): Coordinates, sprite: &[u8]) -> bool {
        let mut collision_happened = false;

        // We iterate on each row of the sprite
        // since the sprite is an slice of byte, each row is only a byte
        for (row_idx, &row) in sprite.iter().enumerate() {
            let bool_row = Self::destructure_byte_to_bool(row);

            // We iterate on each bit of the byte
            for (col_idx, &state) in bool_row.iter().enumerate() {
                let pixel_pos = ((x + col_idx), (y + row_idx));
                let current_pixel = self.get_pixel(pixel_pos);

                collision_happened = collision_happened || (current_pixel && state);

                self.set_pixel(pixel_pos, state);
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
