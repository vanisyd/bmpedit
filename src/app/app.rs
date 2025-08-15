use std::collections::HashMap;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use super::renderer::Render;
use super::renderer::RenderContext;

static SCALE: f32 = 30.0;

pub type EntityId = u16;

pub struct WindowOptions {
    title: String,
    height: u16,
    width: u16
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: String::from("Application"),
            height: 32,
            width: 64
        }
    }
}

pub struct App {
    window: Option<Window>,
    window_options: WindowOptions,
    entities: HashMap<EntityId, Box<dyn Render>>,
    next_entity_id: EntityId
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            window_options: WindowOptions::default(),
            entities: HashMap::new(),
            next_entity_id: 0
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);

        event_loop.run_app(self).unwrap();
    }

    pub fn add_entity(&mut self, entity: impl Render + 'static) -> EntityId {
        let entity_id = self.get_next_entity_id();
        self.entities.insert(entity_id, Box::new(entity));
        self.next_entity_id += 1;

        entity_id
    }

    fn get_next_entity_id(&self) -> EntityId {
        self.next_entity_id
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let phys_height = (self.window_options.height as f32 * SCALE) as u16;
        let phys_width = (self.window_options.width as f32 * SCALE) as u16;
        let phys_size = PhysicalSize::new(phys_width, phys_height);

        let attrs = Window::default_attributes()
            .with_title(self.window_options.title.clone())
            .with_resizable(false)
            .with_inner_size(phys_size);
        self.window = Some(event_loop.create_window(attrs).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let window_size = window.inner_size();
                let surface = SurfaceTexture::new(window_size.width, window_size.height, window);
                let win_width: u32 = self.window_options.width as u32;
                let win_height: u32 = self.window_options.height as u32;
                let mut pixels = Pixels::new(win_width, win_height, surface).unwrap();
                let frame = pixels.frame_mut();
                let mut render_ctx = RenderContext::new(frame, win_width as usize, win_height as usize);

                for (_, entity) in &self.entities {
                    entity.render(&mut render_ctx).expect("Error rendering");
                    println!("{:?}", render_ctx);
/*                    if let Some(e) = entity.as_interactive() {
                        let clickable = e.as_clickable().unwrap();
                    }*/
                }

                pixels.render().unwrap();
            },
            _ => (),
        }
    }
}
