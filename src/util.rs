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
    let left_mask = 0b11111111 << 8 - end;
    let mask = right_mask & left_mask;
    let masked_val = value & mask;

    // shift the masked val
    masked_val >> 8 - end
}

/// Stores a random number and provides some basic psuedo-random number generation.
pub struct Random {
    v: u32,
}

impl Random {

    /// Seed with the given value and randomize.
    pub fn seed(v: u32) -> Self {
        let mut r = Self { v };
        r.next();
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

    pub fn bool(&mut self) -> bool {
        self.next();
        self.v % 2 == 0
    }

    fn float(&mut self) -> f32 {
        self.next();
        self.v as f32 / u32::MAX as f32
    }

    pub fn in_range(&mut self, start: u32, end: u32) -> u32 {
        (self.float() * (end - start) as f32) as u32 + start
    }
}
