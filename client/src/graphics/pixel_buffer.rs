pub struct PixelBuffer {
    pixels: Vec<u8>,
}

impl PixelBuffer {
    pub const WIDTH: usize = 320;
    pub const HEIGHT: usize = 180;
    pub const CHANNELS: usize = 4;

    pub fn new() -> Self {
        Self { pixels: vec![0; Self::WIDTH * Self::HEIGHT * Self::CHANNELS] }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: u32) {
        let index = (x + y * Self::WIDTH) * Self::CHANNELS;

        self.pixels[index] = (pixel >> 24) as u8;
        self.pixels[index + 1] = (pixel >> 16) as u8;
        self.pixels[index + 2] = (pixel >> 8) as u8;
        self.pixels[index + 3] = pixel as u8;
    }

    pub fn copy_to_slice(&self, slice: &mut [u8]) {
        slice.copy_from_slice(&self.pixels)
    }
}