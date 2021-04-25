mod chip8;
mod desktop;
use crate::chip8::vm::VM;
use crate::desktop::graphics::Graphics;
use crate::desktop::input::SdlInput;
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

    let process_duration = Duration::from_millis(1000 / 60);
    let key_update_duration = Duration::from_millis(200);
    let mut last_process_time: Instant = Instant::now();
    let mut last_key_time: Instant = Instant::now();
    let mut last_key_pressed: Option<Keycode> = None;

    'main: loop {
        match sdl_input.read_input() {
            Some(Keycode::Escape) => break 'main,
            Some(key) => last_key_pressed = Some(key),
            None => {}
        }

        if Instant::now().duration_since(last_key_time) > key_update_duration {
            let current_input = sdl_input.translate_input(last_key_pressed);
            vm.bus.input.set_current_key(current_input);
            last_key_pressed = None;
            last_key_time = Instant::now();
        }

        if Instant::now().duration_since(last_process_time) > process_duration {
            vm.run();
            graphics.draw_screen(vm.get_screen());
            last_process_time = Instant::now();
        }
    }

    Ok(())
}
