// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::env::args;
use std::process::{Command, exit};

use criterion::{Criterion, criterion_group};
use nalgebra::{Perspective3, Point3};
use rand::random_range;

fn main() {
    if args().len() < 2 {
        let status = Command::new(args().next().expect("No program path in argv"))
            .arg("--bench")
            .status()
            .expect("Failed to execute self");
        exit(status.code().expect("No exit code"));
    }

    benches();
    Criterion::default().configure_from_args().final_summary();
}

criterion_group!(benches, bench_mat4_transform_point3);

fn bench_mat4_transform_point3(criterion: &mut Criterion) {
    let mat = Perspective3::<f32>::new(
        random_range(0.5..1.5),
        random_range(45.0..90.0),
        random_range(1.0..10.0),
        random_range(100.0..1000.0),
    )
    .to_homogeneous();
    let pt = Point3::new(
        random_range(-1000.0..1000.0),
        random_range(-1000.0..1000.0),
        random_range(-1000.0..1000.0),
    );

    criterion.bench_function("bench_mat4_transform_point3", |b| {
        b.iter(|| mat.transform_point(&pt));
    });
}
