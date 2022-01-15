#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::sys::*;
use wasm4::runtime::*;
use wasm4::controls::*;

pub mod util;
pub mod entity;
use entity::*;
mod sprite;
use sprite::*;

mod sprite_consts;
use sprite_consts::SpriteList;

const CRIMSON_PALETTE: [u32; 4] = [ 0xeff9d6, 0xba5044, 0x7a1c4b, 0x1b0326 ];

enum GameState {
    Playing
}

struct Game {
    state: GameState,
    controls: Controls,
    player: Entity,
    bullet: Entity,
}

impl Runtime for Game {
    fn start() -> Self {
        unsafe {
            *PALETTE = CRIMSON_PALETTE;
        }

        let mut player = Entity::from_sprite(SpriteList::ship.get());
        player.set_pos((70.0, 80.0));

        let mut bullet = Entity::from_sprite(SpriteList::bullet1.get());
        bullet.set_pos((90.0, 89.0));

        Game {
            state: GameState::Playing,
            controls: Controls::new(),
            bullet, 
            player,
        }
    }

    fn update(&mut self) {
        self.controls.next();

        if self.controls.pressed_or_held(Button::Left) {
            self.player.vel.0 = -1.0;
        } else if self.controls.pressed_or_held(Button::Right) {
            self.player.vel.0 = 1.0;
        } else {
            self.player.vel.0 = 0.0;
        }

        if self.controls.pressed_or_held(Button::Up) {
            self.player.vel.1 = -0.5;
        } else if self.controls.pressed_or_held(Button::Down) {
            self.player.vel.1 = 0.5;
        } else {
            self.player.vel.1 = 0.0;
        }

        if self.controls.pressed(Button::Primary) {
            text("shoot!", 10, 150);
        }

        if self.player.collides_with(&self.bullet) {
            text("collision!", 10, 10);
        }

        self.player.advance();
        self.bullet.draw();
        self.player.draw();
    }
}

main! { Game }
