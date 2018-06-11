#[macro_use]
extern crate criterion;
extern crate grouille;

use criterion::Criterion;
use grouille::{classifier, overlap::remove_overlaps, polygon::polygon_builder::build_polygons, Stl};

fn classify_cordoba(c: &mut Criterion) {
    let mut stl = Stl::new("test_files/cordoba-very-large.stl")
        .expect("failed finding cordoba example stl file");
    let slice = stl.cut_at(1.2);
    let remaining_segments = remove_overlaps(&slice);
    let polygons = build_polygons(&remaining_segments);

    c.bench_function("classify cordoba", move |b| {
        b.iter(|| classifier::brute_force_classification(&polygons))
    });
}

criterion_group!(benches, classify_cordoba);
criterion_main!(benches);
