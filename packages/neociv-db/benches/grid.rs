use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use neociv_db::{exec_stmt, NeocivDB};

fn set_grid(xsize: u8, ysize: u8) -> () {
    exec_stmt!(NeocivDB::default(), "set_grid", xsize, ysize).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("grid 0x0", |b| {
        b.iter(|| set_grid(black_box(0), black_box(0)))
    });

    c.bench_function("grid 1x1", |b| {
        b.iter(|| set_grid(black_box(1), black_box(1)))
    });

    c.bench_function("grid 2x2", |b| {
        b.iter(|| set_grid(black_box(2), black_box(2)))
    });

    c.bench_function("grid 4x4", |b| {
        b.iter(|| set_grid(black_box(4), black_box(4)))
    });

    c.bench_function("grid 8x8", |b| {
        b.iter(|| set_grid(black_box(8), black_box(8)))
    });

    c.bench_function("grid 16x16", |b| {
        b.iter(|| set_grid(black_box(16), black_box(16)))
    });

    c.bench_function("grid 32x32", |b| {
        b.iter(|| set_grid(black_box(32), black_box(32)))
    });

    c.bench_function("grid 64x64", |b| {
        b.iter(|| set_grid(black_box(64), black_box(64)))
    });

    c.bench_function("grid 128x128", |b| {
        b.iter(|| set_grid(black_box(128), black_box(128)))
    });

    c.bench_function("grid 255x255", |b| {
        b.iter(|| set_grid(black_box(255), black_box(255)))
    });
}

criterion_group!(
    name = grid;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3));
    targets = criterion_benchmark
);
criterion_main!(grid);
