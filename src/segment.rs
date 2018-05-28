//! 2d segments

use nalgebra::{angle, Vector2};
use Point;

/// 2d oriented segment
#[derive(Debug)]
pub struct Segment {
    /// starting point
    pub start: Point,
    /// ending point
    pub end: Point,
}

impl Segment {
    /// Create a new 2d segment
    pub fn new(start: Point, end: Point) -> Self {
        Segment { start, end }
    }

    /// Returns supporting angle
    pub fn sweeping_angle(&self) -> f64 {
        angle(&(self.end - &self.start), &Vector2::x_axis())
    }
}
