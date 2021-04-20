use crate::bus::Bus;
use crate::utils;

pub struct Cpu {
    delay: u8,
    i: u16,
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
            pc: 0,
            sound: 0,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
        }
    }

    pub fn execute_opcode(&mut self, bus: &mut Bus, opcode: u16) {
        let hex_digits: [u8; 4] = utils::get_hex_digits(opcode);
        match hex_digits {
            [0x0, 0x0, 0xE, 0x0] => {
                println!("Clearscreen op");
                bus.display.clear();
            }
            [0x0, 0x0, 0xE, 0xE] => {
                println!("Return from chip8 subroutine");
            }
            [0x0, a, b, c] => {
                println!("Perfect {} {} {}", a, b, c);
            }
            [a, b, c, d] => {
                println!("Not implemented for now {} {} {} {}", a, b, c, d);
            }
        }
    }

    pub fn set_vf(&mut self, value: u8) {
        self.v[15] = value;
    }
}

// part of debugging
impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cpu {{\n\tdelay: {},\n\ti: {},\n\tsound: {},\n\tpc: {},\n\tsp: {},\n\tstack: {:?},\n\tv: {:?}\n}}",
            self.delay,
            self.i,
            self.pc,
            self.sound,
            self.sp,
            self.stack,
            self.v
        )
    }
}
