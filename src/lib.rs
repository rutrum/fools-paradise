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
mod color;
mod cloud;

enum GameState {
    Menu,
    Playing,
}

struct Game {
    state: GameState,
    controls: Controls,
    player: Player,
    enemies: Vec<Enemy>,
    turrets: Vec<Turret>,
    bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,
    powerups: Vec<PowerUp>,
    frame: u32,
    play_frame: u32,
    random: Random,
    spawn_cooldown: i32,
    kills: u32,
    powerup_cooldown: i32,
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::Menu,
            controls: Controls::new(),
            bullets: Vec::new(),
            enemies: Vec::new(),
            turrets: Vec::new(),
            enemy_bullets: Vec::new(),
            player: Player::new(),
            powerups: Vec::new(),
            frame: 0,
            play_frame: 0,
            random: Random::seed(0),
            spawn_cooldown: 1,
            powerup_cooldown: 1000,
            kills: 0,
        }
    }
    
    fn score(&self) -> u32 {
        self.play_frame / 10 + 10 * self.kills
    }

    fn draw_entities(&self) {
        if self.player.alive() {
            self.player.draw();
        }
        self.enemies.iter().for_each(|e| e.draw());
        self.turrets.iter().for_each(|e| e.draw());
        self.bullets.iter().for_each(|e| e.draw());
        self.enemy_bullets.iter().for_each(|e| e.draw());
        self.powerups.iter().for_each(|e| e.draw());

        // health
        let heart = SpriteName::heart.get();
        color::set_draw(0x330);
        for x in 0..self.player.health() {
            blit(&heart.data, (x * 8 + 10) as i32, 150, heart.width, heart.height, heart.flags);
        }
    }

    fn update_entities(&mut self) {
        self.player.update(self.frame);
        self.enemies.iter_mut().for_each(|e| e.update(self.frame));
        self.turrets.iter_mut().for_each(|e| e.update(self.frame));
        self.bullets.iter_mut().for_each(|e| e.update(self.frame));
        self.enemy_bullets.iter_mut().for_each(|e| e.update(self.frame));
        self.powerups.iter_mut().for_each(|e| e.update(self.frame));
    }

    fn new_spawn_cooldown(&mut self) {
        self.spawn_cooldown = if self.play_frame > 60 * (100 - 30) {
            30
        } else {
            100 - self.play_frame as i32 / 60
        };
    }

    fn restart(&mut self) {
        self.player = Player::new();
        self.enemies = Vec::new();
        self.bullets = Vec::new();
        self.turrets = Vec::new();
        self.enemy_bullets = Vec::new();
        self.powerups = Vec::new();
        self.play_frame = 0;
        self.spawn_cooldown = 1;
        self.powerup_cooldown = 1000;
        self.kills = 0;
    }

    fn cull_entities(&mut self) {
        self.enemies = core::mem::take(&mut self.enemies)
            .into_iter()
            .filter(|b| !b.off_screen() && b.alive())
            .collect();

        self.turrets = core::mem::take(&mut self.turrets)
            .into_iter()
            .filter(|b| !b.off_screen() && b.alive())
            .collect();

        self.bullets = core::mem::take(&mut self.bullets)
            .into_iter()
            .filter(|b| !b.off_screen() && !b.dead)
            .collect();

        self.enemy_bullets = core::mem::take(&mut self.enemy_bullets)
            .into_iter()
            .filter(|b| !b.off_screen() && !b.dead)
            .collect();

        self.powerups = core::mem::take(&mut self.powerups)
            .into_iter()
            .filter(|b| !b.off_screen() && !b.collected)
            .collect();
    }
}

impl Runtime for Game {
    fn start() -> Self {
        color::Palette::Crimson.set();
        Game::new()
    }

    fn update(&mut self) {
        self.controls.next();
        self.frame += 1;
        self.spawn_cooldown -= 1;
        self.powerup_cooldown -= 1;

        use GameState::*;
        match self.state {
            Menu => menu_update(self),
            Playing => gameplay_update(self),
        }
    }
}

