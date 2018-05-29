//! 2d segments

use nalgebra::{angle, Vector2};
use utils::min_max;
use {CoordinatesHash, HPoint, HashKey};

/// 2d oriented segment
#[derive(Debug)]
pub struct Segment {
    /// starting point
    pub start: HPoint,
    /// ending point
    pub end: HPoint,
}

impl Segment {
    /// Create a new 2d segment
    pub fn new(start: HPoint, end: HPoint) -> Self {
        Segment { start, end }
    }

    /// Returns supporting angle
    pub fn sweeping_angle(&self) -> f64 {
        angle(&(&self.end.0 - &self.start.0), &Vector2::x_axis())
    }

    /// Return our points ordered by lexicographically increasing coordinates.
    pub fn ordered_points(&self) -> [HPoint; 2] {
        min_max(&self.start, &self.end)
    }

    /// Intersect with horizontal line at given y.
    /// Returns only x coordinate of intersection.
    /// Precondition: we are not a quasi-horizontal segment.
    pub fn horizontal_line_intersection(&self, y: f64) -> f64 {
        let alpha = (y - self.start.y) / (self.end.y - self.start.y);
        alpha.mul_add(self.end.x - self.start.x, self.start.x)
    }

    /// Return hashable identifier of the line we lie uppon.
    /// Allows for fast identification of aligned and overlapping segments.
    /// We need two coordinates hash to align nearby floating point keys.
    /// One to hash angles and one to hash coordinates.
    pub fn line_key(
        &self,
        angle_hasher: &mut CoordinatesHash,
        coordinates_hasher: &mut CoordinatesHash,
    ) -> (HashKey, HashKey) {
        let angle_key = angle_hasher.key(self.sweeping_angle());
        if angle_key == HashKey(0.0) {
            // we are horizontal, coordinate is any y
            (angle_key, coordinates_hasher.key(self.start.y))
        } else {
            (
                angle_key,
                coordinates_hasher.key(self.horizontal_line_intersection(0.0)),
            )
        }
    }
}
