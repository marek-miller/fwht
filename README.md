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
naive1                  time:   [958.17 ns 973.02 ns 987.38 ns]
                        change: [-13.433% -11.055% -8.8332%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe

fast1                   time:   [69.338 ns 70.144 ns 71.198 ns]
                        change: [+4.1695% +6.3364% +8.5159%] (p = 0.00 < 0.05)
                        Performance has regressed.

inlined8                time:   [1.6276 ns 1.6297 ns 1.6321 ns]
                        change: [-3.7659% -2.6435% -1.2538%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

fast2                   time:   [217.56 µs 217.96 µs 218.41 µs]
                        change: [-6.7256% -6.1131% -5.4981%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

simd_fwht4              time:   [217.33 µs 217.51 µs 217.71 µs]
                        change: [-0.5967% +0.4177% +1.7902%] (p = 0.58 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  6 (6.00%) high severe

simd_fwht8              time:   [169.15 µs 169.30 µs 169.46 µs]
                        change: [-8.4902% -5.9662% -3.4868%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe
```
