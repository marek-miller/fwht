use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use fwht::{
    fwht,
    naive_wht,
};

fn naive_wht_01() {
    let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];

    naive_wht(data, &mut [0; 8]);
}

fn fast_wht_01() {
    let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];

    fwht(data);
}

fn naive_wht_02() {
    let data = &mut vec![1; 1024];

    naive_wht(data, &mut [0; 1024]);
}

fn fast_wht_02() {
    let data = &mut vec![1; 1024];

    fwht(data);
}

fn naive_wht_03() {
    let data = &mut vec![1; 4096];

    naive_wht(data, &mut [0; 4096]);
}

fn fast_wht_03() {
    let data = &mut vec![1; 4096];

    fwht(data);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("naive1", |b| b.iter(|| naive_wht_01()));
    c.bench_function("fast1", |b| b.iter(|| fast_wht_01()));
    c.bench_function("naive2", |b| b.iter(|| naive_wht_02()));
    c.bench_function("fast2", |b| b.iter(|| fast_wht_02()));
    c.bench_function("naive3", |b| b.iter(|| naive_wht_03()));
    c.bench_function("fast3", |b| b.iter(|| fast_wht_03()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
