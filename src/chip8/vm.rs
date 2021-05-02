use super::bus::Bus;
use super::cpu::Cpu;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VM {
    cpu: Cpu,
    bus: Bus,
}

impl VM {
    pub fn new() -> VM {
        VM {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    pub fn run(&mut self) {
        // println!("{}", self.cpu);
        self.cpu.run(&mut self.bus);
    }

    pub fn get_screen_for_js(&self) -> Box<[JsValue]> {
        self.bus.display.get_screen_for_js()
    }

    pub fn cpu(&mut self) -> &Cpu {
        &self.cpu
    }

    pub fn bus(&mut self) -> &Bus {
        &self.bus
    }
}
