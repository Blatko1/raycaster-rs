use pollster::block_on;
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, Window}};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ray Caster")
        .build(&event_loop)
        .unwrap();

    let graphics = block_on(Graphics::new(&window)).unwrap();

    run(event_loop, window, graphics);
}

fn run(
    event_loop: EventLoop<()>,
    window: Window,
    Graphics {
        device,
        surface,
        adapter,
        queue,
        config,
    }: Graphics,
) {
    event_loop.run(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            winit::event::Event::WindowEvent { window_id, event } => todo!(),
            winit::event::Event::DeviceEvent { device_id, event } => todo!(),
            winit::event::Event::MainEventsCleared => todo!(),
            winit::event::Event::RedrawRequested(_) => todo!(),
            winit::event::Event::RedrawEventsCleared => todo!(),
            winit::event::Event::LoopDestroyed => todo!(),
            _ => ()
        }
    })
}

pub struct Graphics {
    pub device: wgpu::Device,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl Graphics {
    pub async fn new(
        window: &winit::window::Window,
    ) -> Result<Self, wgpu::RequestDeviceError> {
        let backends =
            wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let instance = wgpu::Instance::new(backends);

        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window);
            (size, surface)
        };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("No adapters found!");

        let adapter_info = adapter.get_info();
        println!(
            "Adapter info: Name: {}, backend: {:?}, device: {:?}",
            adapter_info.name, adapter_info.backend, adapter_info.device_type
        );

        let required_features = wgpu::Features::empty();
        let adapter_features = adapter.features();
        let required_limits = wgpu::Limits::default();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: adapter_features & required_features,
                    limits: required_limits,
                },
                None,
            )
            .await?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Ok(Self {
            device,
            surface,
            adapter,
            queue,
            config,
        })
    }
}
