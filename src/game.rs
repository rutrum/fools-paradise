use crate::*;

#[derive(Clone, Copy)]
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

    powerup_cooldown: i32,
    spawn_cooldown: i32,
    time_alive: u32,
    cycle_counter: u32,
    transition_counter: i32,
}

impl Game {
    pub fn new(random: Random) -> Self {
        Self {
            cycle: Cycle::Day,
            state: State::Play,
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
            transition_counter: 0,
        }
    }

    /// Runs every frame, calls other functions to make the game function
    pub fn tick(&mut self) {
        self.spawn_cooldown -= 1;
        self.cycle_counter += 1;
        self.controls.next();
        self.frame += 1;
        self.transition_counter -= 1;

        // Print UI elements
        match self.state {
            State::EndScreen => {
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
            _ => {
                self.resolve_controls();
                color::set_draw(0x02);
                text(self.score().to_string(), 1, 1);
                text(self.time().to_string(), 1, 11);
                self.time_alive += 1;
            }
        }

        // Advance game
        match self.state {
            State::Play | State::EndScreen => {
                if let Cycle::Day = self.cycle {
                    self.powerup_cooldown -= 1;
                }
                self.resolve_cycle();
                self.spawn_entities();
                self.update();
                self.resolve_collisions();
                self.cull_entities();
            }
            State::NightTransition => {
                if self.transition_counter < 0 {
                    self.state = State::Play;
                    self.cycle = Cycle::Night;
                    Palette::Grey.set();
                }
            }
            State::DayTransition => {
                if self.transition_counter < 0 {
                    self.state = State::Play;
                    self.cycle = Cycle::Day;
                    Palette::Day.set();
                }
            }
        }
        self.draw();
    }

    fn resolve_cycle(&mut self) {
        if self.cycle_counter % CYCLE_LENGTH == CYCLE_LENGTH / 6 * 5 {
            // check if 50 passed seconds
            self.cycle = Cycle::Night;
            self.state = State::NightTransition;
            Palette::Grey.set();
            self.blasters.iter_mut().for_each(|b| b.mutate(self.cycle));
        } else if self.cycle_counter % CYCLE_LENGTH == 0 {
            // check if passed 60 seconds
            self.cycle = Cycle::Day;
            self.state = State::DayTransition;
            Palette::Day.set();
            self.blasters.iter_mut().for_each(|b| b.mutate(self.cycle));
        }
        self.transition_counter = 60;
    }

    fn score(&self) -> u32 {
        self.time_alive / 10 + 10 * self.kills
    }
    
    /// Time in seconds
    fn time(&self) -> u32 {
        self.time_alive / 60
    }

    /// Round, every 60 seconds
    fn round(&self) -> u32 {
        self.time_alive / CYCLE_LENGTH + 1
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
                        PowerType::Speed => {
                            self.player.power_up(PowerType::Speed);
                            powerup.collected = true;
                        }
                    }
                }
            }
        } else if self.player.dead() {
            self.state = State::EndScreen;
        }
    }

    fn spawn_entities(&mut self) {

        if self.spawn_cooldown <= 0 {
            if self.round() >= 3 && self.random.in_range(0, 5) < 1 {
                let enemy = Turret::spawn(&mut self.random);
                self.turrets.push(enemy);
                self.new_spawn_cooldown();
            } else {
                let enemy = Blaster::spawn(&mut self.random, self.cycle);
                self.blasters.push(enemy);
                self.new_spawn_cooldown();
            }
        }

        if let Cycle::Day = self.cycle {
            if self.powerup_cooldown <= 0 {
                let powerup = if self.round() >= 3 && !self.player.has_power_up(PowerType::Speed) {
                    PowerUp::spawn(&mut self.random, PowerType::Speed)
                } else if self.round() >= 4 && !self.player.has_power_up(PowerType::Spreader) {
                    PowerUp::spawn(&mut self.random, PowerType::Spreader)
                } else {
                    PowerUp::spawn(&mut self.random, PowerType::Health)
                };
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
