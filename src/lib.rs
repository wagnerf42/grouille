#![feature(slice_patterns)]
#![deny(missing_docs)]
//! grouille is a geometry library for rust.
//! It allows fast paths computations for different CNC machines.
extern crate byteorder;
extern crate itertools;
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
mod point;
pub use point::{Point, Point3, Vector};
