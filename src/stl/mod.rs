//! Handles binary STL files loading and slicing.
//! Provides **Stl** class handling 3d models from stl files.
//! Color information is discarded.
mod facet;
use byteorder::{LittleEndian, ReadBytesExt};
use itertools::repeat_call;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f64::{INFINITY, NEG_INFINITY};
use std::fs::File;
use std::io::{BufReader, Error, Seek, SeekFrom};
use std::path::Path;
use stl::facet::Facet;
use {CoordinatesHash, HashKey, PointsHash, Segment};

/// Loaded STL file as a set of facets.
pub struct Stl {
    /// Vector containing all facets
    pub facets: Vec<Facet>,
    /// Heights hasher used to adjust heights of each point
    pub heights_hasher: CoordinatesHash,
}

impl Stl {
    /// Loads a new stl model from given file.
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Stl, Error> {
        let mut file = File::open(filename)?;
        //read header
        file.seek(SeekFrom::Start(80))?;
        let facets_number = file.read_u32::<LittleEndian>()?;

        let mut facets_data = BufReader::new(file);
        let mut heights_hasher = CoordinatesHash::new(0.0001);

        let facets = repeat_call(|| Facet::new(&mut facets_data, &mut heights_hasher))
            .take(facets_number as usize)
            .collect::<Result<Vec<Facet>, _>>()?;

        Ok(Stl {
            facets,
            heights_hasher,
        })
    }

    /// Prepare for cutting by generating all events.
    fn generate_cutting_events<'a>(&'a mut self, thickness: f64) -> Vec<CuttingEvent<'a>> {
        let (facets, hasher) = (&self.facets, &mut self.heights_hasher);
        let ((z_min, z_max), mut events) = facets
            .iter()
            .filter(|f| !f.is_horizontal())
            .map(|f| (f.heights_limits(), f))
            .fold(
                (
                    (INFINITY, NEG_INFINITY),
                    Vec::with_capacity(3 * facets.len()),
                ),
                |((mut old_min, mut old_max), mut events), ((min, max), facet)| {
                    if min < old_min {
                        old_min = min;
                    }
                    if max > old_max {
                        old_max = max;
                    }
                    events.push(CuttingEvent::FacetStart(min, facet));
                    events.push(CuttingEvent::FacetEnd(max, facet));
                    ((old_min, old_max), events)
                },
            );
        events.extend(
            (1..)
                .scan(z_min, |z, _| {
                    *z += thickness;
                    Some(*z)
                })
                .map(|z| hasher.add(z))
                .take_while(|&z| z < z_max)
                .map(|z| CuttingEvent::Cut(z)),
        );
        events.sort();
        events
    }

    /// cut stl regularly with slices of given thickness.
    pub fn cut(&mut self, thickness: f64, points_hasher: &mut PointsHash) -> Vec<Vec<Segment>> {
        let events = self.generate_cutting_events(thickness);
        let mut alive_facets: HashSet<&Facet> = HashSet::with_capacity(events.len());
        let mut slices = Vec::new();
        for event in &events {
            match *event {
                CuttingEvent::FacetEnd(_, ref f) => {
                    alive_facets.remove(f);
                }
                CuttingEvent::FacetStart(_, f) => {
                    alive_facets.insert(f);
                }
                CuttingEvent::Cut(h) => slices.push(
                    alive_facets
                        .iter()
                        .filter_map(|f| f.intersect(h, points_hasher))
                        .collect(),
                ),
            }
        }
        slices
    }

    /// cut stl regularly with slices of given thickness. (second algorithm)
    pub fn cut2(&mut self, thickness: f64) -> Vec<Vec<Segment>> {
        let mut points_hasher = PointsHash::new(0.001);
        let mut slices = HashMap::new();
        let heights_between = |(zmin, zmax)| {
            (1..)
                .scan(
                    (zmin - zmin % thickness) / thickness,
                    |h, _| -> Option<f64> {
                        let old_h = *h; // we start below
                        *h += thickness;
                        Some(old_h)
                    },
                )
                .take_while(move |h| *h <= zmax + thickness)
        }; // and end above to avoid rounding problems

        for facet in self.facets.iter().filter(|f| !f.is_horizontal()) {
            for height in heights_between(facet.heights_limits()) {
                let height = self.heights_hasher.key(height);
                if let Some(segment) = facet.intersect(height.0, &mut points_hasher) {
                    slices.entry(height).or_insert_with(Vec::new).push(segment);
                }
            }
        }
        let mut slices: Vec<(HashKey, Vec<Segment>)> = slices.drain().collect();
        slices.sort_unstable_by_key(|e| e.0);
        let final_slices: Vec<Vec<Segment>> = slices.drain(..).map(|(_, v)| v).collect();
        final_slices
    }

    /// Cut just one slice at given height.
    /// This is mainly used for debugging or test purposes.
    /// All points are thus hashed with a temporary hasher.
    pub fn cut_at(&mut self, height: f64) -> Vec<Segment> {
        let height = self.heights_hasher.key(height);
        let mut points_hasher = PointsHash::new(0.001);
        self.facets
            .iter()
            .filter(|f| !f.is_horizontal())
            .filter_map(|f| f.intersect(height.0, &mut points_hasher))
            .collect()
    }
}

enum CuttingEvent<'a> {
    /// Given facet appears at this height.
    FacetEnd(f64, &'a Facet),
    /// Given facet disappears at this height.
    FacetStart(f64, &'a Facet),
    /// We cut all alive facets at this height.
    Cut(f64),
}

impl<'a> CuttingEvent<'a> {
    fn height(&self) -> f64 {
        match *self {
            CuttingEvent::FacetEnd(h, _) => h,
            CuttingEvent::FacetStart(h, _) => h,
            CuttingEvent::Cut(h) => h,
        }
    }
    fn type_order(&self) -> u8 {
        match *self {
            CuttingEvent::FacetEnd(_, _) => 0,
            CuttingEvent::FacetStart(_, _) => 1,
            CuttingEvent::Cut(_) => 2,
        }
    }
}

impl<'a> PartialEq for CuttingEvent<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.height() == other.height() || self.type_order() == other.type_order()
    }
}

impl<'a> Eq for CuttingEvent<'a> {}

impl<'a> Ord for CuttingEvent<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> PartialOrd for CuttingEvent<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.height()
            .partial_cmp(&other.height())
            .map(|o| o.then(self.type_order().cmp(&other.type_order())))
    }
}
