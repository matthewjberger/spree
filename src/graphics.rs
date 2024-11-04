pub struct Graphics<'window> {
    pub surface: wgpu::Surface<'window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_format: wgpu::TextureFormat,
    pub depth_texture_view: wgpu::TextureView,
}

/// Creates resources needed for rendering
pub async fn create_renderer_resources<'window>(
    window: impl Into<wgpu::SurfaceTarget<'window>>,
    width: u32,
    height: u32,
) -> Graphics<'window> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all),
        ..Default::default()
    });

    let surface = instance.create_surface(window).unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to request adapter!");
    let (device, queue) = {
        log::info!("WGPU Adapter Features: {:#?}", adapter.features());
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WGPU Device"),
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits {
                        max_texture_dimension_2d: 4096, // Allow higher resolutions on native
                        ..wgpu::Limits::downlevel_defaults()
                    },
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .expect("Failed to request a device!")
    };

    let surface_capabilities = surface.get_capabilities(&adapter);

    // This assumes an sRGB surface texture
    let surface_format = surface_capabilities
        .formats
        .iter()
        .copied()
        .find(|f| !f.is_srgb()) // egui wants a non-srgb surface texture
        .unwrap_or(surface_capabilities.formats[0]);

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width,
        height,
        present_mode: surface_capabilities.present_modes[0],
        alpha_mode: surface_capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &surface_config);

    let depth_texture_view = create_depth_texture(&device, width, height);

    Graphics {
        surface,
        device,
        queue,
        surface_config,
        surface_format,
        depth_texture_view,
    }
}

/// Resize the renderer, call when window resizes
pub fn resize_renderer(graphics: &mut Graphics, width: u32, height: u32) {
    log::info!("Resizing renderer surface to: ({width}, {height})");
    graphics.surface_config.width = width;
    graphics.surface_config.height = height;
    graphics
        .surface
        .configure(&graphics.device, &graphics.surface_config);
    graphics.depth_texture_view = create_depth_texture(&graphics.device, width, height);
}

/// Renders a frame, call once per frame after updates
pub fn render_frame(graphics: &mut Graphics) {
    let mut encoder = graphics
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    let surface_texture = graphics
        .surface
        .get_current_texture()
        .expect("Failed to get surface texture!");

    let surface_texture_view = surface_texture
        .texture
        .create_view(&wgpu::TextureViewDescriptor {
            label: wgpu::Label::default(),
            aspect: wgpu::TextureAspect::default(),
            format: Some(graphics.surface_format),
            dimension: None,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

    encoder.insert_debug_marker("Render scene");

    // This scope around the crate::render_pass prevents the
    // crate::render_pass from holding a borrow to the encoder,
    // which would prevent calling `.finish()` in
    // preparation for queue submission.
    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.19,
                        g: 0.24,
                        b: 0.42,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &graphics.depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }

    graphics.queue.submit(std::iter::once(encoder.finish()));
    surface_texture.present();
}

/// Creates a depth texture and view
fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    let texture = device.create_texture(
        &(wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        }),
    );
    texture.create_view(&wgpu::TextureViewDescriptor {
        label: None,
        format: Some(wgpu::TextureFormat::Depth32Float),
        dimension: Some(wgpu::TextureViewDimension::D2),
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        base_array_layer: 0,
        array_layer_count: None,
        mip_level_count: None,
    })
}
