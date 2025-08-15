use crate::app::entities::{Interactive, Render, Renderable};
use crate::app::renderer::{Position, RenderContext, Size, PIXEL_BYTES_AMT};

pub struct Button<R: Renderable> {
    inner: R,
    position: Position,
    text: String,
    size: Size
}

impl<R: Renderable> Button<R> {
    pub fn new(inner: R, position: Position) -> Self {
        Self {
            inner,
            position,
            text: String::from("Test"),
            size: (5, 5)
        }
    }
}

impl<R: Renderable> Render for Button<R> {
    fn render(&self, ctx: &mut RenderContext) -> Result<(), String> {
        let pixels = self.inner.get_pixels()?;
        let mut offset = self.position.as_offset(ctx.width) * PIXEL_BYTES_AMT;
        let slice_width = self.inner.get_width() as usize * PIXEL_BYTES_AMT;
        for slice in pixels.chunks_exact(slice_width) {
            ctx.fill_pixels(offset, slice)
                .map_err(|err| format!("Failed to render button: {:?}", err))?;
            offset += ctx.width * PIXEL_BYTES_AMT;
        }

        Ok(())
    }
}

impl <R: Renderable> Interactive for Button<R> {
    fn on_click(&mut self) {
        println!("I'm clicked!");
    }

    fn as_clickable(&self) -> Option<&dyn Interactive> {
        Some(self)
    }
}