use super::*;
use crate::Sprite;
use crate::sound;
use crate::Random;

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
    health: u32,
}

impl Enemy {
    pub fn spawn(random: &mut Random) -> Self {
        let x = random.in_range(8, 160 - 8) as f32;
        Self {
            sprites: vec![
                Sprite::enemy1,
                Sprite::enemy2,
                Sprite::enemy3,
                Sprite::enemy4,
            ],
            state: EnemyState::Stationary,
            pos: (x, -5.0),
            vel: (0.0, 1.0),
            fire_counter: 60,
            death_counter: 0,
            health: 1,
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

    fn health(&self) -> u32 {
        self.health
    }

    fn health_mut(&mut self) -> &mut u32 {
        &mut self.health
    }

    fn kill(&mut self) {
        sound::enemy_death();
        self.state = EnemyState::Dying;
        self.death_counter += 1;
    }
}

impl Shoot for Enemy {
    fn shoot(&mut self) -> Vec<Bullet> {
        if self.fire_counter > 60 {

        sound::enemy_fire();
        self.fire_counter = 0;
        let mut bullet = Bullet::new((
            self.x_pos(),
            self.bottom() as f32,
        ));
        bullet.vel.1 = 2.0;
        vec![bullet]
        } else {
            vec![]
        }
    }
}

impl Render for Enemy {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }

    fn sprite(&self) -> Sprite { 
        use EnemyState::*;
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

impl Movement for Enemy {
    fn x_pos_mut(&mut self) -> &mut f32 { &mut self.pos.0 }
    fn y_pos_mut(&mut self) -> &mut f32 { &mut self.pos.1 }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }

    fn update(&mut self, _: u32) { 
        if self.dying() {
            self.death_counter += 1;
        } else {
            self.fire_counter += 1;
        }
        self.advance();
    }
}
