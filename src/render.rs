use crate::Graphics;

pub struct Pipeline {
    default: wgpu::RenderPipeline
}

impl Pipeline {
    pub fn new(g: &Graphics) -> Self {
        let default_layout = g.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Default Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let default = g.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Default Render Pipeline"),
            layout: Some(&default_layout),
            vertex: wgpu::VertexState {
                module: todo!(),
                entry_point: todo!(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: Some(wgpu::IndexFormat::Uint16),
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: todo!(),
                entry_point: todo!(),
                targets: todo!(),
            }),
            multiview: None,
        });

        Self {
            default
        }
    }
}

pub struct Render {
    frame: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
    encoder: wgpu::CommandEncoder,
}

impl Render {
    pub fn new(g: &Graphics) -> Self {
        let frame = match g.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                g.surface.configure(&g.device, &g.config);
                g.surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = g
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        Self {
            frame,
            view,
            encoder,
        }
    }

    pub fn draw<D: Draw>(&mut self, draws: Vec<&D>) {
        let rpass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        for d in draws {
            d.draw(&rpass)
        }
    }

    pub fn finish(self) -> (wgpu::SurfaceTexture, wgpu::CommandBuffer) {
        (self.frame, self.encoder.finish())
    }
}

pub trait Draw {
    fn draw(&self, rpass: &wgpu::RenderPass);
}
