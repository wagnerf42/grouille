//! 2d segments

use std::f64::consts::PI;
use utils::{is_almost, min_max};
use {CoordinatesHash, HashKey, Point};

/// 2d oriented segment
#[derive(Debug, PartialEq, Eq, Hash)]
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

    /// Return segment in opposite direction
    pub fn reverse(&self) -> Self {
        Segment::new(self.end, self.start)
    }

    /// Returns supporting angle
    pub fn sweeping_angle(&self) -> f64 {
        let angle = (self.end - self.start).angle();
        if angle == PI {
            0.0
        } else if angle < 0.0 {
            angle + PI
        } else {
            angle
        }
    }

    /// Return our points ordered by lexicographically increasing coordinates.
    pub fn ordered_points(&self) -> [Point; 2] {
        min_max(&self.start, &self.end)
    }

    /// Intersects two segments.
    pub fn intersection_with(&self, other: &Segment) -> Option<Point> {
        // we solve system obtained by considering the point is inside both segments.
        // p = self.start + alpha * self.direction_vector()
        // p = other.start + beta * self.direction_vector()
        let d = self.end - self.start;
        let d2 = other.end - other.start;
        let denominator = d2.x * d.y - d.x * d2.y;
        if is_almost(denominator, 0.0) {
            None // almost parallel lines
        } else {
            let alpha = (d2.x * (other.start.y - self.start.y)
                + d2.y * (self.start.x - other.start.x)) / denominator;
            let beta = (d.x * (other.start.y - self.start.y) + d.y * (self.start.x - other.start.x))
                / denominator;
            if (is_almost(0.0, alpha) || is_almost(1.0, alpha) || (0.0 < alpha && alpha < 1.0))
                && (is_almost(0.0, beta) || is_almost(1.0, beta) || (0.0 < beta && beta < 1.0))
            {
                Some(self.start + d * alpha)
            } else {
                None
            }
        }
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
