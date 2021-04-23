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
use std::time::Instant;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();

    let mut graphics = Graphics::new(&sdl_context);
    let mut vm = VM::new();
    let mut sdl_input = SdlInput::new(&sdl_context);
    let game = fs::read("./games/PONG.c8").expect("Couldn't read the file");

    vm.bus.memory.load_game(game);

    let chip8_duration = Duration::new(0, 1_000_000_000 / 60);
    let mut last_process_time: Instant = Instant::now();

    'main: loop {
        match sdl_input.read_input() {
            Some(Keycode::Escape) | Some(Keycode::T) => break 'main,
            key => {
                vm.bus.input.translate_input(key);
            }
        }

        if Instant::now().duration_since(last_process_time) > chip8_duration {
            vm.run();
            graphics.draw_screen(vm.get_screen());
            last_process_time = Instant::now();
        }
    }

    Ok(())
}
