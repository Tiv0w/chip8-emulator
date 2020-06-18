use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;

pub struct Graphics {
    canvas: Canvas<sdl2::video::Window>,
}

impl Graphics {
    pub fn new(sdl_context: &sdl2::Sdl) -> Graphics {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("CHIP-8 YAYA", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas: Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();
        // Set the canvas' scale to be more human acceptable
        canvas.set_scale(10.0, 10.0).unwrap();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        Graphics { canvas }
    }

    pub fn draw_display(&mut self, screen: &[bool]) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        for y in 0..32 {
            let offset = 64 * y;
            for x in 0..64 {
                if screen[offset + x] {
                    self.canvas
                        .draw_point(Point::new(x as i32, y as i32))
                        .unwrap();
                }
            }
        }

        self.canvas.present();
    }
}