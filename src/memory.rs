pub struct Memory {
    mem: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: [0; 4096] }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.mem[address]
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.mem[address] = value;
    }
}
