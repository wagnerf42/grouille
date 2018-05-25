//! Handles binary STL files loading and slicing.
//! Provides **Stl** class handling 3d models from stl files.
//! Color information is discarded.
mod facet;
use byteorder::{LittleEndian, ReadBytesExt};
use itertools::repeat_call;
use std::f64::{INFINITY, NEG_INFINITY};
use std::fs::File;
use std::io::{BufReader, Error, Seek, SeekFrom};
use stl::facet::Facet;
use CoordinatesHash;

/// Loaded STL file as a set of facets.
pub struct Stl {
    /// Vector containing all facets
    pub facets: Vec<Facet>,
    /// Heights hasher used to adjust heights of each point
    pub heights_hasher: CoordinatesHash,
}

impl Stl {
    /// Loads a new stl model from given file.
    pub fn new(filename: &str) -> Result<Stl, Error> {
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
    fn generate_cutting_events<'a>(&'a self, thickness: f64) -> Vec<CuttingEvent<'a>> {
        let ((z_min, z_max), mut events) =
            self.facets.iter().map(|f| (f.heights_limits(), f)).fold(
                (
                    (INFINITY, NEG_INFINITY),
                    Vec::with_capacity(3 * self.facets.len()),
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
                .scan(z_min, |&mut z, _| Some(z + thickness))
                .map(|z| self.heights_hasher.add(z))
                .take_while(|&z| z < z_max)
                .map(|z| CuttingEvent::Cut(z)),
        );
        events
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
