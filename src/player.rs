use crate::render::Draw;

pub struct Player {
    x: f32,
    y: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 300.0, y: 300.0 }
    }
}

impl Draw for Player {
    fn draw(&self, rpass: &wgpu::RenderPass) {
        
    }
}
