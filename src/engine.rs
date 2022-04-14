use crate::{player::Player, Graphics, render::Render};

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
        render.draw(vec![&self.player]);

        // Finish drawing
        let (frame, buf) = render.finish();
        g.queue.submit(Some(buf));
        frame.present();
    }
}