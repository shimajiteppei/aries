pub trait VideoAdapter {
    fn draw_frame(&mut self, pixels: [[u8; 3]; 256 * 240]);
}
