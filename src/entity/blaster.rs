use super::*;
use crate::Sprite;
use crate::sound;
use crate::Random;

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Stationary,
    Dying,
}

#[derive(Clone, Debug)]
pub struct Blaster {
    sprites: Vec<Sprite>,
    state: State,
    pos: (f32, f32),
    vel: (f32, f32),
    fire_counter: u32,
    death_counter: u32,
    health: u32,
    fire_cap: u32,
    bullet_speed: f32,
}

impl Blaster {
    pub fn spawn(random: &mut Random, cycle: Cycle, player: &Player) -> Self {
        let x = if let Cycle::Day = cycle {
            random.in_range(6, 160 - 6) as f32
        } else {
            random.in_range(
                6.max(player.pos.0 as i32 - 40) as u32,
                (160 - 6).min(player.pos.0 as i32 + 40) as u32,
            ) as f32
        };
        let mut blaster = Self {
            sprites: vec![
                Sprite::enemy1,
                Sprite::enemy2,
                Sprite::enemy3,
                Sprite::enemy4,
            ],
            state: State::Stationary,
            pos: (x, -5.0),
            vel: (0.0, 0.0),
            fire_counter: 90,
            death_counter: 0,
            health: 1,
            fire_cap: 0,
            bullet_speed: 0.0,
        };
        blaster.mutate(cycle);
        blaster
    }
}

impl Alive for Blaster {
    fn dead(&self) -> bool {
        self.death_counter > 20
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

impl CycleDependent for Blaster {
    fn mutate(&mut self, cycle: Cycle) {
        if let Cycle::Day = cycle {
            self.vel.1 = 0.5;
            self.fire_cap = 120;
            self.bullet_speed = 1.2;
        } else {
            self.vel.1 = 1.0;
            self.fire_cap = 90;
            self.bullet_speed = 2.0;
        }
    }
}

impl Shoot for Blaster {
    fn shoot(&mut self) -> Vec<Bullet> {
        if self.fire_counter >= self.fire_cap {
            sound::enemy_fire();
            self.fire_counter = 0;
            let mut bullet = Bullet::new((
                self.pos.0,
                self.bottom() as f32,
            ));
            bullet.vel.1 = self.bullet_speed;
            vec![ bullet ]
        } else {
            vec![]
        }
    }
}

impl Render for Blaster {
    fn pos(&self) -> (f32, f32) { self.pos }

    fn sprite(&self) -> Sprite { 
        use State::*;
        let idx = match self.state {
            Stationary => 0,
            Dying => match self.death_counter {
                x if x < 7 => 1,
                x if x < 15 => 2,
                _ => 3,
            },
        };

        self.sprites[idx]
    }
}

impl Movement for Blaster {
    fn pos_mut(&mut self) -> &mut (f32, f32) { &mut self.pos }
    fn vel(&self) -> (f32, f32) { self.vel }
    fn vel_mut(&mut self) -> &mut (f32, f32) { &mut self.vel }

    fn update(&mut self, _: u32) { 
        if self.dying() {
            self.death_counter += 1;
        } else {
            self.fire_counter += 1;
        }
        self.advance();
    }
}
