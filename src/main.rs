mod engine;
mod player;
mod render;

use engine::Engine;
use pollster::block_on;
use std::time::{Duration, Instant, SystemTime};
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ray Caster")
        .build(&event_loop)
        .unwrap();

    let graphics = block_on(Graphics::init(&window)).unwrap();
    let engine = Engine::new();

    run(event_loop, window, engine, graphics);
}

fn run(
    event_loop: EventLoop<()>,
    window: Window,
    mut engine: Engine,
    mut graphics: Graphics,
) {
    let mut then = SystemTime::now();
    let mut now = SystemTime::now();
    let mut fps = 0;
    let target_framerate = Duration::from_secs_f64(1.0 / 60.0);
    let mut delta_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(new_size)
                | winit::event::WindowEvent::ScaleFactorChanged {
                    new_inner_size: &mut new_size,
                    ..
                } => {
                    graphics.config.width = new_size.width;
                    graphics.config.height = new_size.height;
                    graphics
                        .surface
                        .configure(&graphics.device, &graphics.config);
                }

                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }

                winit::event::WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                _ => (),
            },

            winit::event::Event::RedrawRequested(_) => {
                engine.update();

                engine.render(&graphics);

                // FPS counting
                fps += 1;
                if now.duration_since(then).unwrap().as_millis() > 1000 {
                    window.set_title(&format!("Ray Caster FPS: {}", fps));
                    fps = 0;
                    then = now;
                }
                now = SystemTime::now();
            }

            winit::event::Event::MainEventsCleared => {
                // Screen redrawing
                if target_framerate <= delta_time.elapsed() {
                    window.request_redraw();
                    delta_time = Instant::now();
                } else {
                    *control_flow = ControlFlow::WaitUntil(
                        Instant::now() + target_framerate - delta_time.elapsed(),
                    );
                }
            }
            _ => (),
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
    pub async fn init(
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
