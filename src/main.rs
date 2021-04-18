mod cpu;
mod display;
mod graphics;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::graphics::Graphics;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<(), String> {
    let mut cpu = Cpu::new();
    println!("{:?}", cpu);
    let mut display = Display::new();
    display.clear();

    let sdl_context = sdl2::init().unwrap();

    let mut graphics = Graphics::new(&sdl_context);
    graphics.draw_display(display.screen);

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
                    let collision = display.draw((3, 2), &array);
                    cpu.set_vf(collision as u8);
                    graphics.draw_display(display.screen);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    cpu.execute_opcode(0x00E0);
                    cpu.execute_opcode(0x00EE);
                }
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    Ok(())
}
