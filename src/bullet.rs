use crate::entity::Entity;
use crate::sprite::Sprite;
use crate::SpriteList;

#[derive(Clone, Debug)]
pub struct Bullet {
    pub sprites: Vec<Sprite>,

    pub pos: (f32, f32),
    pub vel: (f32, f32),

    pub state: u32,
}

impl Bullet {
    pub fn new(pos: (f32, f32)) -> Self {
        Self {
            sprites: vec![
                SpriteList::bullet1.get(),
                SpriteList::bullet2.get(),
                SpriteList::bullet3.get(),
                SpriteList::bullet4.get(),
            ],
            pos,
            vel: (0.0, 0.0),
            state: 0,
        }
    }
}

impl Entity for Bullet {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    fn sprite(&self) -> &Sprite { 
        &self.sprites[self.state as usize % 4]
    }

    fn update(&mut self, frame: u32) { 
        self.state = frame / 10;
        self.advance() 
    }
}
