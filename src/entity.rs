use crate::sprite::Sprite;
use crate::wasm4::sys::*;
use crate::util;

#[derive(Default, Clone, Debug)]
pub struct Entity {
    pub sprite: Sprite,

    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl Entity {

    pub fn from_sprite(sprite: Sprite) -> Self {
        Self {
            sprite,
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
        }
    }

    /// Renders the entity's sprite at its integral position.
    pub fn draw(&self) {
        unsafe {
            *DRAW_COLORS = 0x4320; // backwards to indexed colors
        }
        blit(
            &self.sprite.data, 
            self.left(), 
            self.top(), 
            self.sprite.width, 
            self.sprite.height, 
            self.sprite.flags
        );

        // draw a second copy for left size of screen
        if self.right() > 160 {
            blit(
                &self.sprite.data, 
                self.left() - 160, 
                self.top(), 
                self.sprite.width, 
                self.sprite.height, 
                self.sprite.flags
            );
        } else if self.left() < 0 {
            blit(
                &self.sprite.data, 
                self.left() + 160, 
                self.top(), 
                self.sprite.width, 
                self.sprite.height, 
                self.sprite.flags
            );
        }
    }

    /// Updates the position.
    pub fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    /// Adjusts the position based on the velocity.  This also accounts for wrapping
    /// around the screen.
    pub fn advance(&mut self) {
        self.pos.0 += self.vel.0;
        if self.pos.0 > 160.0 { self.pos.0 -= 160.0 }
        if self.pos.0 < 0.0 { self.pos.0 += 160.0 }

        self.pos.1 += self.vel.1;
        if self.pos.1 > 160.0 - self.sprite.height as f32 { self.pos.1 = 160.0 - self.sprite.height as f32 }
        if self.pos.1 < 0.0 { self.pos.1 += 160.0 }
    }

    /// The left most pixel of the entity
    pub fn left(&self) -> i32 {
        self.pos.0 as i32 - self.sprite.width as i32 / 2
    }

    /// The right most pixel of the entity
    pub fn right(&self) -> i32 {
        self.left() + self.sprite.width as i32
    }

    /// The top most pixel of the entity
    pub fn top(&self) -> i32 {
        self.pos.1 as i32 - self.sprite.height as i32 / 2
    }

    /// The bottom most pixel of the entity
    pub fn bottom(&self) -> i32 {
        self.top() + self.sprite.height as i32
    }

    /// Not perfect.  Doesn't account for when
    /// graphic is across the screen (maybe doesn't matter?)
    pub fn collides_with(&self, other: &Entity) -> bool {
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
                let self_iter = self.sprite.range_iterator(
                    left - self.left(),
                    right - self.left(),
                    top - self.top(),
                    bottom - self.top(),
                );
                let other_iter = other.sprite.range_iterator(
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
        
    pub fn collides_with_basic(&self, other: &Entity) -> bool {
        util::range_overlap(
            self.left(), self.right(),
            other.left(), other.right(),
        ) && util::range_overlap(
            self.top(), self.bottom(),
            other.top(), other.bottom(),
        )
    }
}
