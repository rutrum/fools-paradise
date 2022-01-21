use super::*;
use crate::Sprite;
use crate::sound;
use crate::Random;

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Moving,
    Stationary,
    Firing,
    Dying,
}

/// Moves into position and then continues fire
#[derive(Clone, Debug)]
pub struct Turret {
    pub sprites: Vec<Sprite>,
    pub state: State,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub fire_counter: i32,
    pub death_counter: u32,
    health: u32,
    target_height: f32,
}

impl Turret {
    pub fn spawn(random: &mut Random) -> Self {
        let rand_x = random.in_range(20, 140) as f32;
        Self {
            sprites: vec![
                Sprite::turret1,
                Sprite::turret2,
                Sprite::turret3,
                Sprite::turret4,
                Sprite::turret5,
                Sprite::turret6,
                Sprite::turret7,
                Sprite::turret8,
                Sprite::turret9,
            ],
            state: State::Moving,
            pos: (rand_x, -5.0),
            vel: (0.0, 0.25),
            fire_counter: 0,
            death_counter: 0,
            health: 3,
            target_height: random.in_range(20, 100) as f32,
        }
    }
}

impl Alive for Turret {
    fn dead(&self) -> bool {
        self.death_counter >= 40 
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
        if self.health == 1 && self.fire_counter > 60 {
            // only left gun
            sound::enemy_fire();
            self.state = State::Firing;
            self.fire_counter = -10;
            let mut bullet = Bullet::new((
                self.pos.0 - 4.0,
                self.bottom() as f32 + 1.0,
            ));
            bullet.vel.1 = 1.5;
            vec![ bullet ]
        } else if self.fire_counter > 120 {
            sound::enemy_fire();
            self.state = State::Firing;
            self.fire_counter = -10;
            let mut bullet = Bullet::new((
                self.pos.0 + 4.0,
                self.bottom() as f32 + 1.0,
            ));
            bullet.vel.1 = 1.5;
            let mut bullet2 = Bullet::new((
                self.pos.0 - 4.0,
                self.bottom() as f32 + 1.0,
            ));
            bullet2.vel.1 = 1.5;
            vec![bullet, bullet2]
        } else {
            vec![]
        }
    }
}

impl Render for Turret {
    fn pos(&self) -> (f32, f32) { self.pos }

    fn sprite(&self) -> Sprite { 
        use State::*;
        let idx = match self.state {
            Moving => (self.pos.1.abs() / 2.0) as usize % 3,
            Stationary => if self.health == 1 {
                4
            } else {
                0
            }
            Firing => if self.health == 1 {
                5
            } else {
                3
            }
            Dying => if self.death_counter > 30 {
                8
            } else {
                self.death_counter as usize / 10 + 5
            }
        };

        self.sprites[idx]
    }
}

impl Movement for Turret {
    fn pos_mut(&mut self) -> &mut (f32, f32) { &mut self.pos }
    fn vel(&self) -> (f32, f32) { self.vel }
    fn vel_mut(&mut self) -> &mut (f32, f32) { &mut self.vel }

    fn update(&mut self, _: u32) { 
        if self.pos.1 > self.target_height || self.health == 1 {
            self.vel.1 = 0.0;
            if self.fire_counter > 0 {
                self.state = State::Stationary;
            }
        }
        if self.dying() {
            self.death_counter += 1;
            self.state = State::Dying;
        } else {
            self.fire_counter += 1;
        }
        self.advance();
    }
}
