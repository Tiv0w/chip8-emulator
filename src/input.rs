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
}
