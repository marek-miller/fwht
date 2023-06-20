#![feature(portable_simd)]
use std::simd::Simd;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use fwht::{
    fwht,
    fwht4_inl,
    fwht4_simd,
    fwht8_inl,
    fwht8_simd,
    wht8,
    Naive, fwht_par,
};

// fn naive_wht_01() {
//     let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
//     let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];
//     let mut transform = Naive::new();
//     transform.init(data);

//     transform.process(data);
//     assert_eq!(*data, expected);
// }

// fn fast_wht_01() {
//     let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
//     let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];

//     fwht(data, 1);
//     assert_eq!(*data, expected);
// }

// fn inlined_wht8_01() {
//     let data = &mut [1, 0, 1, 0, 0, 1, 1, 0];
//     let expected = [4, 2, 0, -2, 0, 2, 0, 2];

//     wht8(data);
//     assert_eq!(*data, expected);
// }

// fn naive_wht_02() {
//     let data = &mut vec![1; 2048];
//     let mut transform = Naive::new();
//     transform.init(data);

//     transform.process(data);
// }

const DATALEN: usize = 1024*1024;

fn fwht_02() {
    let data = &mut vec![1; DATALEN];

    fwht(data, 1);
}


fn fwht_par_02() {
    let data = &mut vec![1; DATALEN];

    fwht_par(data, 1);
}

fn fwht4_simd_02(scratch: &mut [Simd<i64, 4>]) {
    let data = &mut vec![1; DATALEN];

    fwht4_simd(data, scratch);
}

fn fwht8_simd_02(scratch: &mut [Simd<i64, 8>]) {
    let data = &mut vec![1; DATALEN];

    fwht8_simd(data, scratch);
}

fn fwht4_inl_02() {
    let data = &mut vec![1; DATALEN];

    fwht4_inl(data);
}

fn fwht8_inl_02() {
    let data = &mut vec![1; DATALEN];

    fwht8_inl(data);
}

fn bench(c: &mut Criterion) {
    // let frame = Simd::from([0; 4]);
    // let mut scratch_4 = vec![frame; DATALEN / 4];

    // let frame = Simd::from([0; 8]);
    // let mut scratch_8 = vec![frame; DATALEN / 8];
    
    
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_function("2. fast", |b| b.iter(|| fwht_02()));
    group.bench_function("2. fast_par", |b| b.iter(|| fwht_par_02()));
    group.bench_function("2. fwht4_inl", |b| b.iter(|| fwht4_inl_02()));
    group.bench_function("2. fwht8_inl", |b| b.iter(|| fwht8_inl_02()));
    // group.bench_function("2. fwht4_simd", |b| {
    //     b.iter(|| fwht4_simd_02(&mut scratch_4))
    // });
    // group.bench_function("2. fwht8_simd", |b| {
    //     b.iter(|| fwht8_simd_02(&mut scratch_8))
    // });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

