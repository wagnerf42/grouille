//! Handles binary STL files loading and slicing.
//! Provides **Stl** class handling 3d models from stl files.
//! Color information is discarded.
mod facet;
use byteorder::{LittleEndian, ReadBytesExt};
use itertools::repeat_call;
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
}
