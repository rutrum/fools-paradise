use crate::*;

#[derive(Clone, PartialEq, Copy)]
pub enum Cycle {
    Day,
    Night,
}

pub enum State {
    Play,
    EndScreen,
    NightTransition,
    DayTransition,
}

const CYCLE_LENGTH: u32 = 3600;

pub struct Game {
    cycle: Cycle,
    state: State,
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

    day: u32,
    spawn_cooldown: i32,
    time_alive: u32,
    cycle_counter: u32,
    transition_counter: i32,
}

impl Game {
    pub fn new(random: Random) -> Self {
        Palette::Grey.set();
        Self {
            cycle: Cycle::Day,
            state: State::DayTransition,
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

            day: 1,
            spawn_cooldown: 1,
            time_alive: 0,
            cycle_counter: 0,
            transition_counter: 60,
        }
    }

    /// Runs every frame, calls other functions to make the game function
    pub fn tick(&mut self) {
        self.spawn_cooldown -= 1;
        self.transition_counter -= 1;

        if let State::Play | State::EndScreen = self.state {
            self.controls.next();
        }

        // Print UI elements
        match self.state {
            State::EndScreen => {
                color::set_draw(0x03);

                match self.cycle {
                    Cycle::Day => {
                        text(format!("Day {}", self.day), 30, 30);
                    }
                    Cycle::Night => {
                        text(format!("Night {}", self.day), 30, 30);
                    }
                }

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
            _ => {
                self.cycle_counter += 1;
                self.resolve_controls();
                color::set_draw(0x02);
                if self.time_alive > 0 {
                    text(self.score().to_string(), 1, 1);
                    match self.cycle {
                        Cycle::Day => {
                            text(format!("Day {}", self.day), 1, 11);
                        }
                        Cycle::Night => {
                            text(format!("Night {}", self.day), 1, 11);
                        }
                    }
                }
            }
        }

        if let State::Play = self.state {
            self.time_alive += 1;
        }

        // Advance game
        match self.state {
            State::Play | State::EndScreen => {
                self.frame += 1;
                self.resolve_cycle();
                self.spawn_entities();
                self.update();
                self.resolve_collisions();
                self.cull_entities();
                self.draw();
            }
            State::NightTransition => {
                if self.transition_counter >= 60 {
                    Palette::Grey.transition_to(((self.transition_counter - 60) / 15) as usize);
                } else if self.transition_counter > 0 {
                    Palette::Night.transition_to((self.transition_counter / 15) as usize);
                }
                if self.transition_counter == 0 {
                    self.state = State::Play;
                    self.cycle = Cycle::Night;
                }
                self.draw();
                self.draw_sun_moon();
            }
            State::DayTransition => {
                if self.transition_counter >= 60 {
                    Palette::Grey.transition_to(((self.transition_counter - 60) / 15) as usize);
                } else if self.transition_counter > 0 {
                    Palette::Day.transition_to((self.transition_counter / 15) as usize);
                }
                if self.transition_counter == 0 {
                    self.state = State::Play;
                    self.cycle = Cycle::Day;
                }
                self.draw();
                self.draw_sun_moon();
            }
        }
    }

    fn draw_sun_moon(&mut self) {
        let center = (80, 80);
        let f = self.transition_counter;

        match self.state {
            State::DayTransition => {
                color::set_draw(0x20);
                Sprite::sun.get().draw(center.0 - 4, (center.1 - 10 + f / 10) as i32);
            }
            State::NightTransition => {
                color::set_draw(0x20);
                Sprite::moon.get().draw(center.0 - 4, (center.1 - 10 + f / 10) as i32);
            }
            _ => {}
        }

        color::set_draw(0x4320);
        Sprite::land.get().draw(center.0 - 16, center.1);

        match self.cycle {
            Cycle::Day => {
                color::set_draw(0x01);
                text(format!("Day {}", self.day), 58, 95);
                color::set_draw(0x12);
                text(format!("Day {}", self.day), 59, 96);
            }
            Cycle::Night => {
                color::set_draw(0x01);
                text(format!("Night {}", self.day), 52, 95);
                color::set_draw(0x12);
                text(format!("Night {}", self.day), 53, 96);
            }
        }
    }

    fn resolve_cycle(&mut self) {
        if self.cycle_counter % CYCLE_LENGTH == CYCLE_LENGTH / 4 * 3 {
            // check if 45 passed seconds
            self.cycle = Cycle::Night;
            self.state = State::NightTransition;
            self.blasters.iter_mut().for_each(|b| b.mutate(self.cycle));
            self.turrets.iter_mut().for_each(|b| b.mutate(self.cycle));
        } else if self.cycle_counter % CYCLE_LENGTH == 0 {
            // check if passed 60 seconds
            self.cycle = Cycle::Day;
            self.state = State::DayTransition;
            self.day += 1;
            self.blasters.iter_mut().for_each(|b| b.mutate(self.cycle));
            self.turrets.iter_mut().for_each(|b| b.mutate(self.cycle));
        }
        self.transition_counter = 119;
    }

    fn score(&self) -> u32 {
        self.time_alive / 10 + 10 * self.kills
    }
    
    /// Time in seconds
    fn time(&self) -> u32 {
        self.time_alive / 60
    }

    /// Round, every 60 seconds
    fn round(&self) -> i32 {
        self.time_alive as i32 / CYCLE_LENGTH as i32 + 1
    }

    fn is_day(&self) -> bool {
        self.cycle == Cycle::Day
    }

    fn is_night(&self) -> bool {
        self.cycle == Cycle::Night
    }

    fn draw(&mut self) {
        cloud::draw(self.frame, 1.4);

        color::set_draw(0x4320);
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
        for x in 0..self.player.health() {
            heart.draw((x * 8 + 10) as i32, 150);
        }

        // speed boosts
        let speedup = Sprite::speed.get();
        for x in 0..self.player.total_speed_powerups() {
            speedup.draw((x * 8 + 10) as i32, 140);
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


        for enemy in self.blasters.iter().filter(|e| e.dead()) {
            let p = 0.15 - 0.03 * self.player.health() as f32;
            if self.is_day() && self.random.uniform_lt(p) {
                let pos = enemy.pos();
                let pt = self.get_power_type();

                if let Some(pt) = pt {
                    self.powerups.push(PowerUp::spawn(pt, pos));
                }
            }
        }

        for enemy in self.turrets.iter().filter(|e| e.dead()) {
            let p = 0.18 - 0.03 * self.player.health() as f32;
            if self.is_day() && self.random.uniform_lt(p) {
                let pos = enemy.pos();
                let pt = self.get_power_type();

                if let Some(pt) = pt {
                    self.powerups.push(PowerUp::spawn(pt, pos));
                }
            }
        }

        if !self.player.dying() {
            for bullet in &mut self.enemy_bullets {
                if self.player.collides_with(bullet) {
                    self.player.damage(bullet.damage);
                    bullet.dead = true;
                }
            }

            for powerup in &mut self.powerups {
                if self.player.collides_with(powerup) {
                    self.player.power_up(powerup.t);
                    powerup.collected = true;
                }
            }
        } else if self.player.dead() {
            self.state = State::EndScreen;
        }
    }

    fn spawn_entities(&mut self) {
        if self.spawn_cooldown <= 0 {
            if self.round() >= 0 && self.turrets.len() < 3 && self.random.in_range(0, (10 - self.round()).max(3) as u32) < 1 {
                let enemy = Turret::spawn(&mut self.random);
                self.turrets.push(enemy);
                self.new_spawn_cooldown();
            } else {
                let enemy = Blaster::spawn(&mut self.random, self.cycle, &self.player);
                self.blasters.push(enemy);
                self.new_spawn_cooldown();
            }
        }
    }

    fn get_power_type(&self) -> Option<PowerType> {
        if self.round() >= 3 && self.player.speed < 1.5 {
            Some(PowerType::Speed)
        } else if self.round() >= 5 && self.player.speed < 2.0 {
            Some(PowerType::Speed)
        } else if self.player.health < 5 {
            Some(PowerType::Health)
        } else {
            None
        }
    }

    fn new_spawn_cooldown(&mut self) {
        let cooldown = if self.frame > 300 * (100 - 30) {
            30
        } else {
            100 - self.frame as i32 / 300
        };

        self.spawn_cooldown = match self.cycle {
            Cycle::Day => cooldown,
            Cycle::Night => cooldown / 4 * 3,
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
