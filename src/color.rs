use crate::wasm4::sys::*;

pub enum Palette {
    Day,
    Grey,
    Night,
}

impl Palette {
    fn bytes(&self) -> [u32; 4] {
        use Palette::*;
        match self {
            // Crimson: https://lospec.com/palette-list/crimson
            Day => [ 0xeff9d6, 0xba5044, 0x7a1c4b, 0x1b0326 ],

            Grey => [ 0xf1ffe0, 0x988171, 0x463534, 0x1e1721 ],

            Night => [ 0x622e4c, 0x7550e8, 0x608fcf, 0x8be5ff ],
        }
    }

    pub fn set(&self) {
        let p = self.bytes();
        unsafe {
            *PALETTE = p;
        }
    }
}

pub fn set_draw(v: u16) {
    unsafe {
        *DRAW_COLORS = v;
    }
}
