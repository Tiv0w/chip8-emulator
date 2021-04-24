use super::display::Display;
use super::input::Input;
use super::memory::Memory;

pub struct Bus {
    pub display: Display,
    pub memory: Memory,
    pub input: Input,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            display: Display::new(),
            memory: Memory::new(),
            input: Input::new(),
        }
    }
}
