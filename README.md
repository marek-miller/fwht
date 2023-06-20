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
naive1                  time:   [861.77 ns 881.79 ns 905.72 ns]
                        change: [+1.6331% +4.0136% +6.0770%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

fast1                   time:   [69.281 ns 69.409 ns 69.585 ns]
                        change: [-1.6221% -0.9912% -0.3198%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  10 (10.00%) high severe

inlined8                time:   [1.4237 ns 1.4258 ns 1.4283 ns]
                        change: [-4.0250% -3.4762% -2.8652%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

fast2                   time:   [6.3639 µs 6.3720 µs 6.3811 µs]
                        change: [-0.8641% +0.7917% +2.9306%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe

simd_fwht4              time:   [4.4691 µs 4.4734 µs 4.4777 µs]
                        change: [-1.1114% -0.6259% -0.0226%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) high mild
  9 (9.00%) high severe

simd_fwht8              time:   [3.6244 µs 3.6470 µs 3.6753 µs]
                        change: [+4.0638% +5.4140% +7.4585%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  8 (8.00%) high mild
  9 (9.00%) high severe

```
