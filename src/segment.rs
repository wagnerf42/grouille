//! 2d segments

use Point;

/// 2d oriented segment
pub struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    /// Create a new 2d segment
    pub fn new(start: Point, end: Point) -> Self {
        Segment { start, end }
    }
}
