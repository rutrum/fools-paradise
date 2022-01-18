use super::*;
use crate::SpriteName;
use crate::util::Random;

pub struct Cloud {
    pub sprites: Vec<SpriteName>,

    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl Cloud {
    pub fn new(random: &mut Random) -> Self {
        let x = random.in_range(0, 160);
        let pos = (x as f32, 0.0);
        let vel = (0.0, 2.0);

        Self {
            sprites: vec![
                SpriteName::cloud,
            ],
            pos,
            vel,
        }
    }
}

impl Entity for Cloud {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    /// not needed?
    fn sprite_name(&self) -> SpriteName {
        SpriteName::cloud
    }

    fn update(&mut self, _frame: u32) {
        self.advance();
    }
}
