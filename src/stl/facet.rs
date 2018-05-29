//! Provides `Facet` class for handling 3D facets from stl files.
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Error, Read, Seek, SeekFrom};

use itertools::Itertools;
use itertools::MinMaxResult;

use std::hash::{Hash, Hasher};
use utils::min_max;
use {CoordinatesHash, Point, Point3, PointsHash, Segment};

/// A `Facet` is just a triangle in space.
#[derive(Debug)]
pub struct Facet {
    points: [Point3<f64>; 3],
}

/// we hash references to facets (NOT FACETS).
impl<'a> Hash for &'a Facet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((*self) as *const _ as usize).hash(state)
    }
}

/// we compare references to facets (NOT FACETS).
impl<'a> PartialEq for &'a Facet {
    fn eq(&self, other: &Self) -> bool {
        (*self) as *const _ == (*other) as *const _
    }
}

impl<'a> Eq for &'a Facet {}

impl Facet {
    /// Parses binary content into of cursor on stl data into facet.
    pub fn new<R: Read + Seek>(
        raw_data: &mut R,
        heights_hasher: &mut CoordinatesHash,
    ) -> Result<Facet, Error> {
        fn read_point<R: Read>(
            raw_data: &mut R,
            heights_hasher: &mut CoordinatesHash,
        ) -> Result<Point3<f64>, Error> {
            let x = f64::from(raw_data.read_f32::<LittleEndian>()?);
            let y = f64::from(raw_data.read_f32::<LittleEndian>()?);
            let z = f64::from(raw_data.read_f32::<LittleEndian>()?);
            let point = Point3::new(x, y, heights_hasher.add(z));
            Ok(point)
        }
        //skip normal vector
        //no pb unwrapping since we already tested for size outside
        raw_data.seek(SeekFrom::Current(12)).unwrap();
        let new_facet = Facet {
            points: [
                read_point(raw_data, heights_hasher)?,
                read_point(raw_data, heights_hasher)?,
                read_point(raw_data, heights_hasher)?,
            ],
        };
        //skip useless bytes
        raw_data.seek(SeekFrom::Current(2))?;
        Ok(new_facet)
    }
    /// Return segment (at most one, do not call on horizontal facets) intersecting
    /// facet at given height (with rounded points).
    pub fn intersect(&self, height: f64, points_hasher: &mut PointsHash) -> Option<Segment> {
        self.points
            .iter()
            .tuple_combinations() // all facet's segments
            .filter_map(|(p1, p2)| {
                segment_intersection(p1, p2, height).map(|p| points_hasher.add(p)) // cut them at height
            })
            .combinations(2) // all horizontal segments between intersections
            .filter(|i| i[0] != i[1])
            .next() // in fact, there can be no more than 1, so just take it
            .map(|i| Segment::new(i[0], i[1]))
    }

    /// Return our min and max z.
    pub fn heights_limits(&self) -> (f64, f64) {
        match self.points.iter().map(|p| p.z).minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(e) => (e, e),
            _ => panic!("no way"),
        }
    }
}

/// Intersect given 3d segment at given height
fn segment_intersection(start: &Point3<f64>, end: &Point3<f64>, height: f64) -> Option<Point> {
    let [lower_z, higher_z] = min_max(start.z, end.z);
    if height < lower_z || height > higher_z {
        None
    } else {
        let alpha = (height - start.z) / (end.z - start.z);
        let intersecting_x = start.x + alpha * (end.x - start.x);
        let intersecting_y = start.y + alpha * (end.y - start.y);
        Some(Point::new(intersecting_x, intersecting_y))
    }
}
