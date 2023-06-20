# fwht

Fast Walsh–Hadamard transform

- Wikipedia: https://en.wikipedia.org/wiki/Fast_Walsh%E2%80%93Hadamard_transform
- See also: https://doi.org/10.1109/TC.1976.1674569

## WIP

The idea is to perform initial step by inlining explicitly Hadamard transform of
size 8, and then use SIMD vectors to execute remaining fast transforms in
parallel.

## Preliminary results

Simple benchmarking suggests SIMD version of FWT is the fastest:

```
     Running benches/my_benchmark.rs (target/release/deps/my_benchmark-63a9a630d87c2f69)
Gnuplot not found, using plotters backend
naive1                  time:   [883.36 ns 890.16 ns 899.70 ns]
                        change: [-2.0808% -0.6008% +0.7826%] (p = 0.43 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

fast1                   time:   [76.254 ns 76.467 ns 76.686 ns]
                        change: [+1.4698% +3.6809% +5.7454%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

inlined8                time:   [1.3883 ns 1.3896 ns 1.3909 ns]
                        change: [-3.8359% -3.3977% -2.9544%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

fast2                   time:   [243.62 µs 244.31 µs 244.97 µs]
                        change: [+3.7880% +4.5522% +5.3498%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

simd2                   time:   [177.87 µs 178.12 µs 178.53 µs]
                        change: [-5.1520% -4.7018% -4.2751%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) high mild
  10 (10.00%) high severe
```
