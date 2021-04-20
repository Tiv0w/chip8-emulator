mod bus;
mod cpu;
mod display;
mod graphics;
mod input;
mod memory;
mod utils;
mod vm;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::vm::VM;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();

    let mut graphics = Graphics::new(&sdl_context);
    let mut vm = VM::new();
    let mut input = Input::new(&sdl_context);

    'main: loop {
        match input.read_input() {
            Some(Keycode::Escape) | Some(Keycode::Q) => break 'main,
            Some(Keycode::D) => {
                let array = [0x20, 0x60, 0x20, 0x20, 0x70];
                let collision = vm.bus.display.draw((3, 2), &array);
                vm.cpu.set_vf(collision as u8);
                println!("{:?}", vm.cpu);
            }
            Some(Keycode::E) => {
                vm.cpu.execute_opcode(&mut vm.bus, 0x00E0);
                vm.cpu.execute_opcode(&mut vm.bus, 0x00EE);
            }
            _ => {}
        }

        graphics.draw_screen(vm.get_screen());
        // Chip8 runs at 60Hz
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    Ok(())
}
