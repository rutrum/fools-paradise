use super::*;
use crate::Random;

pub enum EnemyType {
    Blaster,
    Turret {
        target_height: f32,
    },
    Kite,
}

impl EnemyType {
    fn sprite_list(&self) -> Vec<Sprite> {
        use EnemyType::*;
        match self {
            Blaster => vec![
                Sprite::enemy1,
                Sprite::enemy2,
                Sprite::enemy3,
                Sprite::enemy4,
            ],
            Turret{ .. } => vec![
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
            Kite => vec![
                Sprite::kite1,
                Sprite::kite2,
            ],
        }
    }

    fn starting_health(&self) -> u32 {
        use EnemyType::*;
        match &self {
            Turret{ .. } => 3,
            _ => 1,
        }
    }

    fn velocity(&self, cycle: Cycle) -> (f32, f32) {
        use EnemyType::*;
        use Cycle::*;
        match (&self, cycle) {
            (Blaster, Day) => (0.0, 0.5),
            (Blaster, Night) => (0.0, 1.0),
            _ => (0.0, 0.0),
        }
    }

    fn shoot_freq(&self, cycle: Cycle) -> u32 {
        use EnemyType::*;
        use Cycle::*;
        match (&self, cycle) {
            (Blaster, Day) => 120,
            (Blaster, Night) => 90,
            _ => 120,
        }
    }

    fn bullet_speed(&self, cycle: Cycle) -> f32 {
        use EnemyType::*;
        use Cycle::*;
        match (&self, cycle) {
            (Blaster, Day) => 1.2,
            (Blaster, Night) => 2.0,
            _ => 1.0,
        }
    }
}

pub struct Enemy {
    sprites: Vec<Sprite>,
    t: EnemyType,
    pos: (f32, f32),
    vel: (f32, f32),
    health: u32,
    shoot_freq: u32,

    time_alive: u32,
    dying_counter: u32,
}

impl Enemy {
    pub fn spawn(t: EnemyType, random: &mut Random, cycle: Cycle) -> Self {
        let enemy = Self {
            sprites: t.sprite_list(),
            pos: (0.0, 0.0),
            vel: t.velocity(cycle),
            health: t.starting_health(),

            shoot_freq: t.shoot_freq(cycle),

            time_alive: 0,
            dying_counter: 0,
            t,
        };
        enemy
    }

    /// Updates the enemy based on a change in cycle.
    pub fn mutate(&mut self, cycle: Cycle) {
        self.shoot_freq = self.t.shoot_freq(cycle);
        self.vel = self.t.velocity(cycle);
    }
}
