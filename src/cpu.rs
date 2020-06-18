pub struct Cpu {
    delay: u8,
    i: u16,
    memory: [u8; 4096],
    sound: u8,
    sp: u16,
    stack: [u16; 16],
    v: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay: 0,
            sound: 0,
        }
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cpu {{\n\tdelay: {},\n\ti: {},\n\tmemory: {:?}...,\n\tsound: {},\n\tsp: {},\n\tstack: {:?},\n\tv: {:?}\n}}",
            self.delay,
            self.i,
            &self.memory[..32],
            self.sound,
            self.sp,
            self.stack,
            self.v
        )
    }
}
