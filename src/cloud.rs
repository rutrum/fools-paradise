use crate::wasm4::sys::*;
use crate::util;

fn adjusted_perlin(x: i32, y: i32, scale: f32) -> f32 {
    let x_val = x as f32 / scale;
    let y_val = y as f32 / scale;
    util::perlin(x_val, y_val)
}

pub fn draw(frame: u32, speed: f32) {
    unsafe {
        let buf = FRAMEBUFFER.as_mut().unwrap();

        for x in 0..160_i32 {
            for mut y in 0..160_i32 {
                let offset = (frame as f32 * speed) as i32;
                if ((y - offset) % 2 == 0) && (x + (y - offset)) % 4 == 0 {
                    let idx = (x + y * 160) / 4;
                    let shift = ((x + y * 160) % 4) * 2;

                    let y_val = y - offset;
                    let height = adjusted_perlin(x, y_val, 60.0) * 0.6
                            + adjusted_perlin(x, y_val, 20.0) * 0.3;

                    if height > 0.1 {
                        buf[idx as usize] ^= 0b01 << shift;
                    }
                }
            }
        }
    }
}
