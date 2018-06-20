//#![feature(slice_patterns)]
#![deny(missing_docs)]
//! grouille is a geometry library for rust.
//! It allows fast paths computations for different CNC machines.
extern crate byteorder;
extern crate itertools;
extern crate num_traits;
extern crate streaming_iterator;

mod quadrant;
pub use quadrant::Quadrant;
mod hashes;
pub use hashes::{CoordinatesHash, HashKey, PointsHash};
mod stl;
pub use stl::Stl;
pub mod segment;
#[macro_use]
pub mod tycat;
pub use segment::Segment;
pub mod overlap;
mod point;
pub use point::{Point, Point3, Vector};
pub mod polygon;
pub use polygon::Polygon;
pub mod classifier;
pub mod slice;
pub mod holed_polygon;
pub use holed_polygon::HoledPolygon;
mod utils;
pub mod arc;
pub use arc::{Arc};
pub mod elementary_path;
pub use elementary_path::ElementaryPath;
pub mod intersections;
pub mod pocket;
pub use pocket::Pocket;
