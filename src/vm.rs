use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::{HEIGHT, WIDTH};

pub struct VM {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl VM {
    pub fn new() -> VM {
        VM {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    pub fn run(&mut self) {
        println!("{:?}", self.cpu);
        self.cpu.run(&mut self.bus);
    }

    pub fn get_screen(&self) -> [[bool; HEIGHT]; WIDTH] {
        self.bus.display.screen
    }

    fn execute_opcode(&mut self, opcode: u16) {
        self.cpu.execute_opcode(&mut self.bus, opcode);
    }
}
