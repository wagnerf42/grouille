//! Quadrants delimit rectangular regions of 2d space.
//! They have many uses like being simple shapes containing other more complex ones.
use utils::{max, min};
use Point;
use std::f64::{INFINITY, NEG_INFINITY};

#[derive(Debug)]
/// A `Quadrant` delimits a rectangular region in the plane.
pub struct Quadrant {
    mins: [f64; 2],
    maxs: [f64; 2],
}

impl Quadrant {
    /// Create a new `Quadrant` containing an empty region.
    pub fn new() -> Self {
        Quadrant {
            mins: [INFINITY; 2],
            maxs: [NEG_INFINITY; 2],
        }
    }
    /// Extend quadrant (return a new one) by computing the smallest one containing
    /// the old one and the added point.
    pub fn add(self, point: &Point) -> Self {
        Quadrant {
            mins: [min(self.mins[0], point.x), min(self.mins[1], point.y)],
            maxs: [max(self.maxs[0], point.x), max(self.maxs[1], point.y)],
        }
    }

    /// Extend Self by merging other quadrant.
    pub fn update(&mut self, other: &Self) {
        self.mins[0] = min(self.mins[0], other.mins[0]);
        self.mins[1] = min(self.mins[1], other.mins[1]);
        self.maxs[0] = max(self.maxs[0], other.maxs[0]);
        self.maxs[1] = max(self.maxs[1], other.maxs[1]);
    }

    /// Return limits for given dimension.
    pub fn limits(&self, dimension: usize) -> (f64, f64) {
        (self.mins[dimension], self.maxs[dimension])
    }
}
