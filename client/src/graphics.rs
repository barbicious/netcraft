use sdl3::pixels::PixelFormat;
use sdl3::render::{ScaleMode, Texture, WindowCanvas};
use crate::graphics::pixel_buffer::PixelBuffer;

pub mod pixel_buffer;

pub struct Renderer {
    pixel_buffer: PixelBuffer,
    window_canvas: WindowCanvas,
    screen_texture: Texture
}

impl Renderer {
    pub fn new(window_canvas: WindowCanvas) -> anyhow::Result<Self> {
        let texture_creator = window_canvas.texture_creator();
        let mut screen_texture = texture_creator.create_texture_streaming(Some(PixelFormat::ARGB8888), PixelBuffer::WIDTH as u32, PixelBuffer::HEIGHT as u32)?;
        screen_texture.set_scale_mode(ScaleMode::Nearest);

        let pixel_buffer = PixelBuffer::new();

        Ok(Self {
            pixel_buffer,
            window_canvas,
            screen_texture
        })
    }

    pub fn flush(&mut self) {
        self.window_canvas.clear();

        self.pixel_buffer.clear();
    }

    pub fn blit_pixel(&mut self, x: i32, y: i32, color: u32) {
        self.pixel_buffer.set_pixel(x as usize, y as usize, color);
    }

    pub fn splat(&mut self) -> anyhow::Result<()> {
        self.screen_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            self.pixel_buffer.copy_to_slice(buffer)
        })?;

        self.window_canvas.copy(&self.screen_texture, None, None)?;

        self.window_canvas.present();

        Ok(())
    }
}