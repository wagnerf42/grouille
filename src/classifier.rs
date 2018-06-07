//! This module allows for the initial classification
//! of polygons into a polygon tree.
use std::iter::repeat;
use Polygon;

/// Take some polygons to classify and return two vectors : classified ones and roots
pub fn brute_force_classification<'a>(
    polygons: &'a [Polygon],
) -> (Vec<(usize, usize)>, Vec<usize>) {
    // our results
    let mut roots: Vec<usize> = Vec::with_capacity(polygons.len()); // indices of all root polygons
    let mut classified: Vec<(usize, usize)> = Vec::with_capacity(polygons.len()); // (id, father_id)

    let mut fathers: Vec<usize> = (0..polygons.len()).collect(); // to each polygon its father (or self if none)
    let mut polygons_indices = fathers.clone();

    while !polygons_indices.is_empty() {
        // take a y with at least one polygon
        let y = polygons[*polygons_indices.first().unwrap()]
            .points()
            .first()
            .unwrap()
            .y;
        // use this y to figure out all polygons relative positions here
        let mut intersections = Vec::with_capacity(polygons.len() * 20);
        let mut remaining_indices = Vec::with_capacity(polygons.len());
        for index in polygons_indices {
            let mut keep = true;
            for x in polygons[index].intersections_at_y(y) {
                intersections.push((x, index));
                keep = false;
            }
            if keep {
                remaining_indices.push(index);
            }
        }
        intersections.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        // now we loop on all intersections, figuring out who is included in whom
        let mut i = intersections.into_iter();
        let (_, mut previous_one) = i.next().unwrap();

        let mut counts = repeat(0).take(polygons.len()).collect::<Vec<usize>>();
        counts[previous_one] = 1;

        for (_, index) in i {
            if counts[index] == 0 {
                // first time we see it, classify it
                if counts[previous_one] % 2 == 1 {
                    // we are included in neighbour : we are his son
                    fathers[index] = previous_one;
                    classified.push((index, fathers[index]));
                } else {
                    // we are not included in neighbour : we are his brother
                    if fathers[previous_one] != previous_one {
                        fathers[index] = fathers[previous_one];
                        classified.push((index, fathers[index]));
                    } else {
                        roots.push(index);
                    }
                }
            }
            counts[index] += 1;
            previous_one = index;
        }

        polygons_indices = remaining_indices;
    }
    (classified, roots)
}
