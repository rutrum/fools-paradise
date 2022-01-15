#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::sys::*;
use wasm4::runtime::*;
pub use wasm4::controls::*;

pub mod util;
pub mod entity;
use entity::*;
mod sprite;
use sprite::*;

mod sprite_consts;
use sprite_consts::SpriteList;

mod player;
use player::*;
mod bullet;
use bullet::*;

const CRIMSON_PALETTE: [u32; 4] = [ 0xeff9d6, 0xba5044, 0x7a1c4b, 0x1b0326 ];

enum GameState {
    Playing
}

struct Game {
    state: GameState,
    controls: Controls,
    player: Player,
    bullets: Vec<Bullet>,
    frame: u32,
}

impl Runtime for Game {
    fn start() -> Self {
        unsafe {
            *PALETTE = CRIMSON_PALETTE;
        }

        Game {
            state: GameState::Playing,
            controls: Controls::new(),
            bullets: Vec::new(),
            player: Player::new(),
            frame: 0,
        }
    }

    fn update(&mut self) {
        let mut player = &mut self.player;
        
        self.controls.next();

        if self.controls.pressed_or_held(Button::Left) {
            player.vel.0 = -1.0;
        } else if self.controls.pressed_or_held(Button::Right) {
            player.vel.0 = 1.0;
        } else {
            player.vel.0 = 0.0;
        }

        if self.controls.pressed_or_held(Button::Up) {
            player.vel.1 = -0.5;
        } else if self.controls.pressed_or_held(Button::Down) {
            player.vel.1 = 0.5;
        } else {
            player.vel.1 = 0.0;
        }

        if self.controls.pressed(Button::Primary) {
            text("shoot!", 10, 150);
            let mut bullet = Bullet::new((
                player.x_pos(),
                player.top() as f32,
            ));
            bullet.vel.1 = -2.0;
            self.bullets.push(bullet);
        }

        self.bullets.iter_mut().for_each(|bullet| {
            bullet.update(self.frame);
            bullet.draw();
        });

        text(format!("{}", self.bullets.len()), 10, 10);

        player.update(self.frame);
        player.draw();

        self.frame += 1;
    }
}

main! { Game }
