use crate::display::Display;
use crate::memory::Memory;

pub struct Bus {
    pub display: Display,
    pub memory: Memory,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            display: Display::new(),
            memory: Memory::new(),
        }
    }
}
