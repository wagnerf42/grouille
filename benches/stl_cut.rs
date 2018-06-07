#[macro_use]
extern crate criterion;
extern crate grouille;

use criterion::Criterion;
use grouille::Stl;

fn cut_cordoba(c: &mut Criterion) {
    let mut stl = Stl::new("test_files/cordoba.stl").expect("failed loading cordoba stl file");
    c.bench_function("cut cordoba", move |b| b.iter(|| stl.cut(0.1)));
}

criterion_group!(benches, cut_cordoba);
criterion_main!(benches);
