#[derive(Debug, Default)]
pub struct Register {
    pub pulse1: [u8; 4],
    pub pulse2: [u8; 4],
    pub triangle: [u8; 4],
    pub noise: [u8; 4],
    pub dmc: [u8; 4],
    pub status: u8,
    pub frame_counter: u8,
}
