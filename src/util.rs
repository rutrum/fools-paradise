/// Some mathematical functions

/// Returns if two ranges overlap.
pub fn range_overlap(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    y2 > x1 && x2 > y1
}

/// Computes the intersection of two ranges.  If there is no overlap then None is returned.
pub fn range_intersection(x1: i32, x2: i32, y1: i32, y2: i32) -> Option<(i32, i32)> {
    if !range_overlap(x1, x2, y1, y2) {
        None
    } else {
        Some((x1.max(y1), x2.min(y2)))
    }
}

/// Returns the bits within a certain range in a u8.
/// For example, the bit range of 18 = 0b00010010 from 1 to 5
/// is 0010, which is returned as 0b00000010.
pub fn bit_range(value: u8, start: usize, end: usize) -> u8 {
    let right_mask = 0b11111111 >> start;
    let left_mask = 0b11111111 << (8 - end);
    let mask = right_mask & left_mask;
    let masked_val = value & mask;

    // shift the masked val
    masked_val >> (8 - end)
}

/// Find the value w proportion along the line from start to end
pub fn interpolate(start: f32, end: f32, w: f32) -> f32 {
    (end - start) * (3.0 - w * 2.0) * w * w + start
    //(end - start) * w + start
}

/// Generates a random gradient with a seed fixed by the x, y coordinates
pub fn random_gradient(x: i32, y: i32) -> (f32, f32) {
    let seed = x + 160 * y;
    let mut r = Random::seed(seed.abs() as u32);
    (r.angle().sin(), r.angle().cos())
}

/// Returns product of random gradient and distance
pub fn dot_grid_gradient(ix: i32, iy: i32, x: f32, y: f32) -> f32 {
    let grad = random_gradient(ix, iy);
    let dist = (x - ix as f32, y - iy as f32);
    grad.0 * dist.0 + grad.1 * dist.1
}

/// Perlin noise!
pub fn perlin(x: f32, y: f32) -> f32 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;

    let ws = (x - ix as f32, y - iy as f32);

    let n = (
        dot_grid_gradient(ix, iy, x, y),
        dot_grid_gradient(ix+1, iy, x, y),
    );
    let m = (
        dot_grid_gradient(ix, iy+1, x, y),
        dot_grid_gradient(ix+1, iy+1, x, y),
    );
    let inter_xs = (
        interpolate(n.0, n.1, ws.0),
        interpolate(m.0, m.1, ws.0),
    );
    interpolate(inter_xs.0, inter_xs.1, ws.1)
}

/// Stores a random number and provides some basic psuedo-random number generation.
#[derive(Clone)]
pub struct Random {
    v: u32,
}

impl Random {

    /// Seed with the given value and randomize.
    pub fn seed(v: u32) -> Self {
        let mut r = Self { v };
        r.next();
        r.next();
        r
    }

    /// Perform an xor shift
    fn next(&mut self) {
        let mut v = self.v;
        v ^= v << 13;
        v ^= v >> 17;
        v ^= v << 5;
        self.v = v;
    }

    fn float(&mut self) -> f32 {
        self.next();
        let x = self.v as f32 / u32::MAX as f32;
        assert!(x <= 1.0);
        x
    }

    /// Returns if a random float from 0 to 1 is less than or equal to the provided value.
    /// If `f` is 1 then `uniform_lt` always returns true.
    pub fn uniform_lt(&mut self, f: f32) -> bool {
        self.float() <= f
    }

    pub fn in_range(&mut self, start: u32, end: u32) -> u32 {
        (self.float() * (end - start) as f32) as u32 + start
    }

    pub fn angle(&mut self) -> f32 {
        self.float() * 2.0 * std::f32::consts::PI
    }
}
