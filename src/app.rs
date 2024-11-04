use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, window::Window,
};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    last_render_time: Option<Instant>,
    // renderer: Option<Renderer<'static>>,
    // gui_state: Option<egui_winit::State>,
    // last_render_time: Option<Instant>,
    // #[cfg(target_arch = "wasm32")]
    // renderer_receiver: Option<Receiver<Renderer<'static>>>,
    // last_size: (u32, u32),
    // panels_visible: bool,
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

        // let (width, height) = (
        //     window_handle.inner_size().width,
        //     window_handle.inner_size().height,
        // );
        // let renderer =
        //     pollster::block_on(
        //         async move { Renderer::new(window_handle.clone(), width, height).await },
        //     );
        // self.renderer = Some(renderer);
        //
        self.last_render_time = Some(Instant::now());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let (Some(window), Some(last_render_time)) =
            (self.window.as_ref(), self.last_render_time.as_mut())
        else {
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
                log::info!("Resizing renderer surface to: ({width}, {height})");
                // renderer.resize(width, height);
                // self.last_size = (width, height);
            }
            WindowEvent::CloseRequested => {
                log::info!("Close requested. Exiting...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let _delta_time = now - *last_render_time;
                *last_render_time = now;
                // renderer.render_frame(delta_time);
            }
            _ => (),
        }

        window.request_redraw();
    }
}
