pub struct Cpu {
    delay: u8,
    i: u16,
    memory: [u8; 4096],
    pc: u16,
    sound: u8,
    sp: u16,
    stack: [u16; 16],
    v: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            delay: 0,
            i: 0,
            memory: [0; 4096],
            pc: 0,
            sound: 0,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
        }
    }

    pub fn set_vf(&mut self, value: u8) {
        self.v[15] = value;
    }

    fn get_hex_digits(opcode: u16) -> [u8; 4] {
        let mut digits_array: [u8; 4] = [0; 4];

        for i in 0..4 {
            let hex: u16 = 0x10;
            digits_array[i] = ((opcode / hex.pow(i as u32)) % hex) as u8;
        }
        digits_array.reverse();

        digits_array
    }

    pub fn execute_opcode(&mut self, opcode: u16) {
        let hex_digits: [u8; 4] = Self::get_hex_digits(opcode);
        match hex_digits {
            [0x0, 0x0, 0xE, 0x0] => println!("Clearscreen op"),
            [0x0, 0x0, 0xE, 0xE] => println!("Return from chip8 subroutine"),
            [0x0, a, b, c] => println!("Perfect {} {} {}", a, b, c),
            [a, b, c, d] => println!("Not implemented for now {} {} {} {}", a, b, c, d),
        }
    }
}

// part of debugging
impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cpu {{\n\tdelay: {},\n\ti: {},\n\tmemory: {:?}...,\n\tsound: {},\n\tpc: {},\n\tsp: {},\n\tstack: {:?},\n\tv: {:?}\n}}",
            self.delay,
            self.i,
            &self.memory[..32],
            self.pc,
            self.sound,
            self.sp,
            self.stack,
            self.v
        )
    }
}
