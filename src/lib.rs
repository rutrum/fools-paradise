#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::sys::*;
pub use wasm4::*;

mod util;
use util::Random;
mod entity;
pub use entity::*;

mod sprite;
pub use sprite::*;

mod sound;

const CRIMSON_PALETTE: [u32; 4] = [ 0xeff9d6, 0xba5044, 0x7a1c4b, 0x1b0326 ];

enum GameState {
    Menu,
    Playing,
}

struct Game {
    state: GameState,
    controls: Controls,
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,
    frame: u32,
    play_frame: u32,
    random: Random,
    spawn_cooldown: i32,
}

impl Game {
    fn score(&self) -> u32 {
        self.play_frame / 10
    }

    fn draw_entities(&self) {
        if self.player.alive() {
            self.player.draw();
        }
        self.enemies.iter().for_each(|e| e.draw());
        self.bullets.iter().for_each(|e| e.draw());
        self.enemy_bullets.iter().for_each(|e| e.draw());
    }

    fn update_entities(&mut self) {
        self.player.update(self.frame);
        self.enemies.iter_mut().for_each(|e| e.update(self.frame));
        self.bullets.iter_mut().for_each(|e| e.update(self.frame));
        self.enemy_bullets.iter_mut().for_each(|e| e.update(self.frame));
    }

    fn new_spawn_cooldown(&mut self) {
        self.spawn_cooldown = if self.play_frame > 60 * (100 - 30) {
            30
        } else {
            100 - self.play_frame as i32 / 60
        };
    }
}

impl Runtime for Game {
    fn start() -> Self {
        unsafe {
            *PALETTE = CRIMSON_PALETTE;
        }

        Game {
            state: GameState::Menu,
            controls: Controls::new(),
            bullets: Vec::new(),
            enemies: Vec::new(),
            enemy_bullets: Vec::new(),
            player: Player::new(),
            frame: 0,
            play_frame: 0,
            random: Random::seed(0),
            spawn_cooldown: 1,
        }
    }

    fn update(&mut self) {
        self.controls.next();
        self.frame += 1;
        self.spawn_cooldown -= 1;

        use GameState::*;
        match self.state {
            Menu => menu_update(self),
            Playing => gameplay_update(self),
        }
    }
}

fn menu_update(game: &mut Game) {
    unsafe {
        *DRAW_COLORS = 0x03; // backwards to indexed colors
    }
    text("Game Name!", 10, 10);
    text("Press action", 10, 130);
    text("button to start", 10, 140);

    let s = SpriteName::enemy1.get();

    unsafe {
        *DRAW_COLORS = 0x4320; // backwards to indexed colors
    }
    for (x, y) in [(20, 50), (100, 70), (130, 55), (55, 40), (45, 85)] {
        blit(&s.data, x, y, s.width, s.height, s.flags);
    }

    if game.controls.pressed(Button::Primary) {
        game.random = Random::seed(game.frame);
        game.state = GameState::Playing;
    }
}

fn gameplay_update(game: &mut Game) {
    if game.player.alive() {
        controls_update(game);
        unsafe {
            *DRAW_COLORS = 0x02; // backwards to indexed colors
        }
        text(game.score().to_string(), 1, 1);
        game.play_frame += 1;
    } else {
        unsafe {
            *DRAW_COLORS = 0x03; // backwards to indexed colors
        }
        text("Final score:", 30, 50);
        text(game.score().to_string(), 30, 60);
    }


    // Update physics
    game.update_entities();

    // Check collisions and update
    for enemy in &mut game.enemies {
        if enemy.ready_to_shoot() {
            game.enemy_bullets.push(enemy.shoot());
        }

        // ensure that bullets pass through dying enemies
        if !enemy.dying() {
            for bullet in &mut game.bullets {
                if enemy.collides_with(bullet) {
                    enemy.kill();
                    bullet.dead = true;
                }
            }
        }

        if !game.player.dying() && enemy.collides_with(&game.player) {
            game.player.kill();
            enemy.kill();
        }
    }

    if !game.player.dying() {
        for bullet in &mut game.enemy_bullets {
            if game.player.collides_with(bullet) {
                game.player.kill();
                bullet.dead = true;
            }
        }
    }

    // garbage collection
    game.enemies = core::mem::take(&mut game.enemies)
        .into_iter()
        .filter(|b| !b.off_screen() && b.alive())
        .collect();

    game.bullets = core::mem::take(&mut game.bullets)
        .into_iter()
        .filter(|b| !b.off_screen() && !b.dead)
        .collect();

    game.enemy_bullets = core::mem::take(&mut game.enemy_bullets)
        .into_iter()
        .filter(|b| !b.off_screen() && !b.dead)
        .collect();

    // draw
    game.draw_entities();

    if game.spawn_cooldown <= 0 {
        let mut enemy = Enemy::new();
        *enemy.x_pos_mut() = game.random.in_range(8, 160 - 8) as f32;
        game.enemies.push(enemy);
        game.new_spawn_cooldown();
    }
}

fn controls_update(game: &mut Game) {
    let mut player = &mut game.player;

    if game.controls.pressed_or_held(Button::Left) {
        player.move_left();
    } else if game.controls.pressed_or_held(Button::Right) {
        player.move_right();
    } else {
        player.vel.0 = 0.0;
    }

    if game.controls.pressed_or_held(Button::Up) {
        player.move_up();
    } else if game.controls.pressed_or_held(Button::Down) {
        player.move_down();
    } else {
        player.vel.1 = 0.0;
    }

    if game.controls.pressed(Button::Primary) {
        game.bullets.push(player.shoot());
    }
}

main! { Game }
