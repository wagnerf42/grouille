//! We provide here tools for hashing (adjusting) points and coordinates.
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use super::Point;

/// Hash nearby coordinates together in O(1).
pub struct CoordinatesHash {
    precision: f64,
    hashes: [HashMap<i32, f64>; 2],
}

/// Return coordinate's key for first hash.
fn key(precision: f64, c: f64) -> i32 {
    (c / precision).floor() as i32
}

/// Return coordinate's key for second hash.
fn displaced_key(precision: f64, c: f64) -> i32 {
    ((c / precision) + 0.5).floor() as i32
}

impl CoordinatesHash {
    /// Create a new `CoordinatesHash` with given precision.
    pub fn new(precision: f64) -> Self {
        CoordinatesHash {
            precision,
            hashes: [HashMap::new(), HashMap::new()],
        }
    }

    /// Add a new coordinate c to the hash.
    /// If there exists any coordinate c2 such that |c-c2| < precision
    /// then c is hashed as c2 else c is hashed as itself.
    pub fn add(&mut self, c: f64) -> f64 {
        let p = self.precision;
        let first_key = key(p, c);
        if let Some(c2) = self.hashes[0].get(&first_key) {
            return *c2;
        }
        match self.hashes[1].entry(displaced_key(p, c)) {
            Occupied(e) => return *e.get(),
            Vacant(e) => {
                e.insert(c);
            }
        }
        self.hashes[0].insert(first_key, c);
        c
    }
}

/// Align 2d points horizontally and vertically in O(1).
/// This allows us to avoid many rounding errors and to simplify paths.
pub struct PointsHash {
    hashes: [CoordinatesHash; 2],
}

impl PointsHash {
    /// Return a new `PointHash` with given precision.
    pub fn new(precision: f64) -> Self {
        PointsHash {
            hashes: [
                CoordinatesHash::new(precision),
                CoordinatesHash::new(precision),
            ],
        }
    }
    /// Add given point to the hash. Modify point coordinates such that
    /// coordinates near from existing ones are shifted to existing values.
    /// Example:
    /// ```
    /// use grouille::{Point, PointsHash};
    /// let mut hasher = PointsHash::new(0.4);
    /// let p1 = hasher.add(Point::new(1.0, 3.5));
    /// let p2 = hasher.add(Point::new(1.3, 4.2));
    /// assert_eq!(p1, Point::new(1.0, 3.5));
    /// assert_eq!(p2, Point::new(1.0, 4.2)); // 4.2 is too far from 3.5 and is not shifted
    /// ```
    pub fn add(&mut self, mut point: Point) -> Point {
        point.x = self.hashes[0].add(point.x);
        point.y = self.hashes[1].add(point.y);
        point
    }
}
