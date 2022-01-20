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
            
            // Grey Mist: https://lospec.com/palette-list/grey-mist
            Grey => [ 0xf1ffe0, 0x988171, 0x463534, 0x1e1721 ],

            Night => [ 0xffe6ea, 0xe6a1cf, 0x4d4d80, 0x131626 ],
        }
    }

    pub fn set(&self) {
        let p = self.bytes();
        unsafe {
            *PALETTE = p;
        }
    }

    pub fn transition_to(&self, idx: usize) {
        let p = self.bytes();
        unsafe {
            PALETTE.as_mut().unwrap()[idx] = p[idx];
        }
    }
}

pub fn set_draw(v: u16) {
    unsafe {
        *DRAW_COLORS = v;
    }
}
