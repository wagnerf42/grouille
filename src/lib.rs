#![deny(missing_docs)]
//! grouille is a geometry library for rust.
//! It allows fast paths computations for different CNC machines.
extern crate nalgebra;

mod quadrant;
pub use quadrant::Quadrant;
mod hashes;
pub use hashes::{CoordinatesHash, PointsHash};
pub mod tycat;
mod utils;

use nalgebra::geometry::Point2;
/// 2D point.
pub type Point = Point2<f64>;
