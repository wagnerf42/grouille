//! Polygon module.
use Point;

/// Oriented polygons.
#[derive(Debug)]
pub struct Polygon {
    /// Vector of all points forming the edge of the polygon.
    pub points: Vec<Point>,
}

impl Polygon {
    /// Create polygon out of given points vector.
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { points }
    }
}
