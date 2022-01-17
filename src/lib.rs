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
mod enemy;
use enemy::*;

const CRIMSON_PALETTE: [u32; 4] = [ 0xeff9d6, 0xba5044, 0x7a1c4b, 0x1b0326 ];

enum GameState {
    Playing
}

struct Game {
    state: GameState,
    controls: Controls,
    player: Player,
    enemies: Vec<Enemy>,
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
            enemies: Vec::new(),
            player: Player::new(),
            frame: 0,
        }
    }

    fn update(&mut self) {
        let mut player = &mut self.player;
        
        self.controls.next();

        if self.controls.pressed_or_held(Button::Left) {
            player.move_left();
        } else if self.controls.pressed_or_held(Button::Right) {
            player.move_right();
        } else {
            player.vel.0 = 0.0;
        }

        if self.controls.pressed_or_held(Button::Up) {
            player.move_up();
        } else if self.controls.pressed_or_held(Button::Down) {
            player.move_down();
        } else {
            player.vel.1 = 0.0;
        }

        if self.controls.pressed(Button::Primary) {
            self.bullets.push(player.shoot());
        }

        self.bullets.iter_mut().for_each(|bullet| {
            bullet.update(self.frame);
            bullet.draw();
        });

        self.bullets = core::mem::take(&mut self.bullets)
            .into_iter()
            .filter(|b| !b.off_screen())
            .collect();

        //text(format!("{}", self.bullets.len()), 10, 10);

        player.update(self.frame);
        player.draw();

        self.enemies.iter_mut().for_each(|enemy| {
            enemy.update(self.frame);
            enemy.draw();
            if self.bullets.iter().any(|bullet| {
               enemy.collides_with(bullet)
            }) {
                enemy.kill();
            }
        });

        self.enemies = core::mem::take(&mut self.enemies)
            .into_iter()
            .filter(|b| !b.off_screen() && !b.dead())
            .collect();

        self.frame += 1;

        if self.frame % 100 == 30 {
            let mut enemy = Enemy::new();
            *enemy.x_pos_mut() = (self.frame % 57) as f32 * 3.0;
            self.enemies.push(enemy);
        }
    }
}

main! { Game }
