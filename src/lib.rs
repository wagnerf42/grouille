#![deny(missing_docs)]
//! grouille is a geometry library for rust.
//! It allows fast paths computations for different CNC machines.
extern crate byteorder;
extern crate itertools;
extern crate nalgebra;

mod quadrant;
pub use quadrant::Quadrant;
mod hashes;
pub use hashes::{CoordinatesHash, PointsHash};
mod stl;
pub use stl::Stl;
pub mod tycat;
pub mod segment;
pub use segment::Segment;
mod utils;

use nalgebra::geometry::{Point2, Point3};
/// 2D point.
pub type Point = Point2<f64>;
