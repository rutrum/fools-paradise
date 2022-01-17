use super::*;
use crate::Sprite;
use crate::SpriteList;

const TURN_FRAMES: i32 = 15;

#[derive(Clone, Debug)]
pub enum PlayerState {
    Stationary,
    TiltLeft,
    TurnLeft,
    TiltRight,
    TurnRight,
}

impl PlayerState {
    fn sprite_idx(&self) -> usize {
        use PlayerState::*;
        match self {
            Stationary => 0,
            TiltLeft => 1,
            TurnLeft => 2,
            TiltRight => 3,
            TurnRight => 4,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub sprites: Vec<Sprite>,
    pub state: PlayerState,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub movement_counter: i32,
    dead: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            sprites: vec![
                SpriteList::ship1.get(),
                SpriteList::ship2.get(),
                SpriteList::ship3.get(),
                SpriteList::ship4.get(),
                SpriteList::ship5.get(),
            ],
            state: PlayerState::Stationary,
            pos: (80.0, 80.0),
            vel: (0.0, 0.0),
            movement_counter: 0,
            dead: false,
        }
    }

    pub fn move_left(&mut self) {
        self.vel.0 = -2.0;
    }

    pub fn move_right(&mut self) {
        self.vel.0 = 2.0;
    }

    pub fn move_up(&mut self) {
        self.vel.1 = -0.5;
    }

    pub fn move_down(&mut self) {
        self.vel.1 = 0.5;
    }
}

impl Alive for Player {
    fn dead(&self) -> bool {
        self.dead
    }

    fn kill(&mut self) {
        self.dead = true;
    }
}

impl Shoot for Player {
    fn shoot(&mut self) -> Bullet {
        let mut bullet = Bullet::new((
            self.x_pos(),
            self.top() as f32,
        ));
        bullet.vel.1 = -2.0;
        bullet
    }

    fn ready_to_shoot(&self) -> bool {
        true
    }
}

impl Entity for Player {
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

        self.advance();
    }
}