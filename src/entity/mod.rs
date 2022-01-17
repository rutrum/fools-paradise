use crate::Sprite;
use crate::wasm4::sys::*;
use crate::util;

mod player;
pub use player::Player;

mod bullet;
pub use bullet::Bullet;

mod enemy;
pub use enemy::Enemy;

pub trait Shoot {
    /// Create the bullet to be shot
    fn shoot(&mut self) -> Bullet;

    /// Determine if the bullet should be made
    fn ready_to_shoot(&self) -> bool;
}

pub trait Alive {
    /// Is the dead?
    fn dead(&self) -> bool;

    /// Is it dying?  This should be true after killing and always true when `dead()` is true.
    fn dying(&self) -> bool;

    /// Make it dead.
    fn kill(&mut self);

    /// Not dead.  Can still be alive while dying.
    fn alive(&self) -> bool {
        !self.dead()
    }
}

pub trait Entity {

    /// Get the true x position.
    fn x_pos(&self) -> f32;
    /// Get the true y position.
    fn y_pos(&self) -> f32;

    /// Mutable reference to the true x position.
    fn x_pos_mut(&mut self) -> &mut f32;
    /// Mutable reference to the true y position.
    fn y_pos_mut(&mut self) -> &mut f32;

    /// Get the x velocity.
    fn x_vel(&self) -> f32;
    /// Get the y velocity.
    fn y_vel(&self) -> f32;

    /// Get the current sprite.
    fn sprite(&self) -> &Sprite;

    /// Called every frame to update.
    fn update(&mut self, frame: u32);

    /// Adjusts the position based on the velocity.  Ensures that the x position
    /// doesn't go outside left and right walls of screen.
    fn advance(&mut self) {
        *self.x_pos_mut() += self.x_vel();
        if self.x_pos() > 160.0 { *self.x_pos_mut() = 160.0 }
        if self.x_pos() < 0.0 { *self.x_pos_mut() = 0.0 }

        *self.y_pos_mut() += self.y_vel();
    }

    /// The left most pixel of the entity.
    fn left(&self) -> i32 {
        self.x_pos() as i32 - self.sprite().width as i32 / 2
    }

    /// The right most pixel of the entity.
    fn right(&self) -> i32 {
        self.left() + self.sprite().width as i32
    }

    /// The top most pixel of the entity.
    fn top(&self) -> i32 {
        self.y_pos() as i32 - self.sprite().height as i32 / 2
    }

    /// The bottom most pixel of the entity.
    fn bottom(&self) -> i32 {
        self.top() + self.sprite().height as i32
    }

    /// Returns if the sprite would not be rendered at all
    /// if attempted to draw.
    fn off_screen(&self) -> bool {
        self.bottom() < 0 || self.top() > 160 || self.left() > 160 || self.right() < 0
    }

    /// Renders the entity's sprite at its integral position.
    fn draw(&self) {
        unsafe {
            *DRAW_COLORS = 0x4320; // backwards to indexed colors
        }
        blit(
            &self.sprite().data, 
            self.left(), 
            self.top(), 
            self.sprite().width, 
            self.sprite().height, 
            self.sprite().flags
        );
    }

    /// Checks if the current entity collides with another entity.  This is
    /// done by looping over the interesection in sprites and seeing if
    /// there each sprite has a non-transparent pixel in the same location.
    fn collides_with<T: Entity>(&self, other: &T) -> bool {
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
                let self_iter = self.sprite().range_iterator(
                    left - self.left(),
                    right - self.left(),
                    top - self.top(),
                    bottom - self.top(),
                );
                let other_iter = other.sprite().range_iterator(
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
