use crate::wasm4::sys::*;

pub fn enemy_fire() {
    tone(230 | (100 << 16), 10, 23, TONE_PULSE1);
}

pub fn player_fire() {
    tone(440 | (320 << 16), 6, 51, TONE_PULSE1);
}
