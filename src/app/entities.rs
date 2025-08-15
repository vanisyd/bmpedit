pub mod button;

use crate::app::resource::bitmap::Bitmap;
use crate::app::resource::{Resource, PATH_RESOURCES};
use super::app::{App};
use super::entities::button::Button;
use super::renderer::{Position, Render, RenderContext, Renderable};

pub struct Container<R: Render> {
    inner: Vec<R>,
    width: u16,
    height: u16,
    position: Position
}

impl<R: Render> Container<R> {
    pub fn new(inner: R, width: u16, height: u16, position: Position) -> Self {
        Self {
            inner: vec![],
            width,
            height,
            position
        }
    }
}

impl <R: Render> Render for Container<R> {
    fn render(&self, ctx: &mut RenderContext) -> Result<(), String> {
        let mut buf: Vec<u8> = Vec::with_capacity((self.width * self.height) as usize);
        let mut container_ctx = RenderContext::new(&mut buf, self.width as usize, self.height as usize);
        for e in &self.inner {
            e.render(&mut container_ctx)?;
        }

        let offset = self.position.as_offset(self.width as usize);
        for pixels in buf.chunks_exact(self.width as usize) {
            ctx.draw_slice(pixels, offset, 0)
                .map_err(|err| format!("Error rendering container: {:?}", err))?;
        }

        Ok(())
    }
}

pub struct SimpleObject<R: Render> {
    inner: R
}

impl<R: Render> SimpleObject<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner
        }
    }
}

impl <R: Render> Render for SimpleObject<R> {
    fn render(&self, ctx: &mut RenderContext) -> Result<(), String> {
        self.inner.render(ctx)?;
        Ok(())
    }

    fn as_interactive(&self) -> Option<&dyn Interactive> {
        self.inner.as_interactive()
    }
}

pub trait Interactive {
    fn on_click(&mut self) {}
    fn as_clickable(&self) -> Option<&dyn Interactive> {
        None
    }
}

pub fn tst() {
    let bmp = Bitmap::from_file(format!("{}/sprite_x.sprt", PATH_RESOURCES))
        .unwrap();
    let btn = Button::new(bmp, Position::Absolute(0, 1));
    let element = SimpleObject::new(btn);
    let mut app = App::new();
    app.add_entity(element);
    app.run();
}
