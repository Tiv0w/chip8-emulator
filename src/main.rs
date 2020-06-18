mod cpu;
mod display;
use crate::cpu::Cpu;
use crate::display::Display;

fn main() {
    let cpu = Cpu::new();
    let display = Display::new();
    println!("{}", cpu);
}
