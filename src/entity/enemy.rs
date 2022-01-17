use super::*;
use crate::SpriteList;
use crate::sound;

#[derive(Clone, PartialEq, Debug)]
pub enum EnemyState {
    Stationary,
    Dying,
}

#[derive(Clone, Debug)]
pub struct Enemy {
    pub sprites: Vec<Sprite>,
    pub state: EnemyState,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub fire_counter: u32,
    pub death_counter: u32,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            sprites: vec![
                SpriteList::enemy1.get(),
                SpriteList::enemy2.get(),
                SpriteList::enemy3.get(),
                SpriteList::enemy4.get(),
            ],
            state: EnemyState::Stationary,
            pos: (80.0, -5.0),
            vel: (0.0, 0.5),
            fire_counter: 60,
            death_counter: 0,
        }
    }
}

impl Alive for Enemy {
    fn dead(&self) -> bool {
        self.death_counter > 20
    }

    fn dying(&self) -> bool {
        self.death_counter > 0
    }

    fn kill(&mut self) {
        self.state = EnemyState::Dying;
        self.death_counter += 1;
    }
}

impl Shoot for Enemy {
    fn shoot(&mut self) -> Bullet {
        sound::enemy_fire();
        self.fire_counter = 0;
        let mut bullet = Bullet::new((
            self.x_pos(),
            self.bottom() as f32,
        ));
        bullet.vel.1 = 1.0;
        bullet
    }

    fn ready_to_shoot(&self) -> bool {
        self.fire_counter > 120
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
        use EnemyState::*;
        let idx = match self.state {
            Stationary => 0,
            Dying => match self.death_counter {
                x if x < 7 => 1,
                x if x < 15 => 2,
                _ => 3,
            },
        };

        &self.sprites[idx]
    }

    fn update(&mut self, _: u32) { 
        if self.dying() {
            self.death_counter += 1;
        } else {
            self.fire_counter += 1;
        }
        self.advance();
    }
}
