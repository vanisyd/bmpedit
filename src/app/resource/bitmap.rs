use std::io::{BufReader, Error, Read};
use crate::app::renderer::{RenderContext, Renderable, Resizable, Scalable, PIXEL_BYTES_AMT};
use crate::app::resource::Resource;

static SPRITE_CODE: &[u8] = b"SPRT";

pub struct Bitmap {
    width: u16,
    height: u16,
    pixels: Vec<u16>,
    bitmask: Option<Vec<u8>>,
    render_width: u16,
    render_height: u16
}

struct BitmapHeader {
    magic: [u8; 4],
    width: u8,
    height: u8,
    channels: u8,
    px_type: u8,
    has_bitmask: u8
}

impl Bitmap {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            pixels: Vec::with_capacity((width * height) as usize),
            bitmask: None,
            render_width: width,
            render_height: height
        }
    }

    pub fn with_bitmask(&mut self, pixels: Vec<u8>) -> &Self {
        self.bitmask = Some(pixels);
        self
    }
}

impl Renderable for Bitmap {
    fn get_pixels(&self) -> Result<Vec<u8>, String> {
        let mut buf: Vec<u8> = vec![0; (self.get_render_width() * self.get_render_height()) as usize * PIXEL_BYTES_AMT];
        let mut ctx = RenderContext::new(
            &mut buf,
            self.get_render_width() as usize,
            self.get_render_height() as usize
        );
        let mut offset: usize = 0;
        for slice in self.pixels.chunks_exact(self.get_render_width() as usize) {
            ctx.fill_slice(slice, offset)
                .map_err(|err| format!("Error getting pixels: {:?}", err))?;
            offset += self.get_render_width() as usize;
        }

        Ok(buf)
    }

    fn get_width(&self) -> u16 {
        self.width
    }

    fn get_height(&self) -> u16 {
        self.height
    }

    fn get_render_width(&self) -> u16 {
        self.render_width
    }

    fn get_render_height(&self) -> u16 {
        self.render_height
    }
}

impl Resizable for Bitmap {
    fn with_size(&mut self, x: usize, y: usize) -> &Self {
        let mut buf = self.pixels.clone();
        let x_scale = (x - (self.get_render_width() as usize)) + 1;
        let y_scale = (y - (self.get_render_height() as usize)) + 1;
        let new_row_size = x_scale * self.get_render_width() as usize;
        let new_col_size = y_scale * self.get_render_height() as usize;
        let mut buf = Vec::with_capacity(new_row_size * new_col_size);
        for line in self.pixels.chunks_exact(self.get_render_width() as usize) {
            let mut ln_buf: Vec<u16> = Vec::with_capacity(new_row_size);
            for px in line {
                ln_buf = [ln_buf, [*px].repeat(x_scale)].concat();
            }
            buf = [buf, ln_buf.repeat(y_scale)].concat();
        }
        self.pixels = buf;
        self.render_width = new_row_size as u16;
        self.render_height = new_col_size as u16;

        self
    }
}

impl Scalable for Bitmap {
    fn with_scale(&mut self, scale: usize) -> &Self {
        if scale == 1 {
            return self
        }

        let new_row_size = scale * self.get_render_width() as usize;
        let new_col_size = scale * self.get_render_height() as usize;
        let mut buf = Vec::with_capacity(new_row_size * new_col_size);
        for line in self.pixels.chunks_exact(self.get_render_width() as usize) {
            let mut ln_buf: Vec<u16> = Vec::with_capacity(new_row_size);
            for px in line {
                ln_buf = [ln_buf, [*px].repeat(scale)].concat();
            }
            buf = [buf, ln_buf.repeat(scale)].concat();
        }
        self.pixels = buf;
        self.render_width = new_row_size as u16;
        self.render_height = new_col_size as u16;

        self
    }
}

impl Resource for Bitmap {
    fn load_res<R: Sized + Read>(reader: &mut BufReader<R>) -> Bitmap {
        let mut magic: [u8; 4] = [0; 4];
        let mut width: u8 = 0;
        let mut height: u8 = 0;
        let mut channels: u8 = 0;
        let mut px_type: u8 = 0;
        let mut has_bitmask: u8 = 0;
        let mut reserved: u8 = 0;

        reader.read_exact(&mut magic).unwrap();
        reader.read_exact(std::slice::from_mut(&mut width)).unwrap();
        reader.read_exact(std::slice::from_mut(&mut height)).unwrap();
        reader.read_exact(std::slice::from_mut(&mut channels)).unwrap();
        reader.read_exact(std::slice::from_mut(&mut px_type)).unwrap();
        reader.read_exact(std::slice::from_mut(&mut has_bitmask)).unwrap();
        reader.read_exact(std::slice::from_mut(&mut reserved)).unwrap();

        let bitmask: Option<Vec<u8>> = if has_bitmask == 1 {
            let mut buf: Vec<u8> = Vec::with_capacity((width * height) as usize);
            reader.read_exact(&mut buf).unwrap();
            Some(buf)
        } else {
            None
        };

        let mut pixels: Vec<u16> = Vec::with_capacity((width * height) as usize);
        let mut px_buf: [u8; 2] = [0; 2];
        while let Ok(()) = reader.read_exact(&mut px_buf) {
            pixels.push(u16::from_le_bytes(px_buf));
        }

        Self {
            width: width.into(),
            height: height.into(),
            pixels,
            bitmask,
            render_width: width.into(),
            render_height: height.into()
        }
    }
}