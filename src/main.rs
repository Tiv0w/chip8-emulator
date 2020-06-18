mod cpu;
mod display;
mod graphics;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::graphics::Graphics;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use std::time::Duration;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<(), String> {
    let cpu = Cpu::new();
    let mut display = Display::new();
    println!("{}", cpu);

    let sdl_context = sdl2::init().unwrap();

    let mut graphics = Graphics::new(&sdl_context);
    graphics.draw_display(&display.screen);

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let array = [0x20, 0x60, 0x20, 0x20, 0x70];
                    display.draw(3, 2, &array);
                    graphics.draw_display(&display.screen);
                }
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    Ok(())
}
