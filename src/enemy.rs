use crate::entity::*;
use crate::sprite::Sprite;
use crate::bullet::Bullet;
use crate::SpriteList;
use crate::wasm4::sys::*;

#[derive(Clone, PartialEq, Debug)]
pub enum EnemyState {
    Stationary,
    Dead,
}

impl EnemyState {
    fn sprite_idx(&self) -> usize {
        use EnemyState::*;
        match self {
            Stationary => 0,
            Dead => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Enemy {
    pub sprites: Vec<Sprite>,
    pub state: EnemyState,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub movement_counter: i32,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            sprites: vec![
                SpriteList::enemy.get(),
            ],
            state: EnemyState::Stationary,
            pos: (80.0, -5.0),
            vel: (0.0, 0.5),
            movement_counter: 0,
        }
    }

    /// Updates the position.
    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}

impl Alive for Enemy {
    fn dead(&self) -> bool {
        EnemyState::Dead == self.state
    }

    fn kill(&mut self) {
        self.state = EnemyState::Dead;
    }
}

impl Shoot for Enemy {
    fn shoot(&self) -> Bullet {
        let mut bullet = Bullet::new((
            self.x_pos(),
            self.top() as f32,
        ));
        bullet.vel.1 = -2.0;
        bullet
    }
}

impl Entity for Enemy {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    fn sprite(&self) -> &Sprite { 
        &self.sprites[self.state.sprite_idx()] 
    }

    fn update(&mut self, _: u32) { 

        self.advance();
    }
}
