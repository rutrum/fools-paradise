use crate::Sprite;
use super::*;

const ANIMATION_SPEED: u32 = 10;  // frames between states

#[derive(Clone, Debug)]
pub struct Bullet {
    pub sprites: Vec<Sprite>,

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
                Sprite::bullet1,
                Sprite::bullet2,
                Sprite::bullet3,
                Sprite::bullet4,
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
    fn pos(&self) -> (f32, f32) { self.pos }

    fn sprite(&self) -> Sprite { 
        self.sprites[self.state as usize % 4]
    }
}

impl Movement for Bullet {
    fn pos_mut(&mut self) -> &mut (f32, f32) { &mut self.pos }
    fn vel(&self) -> (f32, f32) { self.vel }
    fn vel_mut(&mut self) -> &mut (f32, f32) { &mut self.vel }

    fn update(&mut self, _frame: u32) { 
        self.counter += 1;
        if self.counter % ANIMATION_SPEED == 0 {
            self.state += 1;
        }
        self.advance() 
    }
}
