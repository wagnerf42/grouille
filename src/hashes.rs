//! We provide here tools for hashing (adjusting) points and coordinates.
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

/// Hash nearby coordinates together in O(1).
struct CoordinatesHash {
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
    fn new(precision: f64) -> Self {
        CoordinatesHash {
            precision,
            hashes: [HashMap::new(), HashMap::new()],
        }
    }

    /// Add a new coordinate c to the hash.
    /// If there exists any coordinate c2 such that |c-c2| < precision
    /// then c is hashed as c2 else c is hashed as itself.
    fn add(&mut self, c: f64) -> f64 {
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
