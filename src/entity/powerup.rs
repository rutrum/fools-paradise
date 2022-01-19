use super::*;

#[derive(Clone, PartialEq, Debug)]
pub enum PowerType {
    Health,
    Spreader,
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
    pub fn new(t: PowerType) -> Self {
        Self {
            sprites: vec![
                Sprite::heart,
            ],
            pos: (0.0, -5.0),
            vel: (-0.5, 1.5),
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
            Spreader => 0,
        };

        self.sprites[idx]
    }
}

impl Movement for PowerUp {
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    fn update(&mut self, _frame: u32) {
        self.movement_counter += 1;
        if self.movement_counter >= 30 {
            self.movement_counter = 0;
            self.vel.0 = -self.vel.0;
        }
        self.advance();
    }
}
