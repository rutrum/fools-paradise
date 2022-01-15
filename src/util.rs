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

// Returns the bits within a certain range in a u8.
// For example, the bit range of 18 = 0b00010010 from 1 to 5
// is 0010, which is returned as 0b00000010.
pub fn bit_range(value: u8, start: usize, end: usize) -> u8 {
    let right_mask = 0b11111111 >> start;
    let left_mask = 0b11111111 << 8 - end;
    let mask = right_mask & left_mask;
    let masked_val = value & mask;

    // shift the masked val
    masked_val >> 8 - end
}