fn menu_update(game: &mut Game) {
    // draw clouds using perlin noise cause why not
    cloud::draw(game.frame, -1.0);

    color::set_draw(0x03);
    text("Fool's Paradise", 10, 10);
    text("Start", 10, 130);

    let s = SpriteName::enemy1.get();

    color::set_draw(0x4320);
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
        color::set_draw(0x02);
        text(game.score().to_string(), 1, 1);
        game.play_frame += 1;
    } else {
        color::set_draw(0x03);
        text("Final score:", 20, 50);
        text(game.score().to_string(), 120, 50);
        text("Total kills:", 20, 60);
        text(game.kills.to_string(), 120, 60);

        text("Press action to", 20, 100);
        text("play again.", 20, 110);

        if game.controls.pressed_or_held(Button::Primary) {
            game.restart();
        }
    }

    cloud::draw(game.frame, 1.4);

    // Update physics
    game.update_entities();

    // Check collisions and update
    for enemy in &mut game.enemies {
        if enemy.ready_to_shoot() {
            game.enemy_bullets.append(&mut enemy.shoot());
        }

        // ensure that bullets pass through dying enemies
        if !enemy.dying() {
            for bullet in &mut game.bullets {
                if enemy.collides_with(bullet) {
                    enemy.damage(bullet.damage);
                    if enemy.dying() { game.kills += 1 }
                    bullet.dead = true;
                }
            }
        }

        if !game.player.dying() && enemy.collides_with(&game.player) {
            enemy.kill();
            game.player.damage(1);
        }
    }

    for enemy in &mut game.turrets {
        if enemy.ready_to_shoot() {
            game.enemy_bullets.append(&mut enemy.shoot());
        }

        // ensure that bullets pass through dying enemies
        if !enemy.dying() {
            for bullet in &mut game.bullets {
                if enemy.collides_with(bullet) {
                    enemy.damage(bullet.damage);
                    if enemy.dying() { game.kills += 1 }
                    bullet.dead = true;
                }
            }
        }

        if !game.player.dying() && enemy.collides_with(&game.player) {
            enemy.kill();
            game.player.damage(1);
        }
    }

    if !game.player.dying() {
        for bullet in &mut game.enemy_bullets {
            if game.player.collides_with(bullet) {
                game.player.damage(bullet.damage);
                bullet.dead = true;
            }
        }

        for powerup in &mut game.powerups {
            if game.player.collides_with(powerup) {
                match powerup.t {
                    PowerType::Health => {
                        game.player.health += 1;
                        powerup.collected = true;
                    }
                    PowerType::Spreader => {
                        game.player.power_up(PowerType::Spreader);
                        powerup.collected = true;
                    }
                }
            }
        }
    }

    /*let total_entities = 1 + game.bullets.len() + game.enemy_bullets.len()
        + game.enemies.len() + game.powerups.len();
    text(total_entities.to_string(), 50, 50);
    */

    game.cull_entities();

    // draw
    game.draw_entities();

    if game.spawn_cooldown <= 0 {
        let mut enemy = Turret::new(&mut game.random);
        game.turrets.push(enemy);
        game.new_spawn_cooldown();
    }

    if game.spawn_cooldown <= 0 {
        let mut enemy = Enemy::new();
        *enemy.x_pos_mut() = game.random.in_range(8, 160 - 8) as f32;
        game.enemies.push(enemy);
        game.new_spawn_cooldown();
    }

    if game.powerup_cooldown <= 0 {
        let mut powerup = PowerUp::new(PowerType::Health);
        *powerup.x_pos_mut() = game.random.in_range(20, 160 - 20) as f32;
        game.powerups.push(powerup);
        game.powerup_cooldown = 1000;
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
        game.bullets.append(&mut player.shoot());
    }
}

main! { Game }
