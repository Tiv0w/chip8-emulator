mod bus;
mod cpu;
mod display;
mod graphics;
mod input;
mod memory;
mod utils;
mod vm;
use crate::graphics::Graphics;
use crate::input::SdlInput;
use crate::vm::VM;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();

    let mut graphics = Graphics::new(&sdl_context);
    let mut vm = VM::new();
    let mut sdl_input = SdlInput::new(&sdl_context);

    'main: loop {
        match sdl_input.read_input() {
            // TEMP: input not translated, still for testing
            Some(Keycode::Escape) | Some(Keycode::Q) => break 'main,
            key => {
                vm.bus.input.translate_input(key);
            } // Some(Keycode::D) => {
              //     vm.cpu.execute_opcode(&mut vm.bus, 0xD323);
              // }
              // Some(Keycode::E) => {
              //     vm.cpu.execute_opcode(&mut vm.bus, 0x00E0);
              //     vm.cpu.execute_opcode(&mut vm.bus, 0x00EE);
              // }
              // _ => {}
        }

        vm.run();
        graphics.draw_screen(vm.get_screen());
        // Chip8 runs at 60Hz
        // TEMP: for testing, runs at 3Hz
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 3));
    }

    Ok(())
}
