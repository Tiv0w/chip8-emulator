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
use std::fs;
use std::time::Duration;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();

    let mut graphics = Graphics::new(&sdl_context);
    let mut vm = VM::new();
    let mut sdl_input = SdlInput::new(&sdl_context);
    let game = fs::read("./games/PONG.c8").expect("Couldn't read the file");

    vm.bus.memory.load_game(game);

    'main: loop {
        match sdl_input.read_input() {
            Some(Keycode::Escape) | Some(Keycode::T) => break 'main,
            key => {
                vm.bus.input.translate_input(key);
            }
        }

        vm.run();
        graphics.draw_screen(vm.get_screen());
        // Chip8 runs at 60Hz
        // FIXME: find another way to limit to 60Hz
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    Ok(())
}
