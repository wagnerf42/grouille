//! We define a simple 2d point here together with vectors.
use std::ops::{Add, Div, Mul, Sub};
use utils::is_almost;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
/// 2d point
pub struct Point {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64,
}

impl Eq for Point {}

#[derive(Copy, Clone)]
/// a vector stores the difference between two points
pub struct Vector {
    /// x component
    pub x: f64,
    /// y component
    pub y: f64,
}

impl Point {
    /// Create a new 2d point from given coordinates.
    pub fn new(x: f64, y: f64) -> Point {
        assert!(!x.is_nan());
        assert!(!y.is_nan());
        Point { x, y }
    }

    /// Return distance between two points.
    pub fn distance_to(&self, other: &Self) -> f64 {
        (other - self).norm()
    }

    /// Are two given points almost the same ?
    pub fn is_almost(&self, other: &Self) -> bool {
        is_almost(self.x, other.x) && is_almost(self.y, other.y)
    }

    /// Return the origin.
    pub fn origin() -> Point {
        Point::new(0.0, 0.0)
    }
    /// Return center point between self and other.
    pub fn center_with(&self, other: &Point) -> Point {
        Point::new((self.x + other.x) / 2.0, (self.y + other.y) / 2.0)
    }

    /// Returns if the three given points are approximately aligned.
    pub fn is_aligned_with(&self, p2: &Point, p3: &Point) -> bool {
        let determinant = self.x * p2.y + self.y * p3.x + p2.x * p3.y
            - (p2.y * p3.x + self.y * p2.x + self.x * p3.y);
        determinant.abs() < 0.0002 // values of 0.00015 create problems when generating parallel segments
    }
}

impl Vector {
    /// Create a new 2d vector.
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    /// Create a new 2d vector from polar coordinates.
    pub fn polar(r: f64, angle: f64) -> Vector {
        Vector {
            x: r * angle.cos(),
            y: r * angle.sin(),
        }
    }

    /// Return a perpendicular vector.
    pub fn perpendicular_vector(&self) -> Vector {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }

    /// Return the vector's euclidean norm.
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Compute angle between vector and x axis (will be strictly less than PI).
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, v: Vector) -> Self::Output {
        Point::new(self.x + v.x, self.y + v.y)
    }
}

impl<'a> Add<Vector> for &'a Point {
    type Output = Point;
    fn add(self, v: Vector) -> Self::Output {
        Point::new(self.x + v.x, self.y + v.y)
    }
}

impl<'a> Add<&'a Vector> for Point {
    type Output = Point;
    fn add(self, v: &Vector) -> Self::Output {
        Point::new(self.x + v.x, self.y + v.y)
    }
}

impl<'a, 'b> Add<&'a Vector> for &'b Point {
    type Output = Point;
    fn add(self, v: &Vector) -> Self::Output {
        Point::new(self.x + v.x, self.y + v.y)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a> Sub<Point> for &'a Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a> Sub<&'a Point> for Point {
    type Output = Vector;
    fn sub(self, other: &Point) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> Sub<&'a Point> for &'b Point {
    type Output = Vector;
    fn sub(self, other: &Point) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, c: f64) -> Self::Output {
        Point::new(self.x * c, self.y * c)
    }
}

impl<'a> Mul<f64> for &'a Point {
    type Output = Point;
    fn mul(self, c: f64) -> Self::Output {
        Point::new(self.x * c, self.y * c)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, c: f64) -> Self::Output {
        Vector {
            x: self.x * c,
            y: self.y * c,
        }
    }
}

impl<'a> Mul<f64> for &'a Vector {
    type Output = Vector;
    fn mul(self, c: f64) -> Self::Output {
        Vector {
            x: self.x * c,
            y: self.y * c,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, d: f64) -> Self::Output {
        Vector {
            x: self.x / d,
            y: self.y / d,
        }
    }
}

impl<'a> Div<f64> for &'a Vector {
    type Output = Vector;
    fn div(self, d: f64) -> Self::Output {
        Vector {
            x: self.x / d,
            y: self.y / d,
        }
    }
}

/// small point in 3d with no methods
#[derive(Debug)]
pub struct Point3 {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64,
    /// z coordinate
    pub z: f64,
}

impl Point3 {
    /// Create a new 3d point with given coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { x, y, z }
    }
}
