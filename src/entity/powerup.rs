use super::*;

pub enum PowerType {
    Health,
}

pub struct PowerUp {
    pub t: PowerType,
    pub sprites: Vec<SpriteName>,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub collected: bool,
}

impl PowerUp {
    pub fn new(t: PowerType) -> Self {
        Self {
            sprites: vec![
                SpriteName::heart,
            ],
            pos: (0.0, -5.0),
            vel: (0.0, 1.5),
            t,
            collected: false,
        }
    }
}

impl Entity for PowerUp {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    fn sprite_name(&self) -> SpriteName { 
        use PowerType::*;
        let idx = match self.t {
            Health => 0,
        };

        self.sprites[idx]
    }

    fn update(&mut self, _frame: u32) {
        self.advance();
    }
}
