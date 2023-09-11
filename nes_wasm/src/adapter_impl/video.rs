use nes_core::adapter::video::VideoAdapter;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

pub struct VideoCtx {
    canvas: CanvasRenderingContext2d,
}

impl VideoCtx {
    pub fn new(canvas: CanvasRenderingContext2d) -> Self {
        Self { canvas }
    }
}

impl VideoAdapter for VideoCtx {
    fn draw_frame(&mut self, pixels: [[u8; 3]; 256 * 240]) {
        let mut data: [u8; 256 * 240 * 4] = [0; 256 * 240 * 4];
        for y in 0..240 {
            for x in 0..256 {
                let i = y * 256 + x;
                data[i * 4] = pixels[i][0];
                data[i * 4 + 1] = pixels[i][1];
                data[i * 4 + 2] = pixels[i][2];
                data[i * 4 + 3] = u8::MAX;
            }
        }
        self.canvas
            .put_image_data(
                &ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), 256, 240).unwrap(),
                0.0,
                0.0,
            )
            .unwrap();
    }
}
