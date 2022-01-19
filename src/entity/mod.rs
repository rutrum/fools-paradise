use crate::SpriteName;
use crate::util;
use crate::color;

mod player;
pub use player::Player;

mod bullet;
pub use bullet::Bullet;

mod powerup;
pub use powerup::{PowerUp, PowerType};

mod enemy;
pub use enemy::Enemy;

mod turret;
pub use turret::Turret;

pub trait Movement : Render {

    /// Mutable reference to the true x position.
    fn x_pos_mut(&mut self) -> &mut f32;

    /// Mutable reference to the true y position.
    fn y_pos_mut(&mut self) -> &mut f32;

    /// Get the x velocity.
    fn x_vel(&self) -> f32;

    /// Get the y velocity.
    fn y_vel(&self) -> f32;

    /// Called every frame to update.
    fn update(&mut self, frame: u32);

    /// Adjusts the position based on the velocity.  Ensures that the x position
    /// doesn't go outside left and right walls of screen.
    fn advance_bounded(&mut self, x_bounded: bool, y_bounded: bool) {
        *self.x_pos_mut() += self.x_vel();
        if x_bounded {
            if self.x_pos() > 160.0 { *self.x_pos_mut() = 160.0 }
            if self.x_pos() < 0.0 { *self.x_pos_mut() = 0.0 }
        }

        *self.y_pos_mut() += self.y_vel();
        if y_bounded {
            if self.y_pos() > 160.0 { *self.y_pos_mut() = 160.0 }
            if self.y_pos() < 0.0 { *self.y_pos_mut() = 0.0 }
        }
    }

    /// Adjusts the position based on the velocity.  Ensures that the x position
    /// doesn't go outside left and right walls of screen.
    fn advance(&mut self) {
        self.advance_bounded(false, false);
    }
}

pub trait Render {

    /// The sprite to render in the default draw implementation.
    fn sprite(&self) -> SpriteName;

    /// Get the true x position (center).
    fn x_pos(&self) -> f32;

    /// Get the true y position (center).
    fn y_pos(&self) -> f32;

    /// Get the current width.
    fn width(&self) -> u32 {
        self.sprite().get().width
    }

    /// Get the current height.
    fn height(&self) -> u32 {
        self.sprite().get().height
    }

    /// The left most pixel of the entity.
    fn left(&self) -> i32 {
        self.x_pos() as i32 - self.width() as i32 / 2
    }

    /// The right most pixel of the entity.
    fn right(&self) -> i32 {
        self.left() + self.width() as i32
    }

    /// The top most pixel of the entity.
    fn top(&self) -> i32 {
        self.y_pos() as i32 - self.height() as i32 / 2
    }

    /// The bottom most pixel of the entity.
    fn bottom(&self) -> i32 {
        self.top() + self.height() as i32
    }

    /// Returns if the sprite would not be rendered at all
    /// if attempted to draw.
    fn off_screen(&self) -> bool {
        self.bottom() < 0 || self.top() > 160 || self.left() > 160 || self.right() < 0
    }

    /// Draws the sprite at the location.
    fn draw(&self) {
        color::set_draw(0x4320);
        self.sprite().get().draw(self.left(), self.top());
    }

    /// Checks if the current entity collides with another entity.  This is
    /// done by looping over the interesection in sprites and seeing if
    /// there each sprite has a non-transparent pixel in the same location.
    fn collides_with<T: Render>(&self, other: &T) -> bool {
        let x_overlap = util::range_intersection(
            self.left(), self.right(),
            other.left(), other.right(),
        );
        let y_overlap = util::range_intersection(
            self.top(), self.bottom(),
            other.top(), other.bottom(),
        );

        match (x_overlap, y_overlap) {
            (Some((left, right)), Some((top, bottom))) => {
                let self_iter = self.sprite().get().range_iterator(
                    left - self.left(),
                    right - self.left(),
                    top - self.top(),
                    bottom - self.top(),
                );
                let other_iter = other.sprite().get().range_iterator(
                    left - other.left(),
                    right - other.left(),
                    top - other.top(),
                    bottom - other.top(),
                );
                self_iter.zip(other_iter).any(|(s, o)| {
                    s != 0 && o != 0
                })
            }
            _ => false,
        }
    }
}

pub trait Shoot {
    /// Create the bullet to be shot, if it isn't appropriate to shoot,
    /// it returns an empty vector.
    fn shoot(&mut self) -> Vec<Bullet>;
}

pub trait Alive {
    /// Is the dead?
    fn dead(&self) -> bool;

    /// Is it dying?  This should be true after killing and always true when `dead()` is true.
    fn dying(&self) -> bool;

    /// Amount of health the enemy has.
    fn health(&self) -> u32;

    /// Mutable reference to amount of health the enemy has.
    fn health_mut(&mut self) -> &mut u32;

    /// Make it dead, zero health.
    fn kill(&mut self);

    /// Decrease the health by amount.
    fn damage(&mut self, amount: u32) {
        if amount >= self.health() {
            self.kill();
        } else {
            *self.health_mut() -= amount;
        }
    }
}
