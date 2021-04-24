use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct SdlInput {
    event_pump: EventPump,
}

impl SdlInput {
    pub fn new(sdl_context: &sdl2::Sdl) -> SdlInput {
        SdlInput {
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

pub struct Input {
    pub current_input: Option<u8>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            current_input: None,
        }
    }

    pub fn translate_input(&mut self, key: Option<Keycode>) {
        let translation: Option<u8> = match key {
            Some(keycode) => match keycode {
                Keycode::Exclaim | Keycode::Ampersand => Some(0x1),
                Keycode::At | Keycode::Asterisk => Some(0x2),
                Keycode::Hash | Keycode::LeftParen => Some(0x3),
                Keycode::Dollar | Keycode::RightParen => Some(0xC),

                Keycode::Q | Keycode::U => Some(0x4),
                Keycode::W | Keycode::I => Some(0x5),
                Keycode::E | Keycode::O => Some(0x6),
                Keycode::R | Keycode::P => Some(0xD),

                Keycode::A | Keycode::J => Some(0x7),
                Keycode::S | Keycode::K => Some(0x8),
                Keycode::D | Keycode::L => Some(0x9),
                Keycode::F | Keycode::Colon => Some(0xE),

                Keycode::Z | Keycode::M => Some(0xA),
                Keycode::X | Keycode::Less => Some(0x0),
                Keycode::C | Keycode::Greater => Some(0xB),
                Keycode::V | Keycode::Question => Some(0xF),
                _ => None,
            },
            None => None,
        };
        self.current_input = translation;
    }
}
