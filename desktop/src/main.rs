use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("rustyboy")
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance
        .enumerate_adapters(wgpu::Backends::all())
        .find(|adapter| adapter.is_surface_supported(&surface))
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);

    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &surface_config);

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let output = surface.get_current_texture().unwrap();

            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }

        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    surface_config.width = size.width;
                    surface_config.height = size.height;
                    surface.configure(&device, &surface_config);
                }
                _ => {}
            };
        }

        Event::MainEventsCleared => {
            window.request_redraw();
        }

        _ => {}
    });
}
