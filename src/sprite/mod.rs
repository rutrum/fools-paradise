mod sprite_data;
pub use sprite_data::Sprite;

use crate::wasm4::sys::*;

use crate::util;

#[derive(Debug, Clone, Default)]
pub struct SpriteData {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub data: Vec<u8>,
}

impl SpriteData {
    pub fn range_iterator(&self, left: i32, right: i32, top: i32, bottom: i32) -> SpriteIterator {
        SpriteIterator {
            left: left as u32, 
            top: top as u32,
            width: (right - left) as u32,
            height: (bottom - top) as u32,
            idx: 0,
            sprite: self.clone(),
        }
    }

    pub fn pixel(&self, x: usize, y: usize) -> u8 {
        let pixel_idx = y * self.width as usize + x;
        let byte_idx = pixel_idx / 4;
        let byte_offset = pixel_idx % 4;
        
        let sprite_byte = self.data[byte_idx];
        util::bit_range(
            sprite_byte, 
            byte_offset as usize * 2, 
            (byte_offset as usize + 1) * 2,
        )
    }

    /// Draws the current sprite with the top left pixel at (x, y)
    pub fn draw(&self, x: i32, y: i32) {
        blit(
            &self.data, 
            x,
            y,
            self.width, 
            self.height, 
            self.flags,
        );
    }
}

impl IntoIterator for SpriteData {
    type IntoIter = SpriteIterator;
    type Item = u8;
    
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            left: 0,
            width: self.width,
            top: 0,
            height: self.height,
            idx: 0,
            sprite: self,
        }
    }
}

pub struct SpriteIterator {
    sprite: SpriteData,
    left: u32,
    width: u32,
    top: u32,
    height: u32,
    idx: u32,
}

impl Iterator for SpriteIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.width * self.height {
            return None
        }

        let x = self.idx % self.width + self.left;
        let y = self.idx / self.width + self.top;
        
        let pixel = self.sprite.pixel(x as usize, y as usize);

        self.idx += 1;
        // return value
        Some(pixel)
    }
}
