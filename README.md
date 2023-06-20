# fwht

Fast Walsh–Hadamard transform

- Wikipedia: https://en.wikipedia.org/wiki/Fast_Walsh%E2%80%93Hadamard_transform
- See also: https://doi.org/10.1109/TC.1976.1674569

## WIP

The idea is to perform initial step by inlining explicitly Hadamard transform
of size 8, and then use SIMD vectors to execute remaining fast transforms in
parallel.
