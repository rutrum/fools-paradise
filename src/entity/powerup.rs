use super::*;
use crate::Random;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PowerType {
    Health,
    Spreader,
    Speed,
}

pub struct PowerUp {
    pub t: PowerType,
    pub sprites: Vec<Sprite>,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub collected: bool,
    pub movement_counter: u32,
}

impl PowerUp {
    pub fn spawn(t: PowerType, pos: (f32, f32)) -> Self {
        Self {
            sprites: vec![
                Sprite::heart,
                Sprite::spreader,
                Sprite::speed,
            ],
            pos,
            vel: (-0.3, 0.8),
            t,
            collected: false,
            movement_counter: 0,
        }
    }
}

impl Render for PowerUp {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }

    fn sprite(&self) -> Sprite { 
        use PowerType::*;
        let idx = match self.t {
            Health => 0,
            Spreader => 1,
            Speed => 2,
        };

        self.sprites[idx]
    }
}

impl Movement for PowerUp {
    fn pos_mut(&mut self) -> &mut (f32, f32) { &mut self.pos }
    fn vel(&self) -> (f32, f32) { self.vel }
    fn x_vel_mut(&mut self) -> &mut f32 { &mut self.vel.0 }
    fn y_vel_mut(&mut self) -> &mut f32 { &mut self.vel.1 }

    fn update(&mut self, _frame: u32) {
        self.movement_counter += 1;
        if self.movement_counter >= 30 {
            self.movement_counter = 0;
            self.vel.0 = -self.vel.0;
        }
        self.advance();
    }
}
