#![feature(portable_simd)]
use std::simd::Simd;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use fwht::{
    fwht,
    fwht4,
    fwht8,
    wht8,
    Naive,
};

fn naive_wht_01() {
    let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
    let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];
    let mut transform = Naive::new();
    transform.init(data);

    transform.process(data);
    assert_eq!(*data, expected);
}

fn fast_wht_01() {
    let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
    let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];

    fwht(data);
    assert_eq!(*data, expected);
}

fn inlined_wht8_01() {
    let data = &mut [1, 0, 1, 0, 0, 1, 1, 0];
    let expected = [4, 2, 0, -2, 0, 2, 0, 2];

    wht8(data);
    assert_eq!(*data, expected);
}

// fn naive_wht_02() {
//     let data = &mut vec![1; 2048];
//     let mut transform = Naive::new();
//     transform.init(data);

//     transform.process(data);
// }

fn fast_wht_02() {
    let data = &mut vec![1; 1024];

    fwht(data);
}

fn simd_fwht4_02() {
    let data = &mut vec![1; 1024];

    let frame = Simd::from([0; 4]);
    let mut scratch = vec![frame; 256];
    fwht4(data, &mut scratch);
}

fn simd_fwht8_02() {
    let data = &mut vec![1; 1024];

    let frame = Simd::from([0; 8]);
    let mut scratch = vec![frame; 128];
    fwht8(data, &mut scratch);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("naive1", |b| b.iter(|| naive_wht_01()));
    c.bench_function("fast1", |b| b.iter(|| fast_wht_01()));
    c.bench_function("inlined8", |b| b.iter(|| inlined_wht8_01()));
    // c.bench_function("naive2", |b| b.iter(|| naive_wht_02()));
    c.bench_function("fast2", |b| b.iter(|| fast_wht_02()));
    c.bench_function("simd_fwht4", |b| b.iter(|| simd_fwht4_02()));
    c.bench_function("simd_fwht8", |b| b.iter(|| simd_fwht8_02()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
