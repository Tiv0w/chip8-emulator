use super::display::Display;
use super::input::Input;
use super::memory::Memory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Bus {
    pub display: Display,
    pub memory: Memory,
    pub input: Input,
}

#[wasm_bindgen]
impl Bus {
    pub fn new() -> Bus {
        Bus {
            display: Display::new(),
            memory: Memory::new(),
            input: Input::new(),
        }
    }
}
