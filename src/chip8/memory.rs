pub struct Memory {
    mem: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        let mut default_mem = [0; 4096];
        for (index, byte) in FONT_SET.iter().enumerate() {
            default_mem[index] = *byte;
        }
        Memory { mem: default_mem }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.mem[address]
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.mem[address] = value;
    }

    pub fn load_game(&mut self, game: Vec<u8>) {
        for (index, byte) in game.iter().enumerate() {
            self.mem[index + 0x200] = *byte;
        }
    }
}

static FONT_SET: [u8; 0x50] = [
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
