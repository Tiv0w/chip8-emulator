use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Input {
    event_pump: EventPump,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Input {
        Input {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn read_input(&mut self) -> Option<Keycode> {
        let event: Option<Event> = self.event_pump.poll_event();
        match event {
            Some(Event::Quit { .. }) => Some(Keycode::Escape),
            Some(Event::KeyDown { keycode: code, .. }) => code,
            _ => None,
        }
    }
}
