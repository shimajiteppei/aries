use nes_core::adapter::video::VideoAdapter;
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, Sdl};

pub struct VideoCtx {
    canvas: WindowCanvas,
    pixel_size: u32,
}

impl VideoCtx {
    pub fn new(sdl: &Sdl, pixel_size: u32) -> Self {
        let video_subsystem = sdl.video().expect("Could not initalize SDL video context.");
        let window = video_subsystem
            .window("aries", 256 * pixel_size, 240 * pixel_size)
            .position_centered()
            .build()
            .expect("Could not initialize video subsystem of SDL window.");

        let canvas = window
            .into_canvas()
            .build()
            .expect("Could not make a canvas.");
        Self { canvas, pixel_size }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(Rect::new(
                x * self.pixel_size as i32,
                y * self.pixel_size as i32,
                self.pixel_size,
                self.pixel_size,
            ))
            .expect("Could not draw a rectangle pixel.");
    }
}

impl VideoAdapter for VideoCtx {
    fn draw_frame(&mut self, pixels: [[u8; 3]; 256 * 240]) {
        self.canvas.clear();

        for y in 0..240 {
            for x in 0..256 {
                let rgb = pixels[256 * y + x];
                self.draw_rect(x as i32, y as i32, Color::RGB(rgb[0], rgb[1], rgb[2]));
            }
        }

        self.canvas.present();
    }
}
