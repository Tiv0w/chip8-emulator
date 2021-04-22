use crate::display::Display;
use crate::input::Input;
use crate::memory::Memory;

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
