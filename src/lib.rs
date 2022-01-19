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
mod game;
pub use game::Game;

enum GameState {
    Menu,
    Playing(Game),
}

struct App {
    controls: Controls,
    frame: u32,
    state: GameState,
}

impl App {
    fn new() -> Self {
        Self {
            controls: Controls::new(),
            frame: 0,
            state: GameState::Menu,
        }
    }
}

impl Runtime for App {
    fn start() -> Self {
        color::Palette::Crimson.set();
        App::new()
    }

    fn update(&mut self) {
        self.controls.next();
        self.frame += 1;

        use GameState::*;
        match &mut self.state {
            Menu => menu_update(self),
            Playing(game) => game.tick(),
        }
    }
}

fn menu_update(game: &mut App) {
    // draw clouds using perlin noise cause why not
    cloud::draw(game.frame, -1.0);

    color::set_draw(0x03);
    text("Fool's Paradise", 10, 10);
    text("Start", 10, 130);

    let s = Sprite::enemy1.get();

    color::set_draw(0x4320);
    for (x, y) in [(20, 50), (100, 70), (130, 55), (55, 40), (45, 85)] {
        blit(&s.data, x, y, s.width, s.height, s.flags);
    }

    if game.controls.pressed(Button::Primary) {
        game.state = GameState::Playing(Game::new(Random::seed(game.frame)));
    }
}

main! { App }
