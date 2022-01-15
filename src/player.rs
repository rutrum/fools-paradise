use crate::entity::Entity;
use crate::sprite::Sprite;
use crate::SpriteList;

#[derive(Clone, Debug)]
pub struct Player {
    pub sprites: Vec<Sprite>,

    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl Player {
    pub fn new() -> Self {
        Self {
            sprites: vec![SpriteList::ship.get()],
            pos: (80.0, 80.0),
            vel: (0.0, 0.0),
        }
    }

    /// Updates the position.
    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}

impl Entity for Player {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    fn sprite(&self) -> &Sprite { &self.sprites[0] }

    fn update(&mut self, _: u32) { self.advance() }
}
