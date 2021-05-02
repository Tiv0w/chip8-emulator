use crate::{HEIGHT, WIDTH};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Display {
    screen: [[bool; HEIGHT]; WIDTH],
}

#[wasm_bindgen]
impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[false; HEIGHT]; WIDTH],
        }
    }

    pub fn get_screen_for_js(&self) -> Box<[JsValue]> {
        let mut screen_js: [JsValue; HEIGHT * WIDTH];
        for (col_idx, col) in self.screen.iter().enumerate() {
            for (val_idx, value) in col.iter().enumerate() {
                let idx = HEIGHT * col_idx + val_idx;
                screen_js[idx] = JsValue::from(*value);
            }
        }
        let boxed_screen: Box<[JsValue]> = Box::new(screen_js);
        boxed_screen
    }

    pub fn clear(&mut self) {
        self.screen = [[false; HEIGHT]; WIDTH];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, new_state: bool) {
        self.screen[x][y] ^= new_state;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
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

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision_happened = false;

        // We iterate on each row of the sprite
        // since the sprite is an slice of byte, each row is only a byte
        for (row_idx, &row) in sprite.iter().enumerate() {
            let bool_row = Self::destructure_byte_to_bool(row);

            // We iterate on each bit of the byte
            for (col_idx, &state) in bool_row.iter().enumerate() {
                let pixel_pos_x = (x + col_idx) % WIDTH;
                let pixel_pos_y = (y + row_idx) % HEIGHT;
                let current_pixel = self.get_pixel(pixel_pos_x, pixel_pos_y);

                collision_happened = collision_happened || (current_pixel && state);

                self.set_pixel(pixel_pos_x, pixel_pos_y, state);
            }
        }

        collision_happened
    }
}
