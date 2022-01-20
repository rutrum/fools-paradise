use crate::*;

#[derive(Clone, Copy)]
pub enum Cycle {
    Day,
    Night,
}

pub struct Game {
    cycle: Cycle,
    controls: Controls,
    frame: u32,
    kills: u32,
    random: Random,

    player: Player,
    bullets: Vec<Bullet>,
    powerups: Vec<PowerUp>,

    blasters: Vec<Blaster>,
    turrets: Vec<Turret>,
    enemy_bullets: Vec<Bullet>,

    powerup_cooldown: i32,
    spawn_cooldown: i32,
    time_alive: u32,
    cycle_counter: u32,
}

impl Game {
    pub fn new(random: Random) -> Self {
        Self {
            cycle: Cycle::Day,
            controls: Controls::new(),
            frame: 0,
            kills: 0,
            random,

            player: Player::new(),
            bullets: Vec::new(),
            powerups: Vec::new(),

            blasters: Vec::new(),
            turrets: Vec::new(),
            enemy_bullets: Vec::new(),

            powerup_cooldown: 1000,
            spawn_cooldown: 1,
            time_alive: 0,
            cycle_counter: 0,
        }
    }

    /// Runs every frame, calls other functions to make the game function
    pub fn tick(&mut self) {
        self.spawn_cooldown -= 1;
        self.powerup_cooldown -= 1;
        self.cycle_counter += 1;
        self.controls.next();
        self.frame += 1;

        if !self.player.dead() {
            self.resolve_controls();
            color::set_draw(0x02);
            text(self.score().to_string(), 1, 1);

            text(self.time().to_string(), 1, 11);
            self.time_alive += 1;

        } else {
            color::set_draw(0x03);
            text("Final score:", 20, 50);
            text(self.score().to_string(), 120, 50);
            text("Total kills:", 20, 60);
            text(self.kills.to_string(), 120, 60);

            text("Press action to", 20, 100);
            text("play again.", 20, 110);

            if self.controls.pressed_or_held(Button::Primary) {
                *self = Self::new(self.random.clone());
            }
        }

        self.resolve_cycle();
        self.spawn_entities();
        self.update();
        self.resolve_collisions();
        self.cull_entities();
        self.draw();
    }

    fn resolve_cycle(&mut self) {
        if self.cycle_counter % 3600 == 3000 {
            // check if 50 passed seconds
            self.cycle = Cycle::Night;
            Palette::BlueMold.set();
            self.blasters.iter_mut().for_each(|b| b.mutate(self.cycle));

        } else if self.cycle_counter % 3600 == 0 {
            // check if passed 60 seconds
            self.cycle = Cycle::Day;
            Palette::Crimson.set();
            self.blasters.iter_mut().for_each(|b| b.mutate(self.cycle));
        }
    }

    fn score(&self) -> u32 {
        self.time_alive / 10 + 10 * self.kills
    }
    
    /// Time in seconds
    fn time(&self) -> u32 {
        self.time_alive / 60
    }

    fn draw(&mut self) {
        cloud::draw(self.frame, 1.4);

        if !self.player.dead() {
            self.player.draw();
        }
        self.blasters.iter().for_each(|e| e.draw());
        self.turrets.iter().for_each(|e| e.draw());
        self.bullets.iter().for_each(|e| e.draw());
        self.enemy_bullets.iter().for_each(|e| e.draw());
        self.powerups.iter().for_each(|e| e.draw());

        // health
        let heart = Sprite::heart.get();
        color::set_draw(0x330);
        for x in 0..self.player.health() {
            heart.draw((x * 8 + 10) as i32, 150);
        }
    }

    fn cull_entities(&mut self) {
        self.blasters = core::mem::take(&mut self.blasters)
            .into_iter()
            .filter(|b| !b.off_screen() && !b.dead())
            .collect();

        self.turrets = core::mem::take(&mut self.turrets)
            .into_iter()
            .filter(|b| !b.off_screen() && !b.dead())
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

    fn update(&mut self) {
        self.player.update(self.frame);
        self.blasters.iter_mut().for_each(|e| e.update(self.frame));
        self.turrets.iter_mut().for_each(|e| e.update(self.frame));
        self.bullets.iter_mut().for_each(|e| e.update(self.frame));
        self.enemy_bullets.iter_mut().for_each(|e| e.update(self.frame));
        self.powerups.iter_mut().for_each(|e| e.update(self.frame));
    }

    fn resolve_collisions(&mut self) {

        enemy_collisions(&mut self.player, &mut self.bullets, &mut self.blasters, &mut self.enemy_bullets, &mut self.kills);
        enemy_collisions(&mut self.player, &mut self.bullets, &mut self.turrets, &mut self.enemy_bullets, &mut self.kills);

        if !self.player.dying() {
            for bullet in &mut self.enemy_bullets {
                if self.player.collides_with(bullet) {
                    self.player.damage(bullet.damage);
                    bullet.dead = true;
                }
            }

            for powerup in &mut self.powerups {
                if self.player.collides_with(powerup) {
                    match powerup.t {
                        PowerType::Health => {
                            self.player.health += 1;
                            powerup.collected = true;
                        }
                        PowerType::Spreader => {
                            self.player.power_up(PowerType::Spreader);
                            powerup.collected = true;
                        }
                    }
                }
            }
        }
    }

    fn spawn_entities(&mut self) {
        if self.spawn_cooldown <= 0 {
            let enemy = Blaster::spawn(&mut self.random, self.cycle);
            self.blasters.push(enemy);
            self.new_spawn_cooldown();
        }

        if self.spawn_cooldown <= 0 {
            let enemy = Turret::spawn(&mut self.random);
            self.turrets.push(enemy);
            self.new_spawn_cooldown();
        }

        if let Cycle::Day = self.cycle {
            if self.powerup_cooldown <= 0 {
                let powerup = PowerUp::spawn(&mut self.random, PowerType::Health);
                self.powerups.push(powerup);
                self.powerup_cooldown = 1000;
            }
        }
    }

    fn new_spawn_cooldown(&mut self) {
        self.spawn_cooldown = if self.frame > 60 * (100 - 30) {
            30
        } else {
            100 - self.frame as i32 / 60
        };
    }


    fn resolve_controls(&mut self) {
        let mut player = &mut self.player;
        let controls = &mut self.controls;

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

fn enemy_collisions<T>(
    player: &mut Player, 
    bullets: &mut Vec<Bullet>, 
    enemies: &mut Vec<T>, 
    enemy_bullets: &mut Vec<Bullet>,
    kills: &mut u32,
) where T: Movement + Render + Alive + Shoot {
    for enemy in enemies {
        enemy_bullets.append(&mut enemy.shoot());

        // ensure that bullets pass through dying enemies
        if !enemy.dying() {
            for bullet in bullets.iter_mut() {
                if enemy.collides_with(bullet) {
                    enemy.damage(bullet.damage);
                    if enemy.dying() { *kills += 1 }
                    bullet.dead = true;
                }
            }
        }

        if !player.dying() && enemy.collides_with(player) {
            enemy.kill();
            player.damage(1);
        }
    }
}
