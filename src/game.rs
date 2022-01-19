use crate::Player;
use crate::Controls;
use crate::Button;
use crate::entity::*;

enum Cycle {
    Day,
    Night,
}

struct Game {
    cycle: Cycle,
    controls: Controls,

    player: Player,
    bullets: Vec<Bullet>,

    enemies: Vec<Enemy>,
}

impl Game {
    fn new() -> Self {
        Self {
            cycle: Cycle::Day,
            controls: Controls::new(),

            player: Player::new(),
            bullets: Vec::new(),

            enemies: Vec::new(),
        }
    }

    /// Runs every frame, calls other functions to make the game function
    fn tick() {
        
    }

    fn draw_entities() {

    }

    fn cull_entities() {

    }

    fn advance_physics() {

    }

    fn resolve_collisions() {

    }

    fn spawn_entities() {

    }

    fn resolve_controls(&mut self) {
        let mut player = &mut self.player;
        let mut controls = &mut self.controls;

        if controls.pressed_or_held(Button::Left) {
            player.move_left();
        } else if controls.pressed_or_held(Button::Right) {
            player.move_right();
        } else {
            player.vel.0 = 0.0;
        }

        if controls.pressed_or_held(Button::Up) {
            player.move_up();
        } else if controls.pressed_or_held(Button::Down) {
            player.move_down();
        } else {
            player.vel.1 = 0.0;
        }

        if controls.pressed(Button::Primary) {
            self.bullets.append(&mut player.shoot());
        }
    }

}
