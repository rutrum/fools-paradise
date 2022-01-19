use crate::wasm4::sys::*;

pub enum Palette {
    Crimson,
    BlueMold,
}

impl Palette {
    fn bytes(&self) -> [u32; 4] {
        use Palette::*;
        match self {
            Crimson => [ 0xeff9d6, 0xba5044, 0x7a1c4b, 0x1b0326 ],
            BlueMold => [ 0x191b1a, 0x294257, 0x579c9a, 0x99c9b3 ],
        }
    }

    pub fn set(&self) {
        let p = self.bytes();
        unsafe {
            *PALETTE = p;
        }
    }
}
