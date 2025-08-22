use crate::app::entities::Interactive;

pub static PIXEL_BYTES_AMT: usize = 4;

#[derive(Debug)]
pub enum RenderError {
    PixelOutOfBounds(usize)
}

#[derive(Debug)]
pub struct RenderContext<'a> {
    pub height: usize,
    pub width: usize,
    pub buffer: &'a mut [u8]
}

impl<'a> RenderContext<'a> {
    pub fn new(buffer: &'a mut [u8], width: usize, height: usize) -> Self {
        Self {
            buffer,
            width,
            height
        }
    }

    pub fn draw_slice(&mut self, pixels: &[u8], offset: usize, color: u32) -> Result<(), RenderError> {
        for (i, pixel) in pixels.iter().enumerate() {
            if *pixel == 1 {
                self.draw_pixel(i + offset, color)?;
            }
        }

        Ok(())
    }

    pub fn fill_slice(&mut self, pixels: &[u16], offset: usize) -> Result<(), RenderError> {
        for (i, pixel) in pixels.iter().enumerate() {
            self.fill_pixel(i + offset, *pixel)?;
        }

        Ok(())
    }

    fn draw_pixel(&mut self, pixel_n: usize, color: u32) -> Result<(), RenderError> {
        let start_pos = pixel_n * PIXEL_BYTES_AMT;
        let end_pos = start_pos + PIXEL_BYTES_AMT;
        if let Some(pixels) = self.buffer.get_mut(start_pos..end_pos) {
            pixels[0] = ((color & 0xFF000000) >> 24) as u8;
            pixels[1] = ((color & 0x00FF0000) >> 16) as u8;
            pixels[2] = ((color & 0x0000FF00) >> 8) as u8;
            pixels[3] = (color & 0x000000FF) as u8;
        } else {
            return Err(RenderError::PixelOutOfBounds(pixel_n));
        }

        Ok(())
    }

    fn fill_pixel(&mut self, pixel_n: usize, pixel: u16) -> Result<(), RenderError> { // Allow 32-bit
        let start_pos = pixel_n * PIXEL_BYTES_AMT;
        let end_pos = start_pos + PIXEL_BYTES_AMT;
        if let Some(pixels) = self.buffer.get_mut(start_pos..end_pos) {
            let r = ((pixel & 0xF000) >> 12) as u8;
            let g = ((pixel & 0x0F00) >> 8) as u8;
            let b = ((pixel & 0x00F0) >> 4) as u8;
            let a = (pixel & 0x000F) as u8;

            pixels[0] = r * 17;
            pixels[1] = g * 17;
            pixels[2] = b * 17;
            pixels[3] = a * 17;
        } else {
            return Err(RenderError::PixelOutOfBounds(pixel_n));
        }

        Ok(())
    }

    pub fn fill_pixels(&mut self, offset: Offset, pixels: &[u8]) -> Result<(), RenderError> {
        let end_pos = offset + pixels.len();
        if let Some(buf_pixels) = self.buffer.get_mut(offset..end_pos) {
            buf_pixels.copy_from_slice(pixels);
        } else {
            return Err(RenderError::PixelOutOfBounds(offset))
        }

        Ok(())
    }

    pub fn scale(pixels: &mut Vec<u8>, size: Size) {

    }

    pub fn draw_letter<const L:usize>(&mut self, letter: &[u8], mut offset: Offset, color: u32)
                                      -> Result<(), RenderError>
    {
        let (pixels, _) = letter.as_chunks::<L>();
        for line in pixels {
            self.draw_slice(line, offset, color)?;
            offset += self.width;
        }

        Ok(())
    }
}

pub enum PositionRel {
    Top(usize),
    Right(usize),
    Bottom(usize),
    Left(usize)
    //Element(Position?/Element.Position?)
}

impl PositionRel {
    pub fn as_offset(&self, win_width: usize) -> Offset {
        match self {
            _ => 0
        }
    }
}

pub enum Position {
    Relative(PositionRel),
    Absolute(usize, usize)
}

impl Position {
    pub fn as_offset(&self, win_width: usize) -> Offset {
        match self {
            Self::Absolute(x, y) => y * win_width + x,
            Self::Relative(pos) => pos.as_offset(win_width)
        }
    }
}

pub trait Render {
    fn render(&self, ctx: &mut RenderContext) -> Result<(), String>;
    fn as_interactive(&self) -> Option<&dyn Interactive> {
        None
    }
}

pub trait Renderable {
    fn get_pixels(&self) -> Result<Vec<u8>, String>;
    fn destroy(&self) {}
    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;
    fn get_render_width(&self) -> u16;
    fn get_render_height(&self) -> u16;
}

pub trait Resizable {
    fn with_size(&mut self, x: usize, y: usize) -> &Self;
}

pub trait Scalable {
    fn with_scale(&mut self, scale: usize) -> &Self;
}

pub type Offset = usize;
pub type Size = (usize, usize);