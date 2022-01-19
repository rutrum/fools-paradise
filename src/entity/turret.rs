use super::*;
use crate::SpriteName;
use crate::sound;
use crate::Random;

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Moving,
    Stationary,
    Dying,
}

/// Moves into position and then continues fire
#[derive(Clone, Debug)]
pub struct Turret {
    pub sprites: Vec<SpriteName>,
    pub state: State,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub fire_counter: u32,
    pub death_counter: u32,
    health: u32,
    target_height: f32,
}

impl Turret {
    pub fn new(random: &mut Random) -> Self {
        let rand_x = random.in_range(20, 140) as f32;
        Self {
            sprites: vec![
                SpriteName::turret,
            ],
            state: State::Stationary,
            pos: (rand_x, -5.0),
            vel: (0.0, 0.5),
            fire_counter: 60,
            death_counter: 0,
            health: 2,
            target_height: random.in_range(20, 100) as f32,
        }
    }
}

impl Alive for Turret {
    fn dead(&self) -> bool {
        self.death_counter > 0
    }

    fn dying(&self) -> bool {
        self.death_counter > 0
    }

    fn health(&self) -> u32 {
        self.health
    }

    fn health_mut(&mut self) -> &mut u32 {
        &mut self.health
    }

    fn kill(&mut self) {
        sound::enemy_death();
        self.state = State::Dying;
        self.death_counter += 1;
    }
}

impl Shoot for Turret {
    fn shoot(&mut self) -> Vec<Bullet> {
        sound::enemy_fire();
        self.fire_counter = 0;
        let mut bullet = Bullet::new((
            self.x_pos(),
            self.bottom() as f32,
        ));
        bullet.vel.1 = 2.0;
        vec![bullet]
    }

    fn ready_to_shoot(&self) -> bool {
        self.fire_counter > 60
    }
}

impl Entity for Turret {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    fn sprite_name(&self) -> SpriteName { 
        use State::*;
        let idx = match self.state {
            _ => 0,
        };

        self.sprites[idx]
    }

    fn update(&mut self, _: u32) { 
        if self.y_pos() > self.target_height {
            self.vel.1 = 0.0;
        }
        if self.dying() {
            self.death_counter += 1;
        } else {
            self.fire_counter += 1;
        }
        self.advance();
    }
}
