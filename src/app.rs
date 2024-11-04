use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, window::Window,
};

use crate::graphics::{self, create_renderer_resources, render_frame, resize_renderer};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    last_render_time: Option<Instant>,
    graphics: Option<graphics::Graphics<'static>>,
    last_size: (u32, u32),
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title("Spree");
        let Ok(window) = event_loop.create_window(attributes) else {
            return;
        };
        if self.window.is_some() {
            return;
        }

        let window_handle = Arc::new(window);
        self.window = Some(window_handle.clone());

        let (width, height) = (
            window_handle.inner_size().width,
            window_handle.inner_size().height,
        );
        let graphics = pollster::block_on(async move {
            create_renderer_resources(window_handle.clone(), width, height).await
        });
        self.graphics = Some(graphics);

        self.last_render_time = Some(Instant::now());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let (Some(window), Some(last_render_time), Some(graphics)) = (
            self.window.as_ref(),
            self.last_render_time.as_mut(),
            self.graphics.as_mut(),
        ) else {
            return;
        };

        match event {
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(key_code),
                        ..
                    },
                ..
            } => {
                // Exit by pressing the escape key
                if matches!(key_code, winit::keyboard::KeyCode::Escape) {
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                let (width, height) = ((width).max(1), (height).max(1));
                resize_renderer(graphics, width, height);
                self.last_size = (width, height);
            }
            WindowEvent::CloseRequested => {
                log::info!("Close requested. Exiting...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let _delta_time = now - *last_render_time;
                *last_render_time = now;
                render_frame(graphics);
            }
            _ => (),
        }

        window.request_redraw();
    }
}
