use super::*;
use crate::Sprite;
use crate::sound;
use crate::PowerType;
use crate::color;

const TURN_FRAMES: i32 = 15;

#[derive(Clone, Debug)]
pub enum PlayerState {
    Stationary,
    TiltLeft,
    TurnLeft,
    TiltRight,
    TurnRight,
    Dying,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub sprites: Vec<Sprite>,
    pub state: PlayerState,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub movement_counter: i32,
    death_counter: u32,
    invincible_counter: u32,
    pub health: u32,
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            sprites: vec![
                Sprite::ship1,
                Sprite::ship2,
                Sprite::ship3,
                Sprite::ship4,
                Sprite::ship5,
                Sprite::ship6,
                Sprite::ship7,
                Sprite::ship8,
                Sprite::ship9,
            ],
            state: PlayerState::Stationary,
            pos: (80.0, 120.0),
            vel: (0.0, 0.0),
            movement_counter: 0,
            death_counter: 0,
            invincible_counter: 0,
            health: 3,
            speed: 1.0,
        }
    }
    
    pub fn power_up(&mut self, powerup: PowerType) {
        match powerup {
            PowerType::Speed => { self.speed += 0.5 }
            PowerType::Health => { self.health += 1 }
            _ => {}
        }
    }

    pub fn move_left(&mut self) {
        self.vel.0 = -1.0 * self.speed;
    }

    pub fn move_right(&mut self) {
        self.vel.0 = 1.0 * self.speed;
    }

    pub fn move_up(&mut self) {
        self.vel.1 = -0.5 * self.speed;
    }

    pub fn move_down(&mut self) {
        self.vel.1 = 0.5 * self.speed;
    }
}

impl Alive for Player {
    fn dead(&self) -> bool {
        self.death_counter > 40
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

    fn damage(&mut self, amount: u32) {
        if self.invincible_counter == 0 {
            if self.health <= amount {
                self.health = 0;
                self.kill();
            } else {
                self.health -= amount;
                self.invincible_counter = 60;
                sound::player_damage();
            }
        }
    }

    fn kill(&mut self) {
        sound::player_death();
        self.state = PlayerState::Dying;
    }
}

impl Shoot for Player {
    fn shoot(&mut self) -> Vec<Bullet> {
        sound::player_fire();
        let mut bullet = Bullet::new((
            self.x_pos(),
            self.top() as f32,
        ));
        bullet.vel.1 = -2.0;
        if false {
            sound::player_fire();
            let mut bullet2 = Bullet::new((
                self.x_pos() - 4.0,
                self.top() as f32,
            ));
            bullet2.vel.1 = -2.0;
            bullet2.vel.0 = -0.5;
            let mut bullet3 = Bullet::new((
                self.x_pos() + 4.0,
                self.top() as f32,
            ));
            bullet3.vel.1 = -2.0;
            bullet3.vel.0 = 0.5;
            vec![bullet, bullet2, bullet3]
        } else {
            vec![bullet]
        }
    }
}

impl Render for Player {
    fn x_pos(&self) -> f32 { self.pos.0 }
    fn y_pos(&self) -> f32 { self.pos.1 }

    fn sprite(&self) -> Sprite { 
        use PlayerState::*;
        let idx = match self.state {
            Stationary => 0,
            TiltLeft => 1,
            TurnLeft => 2,
            TiltRight => 3,
            TurnRight => 4,
            Dying => match self.death_counter {
                x if x < 10 => 5,
                x if x < 20 => 6,
                x if x < 30 => 7,
                x if x < 40 => 8,
                _ => 8
            }
        };
        self.sprites[idx] 
    }

    fn draw(&self) {
        if (self.invincible_counter / 5) % 2 == 0 {
            color::set_draw(0x4320);
            self.sprite().get().draw(self.left(), self.top());
        }
    }
}

impl Movement for Player {
    fn pos_mut(&mut self) -> &mut (f32, f32) { &mut self.pos }
    fn x_vel(&self) -> f32 { self.vel.0 }
    fn y_vel(&self) -> f32 { self.vel.1 }
    fn x_vel_mut(&mut self) -> &mut f32 { &mut self.vel.0 }
    fn y_vel_mut(&mut self) -> &mut f32 { &mut self.vel.1 }

    fn update(&mut self, _: u32) { 
        // dying? something different
        if let PlayerState::Dying = self.state {
            self.death_counter += 1;
            return;
        }

        if self.invincible_counter > 0 {
            self.invincible_counter -= 1;
        }

        // update movement counter based on speed
        if self.x_vel() < 0.0 {
            if self.movement_counter > -TURN_FRAMES {
                self.movement_counter -= 1;
            }
        } else if self.x_vel() > 0.0 {
            if self.movement_counter < TURN_FRAMES {
                self.movement_counter += 1;
            }   
        } else if self.movement_counter > 0 {
            self.movement_counter -= 1;
        } else if self.movement_counter < 0 {
            self.movement_counter += 1;
        }

        // update state based on movement counter
        use PlayerState::*;
        self.state = if self.movement_counter <= -TURN_FRAMES {
            TurnLeft
        } else if self.movement_counter >= TURN_FRAMES {
            TurnRight
        } else if self.movement_counter < 0 {
            TiltLeft
        } else if self.movement_counter > 0 {
            TiltRight
        } else {
            Stationary
        };

        self.advance_bounded(true, true);
    }

}
