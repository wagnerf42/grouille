//! We provide here tools for hashing (adjusting) points and coordinates.
use num_traits::float::Float;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;
use Point;

/// Hashable floating points.
/// This is possible because these keys can only be obtained through
/// a `CoordinatesHash`.
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct HashKey(pub f64);

impl Eq for HashKey {}

// all code hashing floating points is taken from the 'ordered_float' crate
impl Hash for HashKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        raw_double_bits(&self.0).hash(state);
    }
}

impl Ord for HashKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        raw_double_bits(&self.x).hash(state);
        raw_double_bits(&self.y).hash(state);
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// canonical raw bit patterns (for hashing)
const CANONICAL_NAN_BITS: u64 = 0x7_ff8_000_000_000_000u64;
const CANONICAL_ZERO_BITS: u64 = 0x0u64;

// masks for the parts of the IEEE 754 float
const SIGN_MASK: u64 = 0x8_000_000_000_000_000u64;
const EXP_MASK: u64 = 0x7_ff0_000_000_000_000u64;
const MAN_MASK: u64 = 0x0_00f_fff_fff_fff_fffu64;

#[inline]
fn raw_double_bits(f: &f64) -> u64 {
    if f.is_nan() {
        return CANONICAL_NAN_BITS;
    }

    let (man, exp, sign) = f.integer_decode();
    if man == 0 {
        return CANONICAL_ZERO_BITS;
    }

    let exp_u64 = u64::from(unsafe { mem::transmute::<i16, u16>(exp) });
    let sign_u64 = if sign > 0 { 1u64 } else { 0u64 };
    (man & MAN_MASK) | ((exp_u64 << 52) & EXP_MASK) | ((sign_u64 << 63) & SIGN_MASK)
}

/// Hash nearby coordinates together in O(1).
pub struct CoordinatesHash {
    precision: f64,
    hash: HashMap<i32, f64>,
}

/// Return coordinate's key for first hash.
fn key(precision: f64, c: f64) -> i32 {
    (c / precision).floor() as i32
}

impl CoordinatesHash {
    /// Create a new `CoordinatesHash` with given precision.
    pub fn new(precision: f64) -> Self {
        let mut hash = CoordinatesHash {
            precision,
            hash: HashMap::new(),
        };
        hash.add(0.0);
        hash
    }

    /// Add a new coordinate c to the hash.
    /// If there exists any coordinate c2 such that |c-c2| < precision
    /// then c is hashed as c2 else c is hashed as itself.
    pub fn add(&mut self, c: f64) -> f64 {
        let p = self.precision;
        let key = key(p, c);
        if let Some(c2) = ((key - 1)..=(key + 1))
            .filter_map(|k| self.hash.get(&k))
            .next()
        {
            *c2
        } else {
            self.hash.insert(key, c);
            c
        }
    }

    /// Add given coordinate and return corresponding hashable key.
    pub fn key(&mut self, c: f64) -> HashKey {
        HashKey(self.add(c))
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
    /// # Example:
    /// ```
    /// use grouille::{Point, PointsHash};
    /// let mut hasher = PointsHash::new(0.4);
    /// let p1 = hasher.add(Point::new(1.0, 3.5));
    /// let p2 = hasher.add(Point::new(1.3, 4.2));
    /// assert_eq!(p1, Point::new(1.0, 3.5));
    /// assert_eq!(p2, Point::new(1.0, 4.2)); // 4.2 is too far from 3.5 and is not shifted
    /// ```
    pub fn add(&mut self, point: Point) -> Point {
        Point::new(self.hashes[0].add(point.x), self.hashes[1].add(point.y))
    }
}
