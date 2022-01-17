use crate::wasm4::sys::*;

pub fn enemy_fire() {
    tone(230 | (100 << 16), 10, 20, TONE_PULSE1);
}

pub fn player_fire() {
    tone(440 | (320 << 16), 6, 51, TONE_PULSE1);
}

pub fn enemy_death() {
    tone(260 | (120 << 16), 56 << 8, 30, TONE_NOISE);
}

pub fn player_death() {
    tone(370 | (250 << 16), 22 | 90 << 8, 30, TONE_NOISE);
}
