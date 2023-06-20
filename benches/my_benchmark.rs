use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use fwht::{
    fwht,
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

fn naive_wht_02() {
    let data = &mut vec![1; 1024];
    let mut transform = Naive::new();
    transform.init(data);

    transform.process(data);
}

fn fast_wht_02() {
    let data = &mut vec![1; 1024];

    fwht(data);
}

fn naive_wht_03() {
    let data = &mut vec![1; 4096];
    let mut transform = Naive::new();
    transform.init(data);

    transform.process(data);
}

fn fast_wht_03() {
    let data = &mut vec![1; 4096];

    fwht(data);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("naive1", |b| b.iter(|| naive_wht_01()));
    c.bench_function("fast1", |b| b.iter(|| fast_wht_01()));
    c.bench_function("inlined8", |b| b.iter(|| inlined_wht8_01()));
    c.bench_function("naive2", |b| b.iter(|| naive_wht_02()));
    c.bench_function("fast2", |b| b.iter(|| fast_wht_02()));
    c.bench_function("naive3", |b| b.iter(|| naive_wht_03()));
    c.bench_function("fast3", |b| b.iter(|| fast_wht_03()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
