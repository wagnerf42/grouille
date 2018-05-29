#![deny(missing_docs)]
//! grouille is a geometry library for rust.
//! It allows fast paths computations for different CNC machines.
extern crate byteorder;
extern crate itertools;
extern crate nalgebra;
extern crate num_traits;

mod quadrant;
pub use quadrant::Quadrant;
mod hashes;
pub use hashes::{CoordinatesHash, HPoint, HashKey, PointsHash};
mod stl;
pub use stl::Stl;
pub mod segment;
pub mod tycat;
pub use segment::Segment;
pub mod overlap;
mod utils;

use nalgebra::geometry::{Point2, Point3};
/// 2D point.
pub type Point = Point2<f64>;
