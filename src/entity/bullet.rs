use crate::SpriteName;
use super::*;

const ANIMATION_SPEED: u32 = 10;  // frames between states

#[derive(Clone, Debug)]
pub struct Bullet {
    pub sprites: Vec<SpriteName>,

    pub pos: (f32, f32),
    pub vel: (f32, f32),

    pub state: u32,
    pub counter: u32,

    pub dead: bool,
    pub damage: u32,
}

impl Bullet {
    pub fn new(pos: (f32, f32)) -> Self {
        Self {
            sprites: vec![
                SpriteName::bullet1,
                SpriteName::bullet2,
                SpriteName::bullet3,
                SpriteName::bullet4,
            ],
            pos,
            vel: (0.0, 0.0),
            state: 0,
            counter: 0,
            dead: false,
            damage: 1,
        }
    }
}

impl Render for Bullet {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }

    fn sprite(&self) -> SpriteName { 
        self.sprites[self.state as usize % 4]
    }
}

impl Movement for Bullet {
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    fn update(&mut self, _frame: u32) { 
        self.counter += 1;
        if self.counter % ANIMATION_SPEED == 0 {
            self.state += 1;
        }
        self.advance() 
    }
}
