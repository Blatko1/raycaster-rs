use crate::{player::Player, Graphics};

pub struct Engine {
    player: Player,
}

impl Engine {
    pub fn new() -> Self {
        let player = Player::new();
        Self { player }
    }

    pub fn update(&mut self) {}

    pub fn render(&self, g: &Graphics) {
        let mut render = Render::new(g);

        // Drawing:
        render.render();

        // Finish drawing
        let (frame, buf) = render.finish();
        g.queue.submit(Some(buf));
        frame.present();
    }
}

pub struct Render {
    frame: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
    encoder: wgpu::CommandEncoder,
}

impl Render {
    fn new(g: &Graphics) -> Self {
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

    fn draw<D: Draw>(&mut self, draw: D) {
        self.draw_vec(vec![draw]);
    }

    fn draw_vec<D: Draw>(&mut self, draws: Vec<D>) {
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

        for draw in draws {
            draw.draw(&rpass)
        }
    }

    fn finish(self) -> (wgpu::SurfaceTexture, wgpu::CommandBuffer) {
        (self.frame, self.encoder.finish())
    }
}

pub trait Draw {
    fn draw(&self, rpass: &wgpu::RenderPass);
}
