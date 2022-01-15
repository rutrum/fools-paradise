use crate::sprite::Sprite;
use crate::wasm4::sys::*;
use crate::util;

pub trait Entity {

    fn x_pos(&self) -> f32;
    fn y_pos(&self) -> f32;

    fn x_pos_mut(&mut self) -> &mut f32;
    fn y_pos_mut(&mut self) -> &mut f32;

    fn x_vel(&self) -> f32;
    fn y_vel(&self) -> f32;

    /// Get the current sprite
    fn sprite(&self) -> &Sprite;

    /// Called every frame to update.
    fn update(&mut self, frame: u32);

    /// Adjusts the position based on the velocity.  This also accounts for wrapping
    /// around the screen.
    fn advance(&mut self) {
        *self.x_pos_mut() += self.x_vel();
        if self.x_pos() > 160.0 { *self.x_pos_mut() -= 160.0 }
        if self.x_pos() < 0.0 { *self.x_pos_mut() += 160.0 }

        *self.y_pos_mut() += self.y_vel();
        if self.y_pos() > 160.0 - self.sprite().height as f32 { *self.y_pos_mut() = 160.0 - self.sprite().height as f32 }
        if self.y_pos() < 0.0 { *self.y_pos_mut() += 160.0 }
    }

    /// The left most pixel of the entity
    fn left(&self) -> i32 {
        self.x_pos() as i32 - self.sprite().width as i32 / 2
    }

    /// The right most pixel of the entity
    fn right(&self) -> i32 {
        self.left() + self.sprite().width as i32
    }

    /// The top most pixel of the entity
    fn top(&self) -> i32 {
        self.y_pos() as i32 - self.sprite().height as i32 / 2
    }

    /// The bottom most pixel of the entity
    fn bottom(&self) -> i32 {
        self.top() + self.sprite().height as i32
    }

    /// Returns if the sprite would not be rendered at all
    /// if attempted to draw.
    fn off_screen(&self) -> bool {
        false
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

        if self.right() > 160 {
            blit(
                &self.sprite().data, 
                self.left() - 160, 
                self.top(), 
                self.sprite().width, 
                self.sprite().height, 
                self.sprite().flags
            );
        } else if self.left() < 0 {
            blit(
                &self.sprite().data, 
                self.left() + 160, 
                self.top(), 
                self.sprite().width, 
                self.sprite().height, 
                self.sprite().flags
            );
        }
    }

    /// Not perfect.  Doesn't account for when
    /// graphic is across the screen (maybe doesn't matter?)
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
