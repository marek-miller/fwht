//! Fast Walsh–Hadamard transform
//!
//! Wikipedia: https://en.wikipedia.org/wiki/Fast_Walsh%E2%80%93Hadamard_transform
//! See also: https://doi.org/10.1109/TC.1976.1674569

#![feature(portable_simd)]
#![feature(slice_as_chunks)]

use std::{
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign,
    },
    simd::Simd,
};

#[inline]
pub fn wht2<T>(data: &mut [T; 2])
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy,
{
    let d = data;
    let (x0, x1) = (d[0] + d[1], d[0] - d[1]);
    (d[0], d[1]) = (x0, x1);
}

#[inline]
pub fn wht4<T>(data: &mut [T; 4])
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy,
{
    let d = data;
    let (x0, x1, x2, x3) = (
        d[0] + d[1] + d[2] + d[3],
        d[0] - d[1] + d[2] - d[3],
        d[0] + d[1] - d[2] - d[3],
        d[0] - d[1] - d[2] + d[3],
    );
    (d[0], d[1], d[2], d[3]) = (x0, x1, x2, x3);
}

#[inline]
pub fn wht8<T>(data: &mut [T; 8])
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy,
{
    let d = data;
    let (x0, x1, x2, x3, x4, x5, x6, x7) = (
        d[0] + d[1] + d[2] + d[3] + d[4] + d[5] + d[6] + d[7],
        d[0] - d[1] + d[2] - d[3] + d[4] - d[5] + d[6] - d[7],
        d[0] + d[1] - d[2] - d[3] + d[4] + d[5] - d[6] - d[7],
        d[0] - d[1] - d[2] + d[3] + d[4] - d[5] - d[6] + d[7],
        d[0] + d[1] + d[2] + d[3] - d[4] - d[5] - d[6] - d[7],
        d[0] - d[1] + d[2] - d[3] - d[4] + d[5] - d[6] + d[7],
        d[0] + d[1] - d[2] - d[3] - d[4] - d[5] + d[6] + d[7],
        d[0] - d[1] - d[2] + d[3] - d[4] + d[5] + d[6] - d[7],
    );

    (d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]) =
        (x0, x1, x2, x3, x4, x5, x6, x7);
}

/// In-place fast Walsh-Hadamard transform of slice `data`.
pub fn fwht<T>(data: &mut [T])
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy,
{
    let mut h = 1;
    let l = data.len();
    while h < l {
        for i in (0..l).step_by(h * 2) {
            for j in i..i + h {
                let x = data[j];
                let y = data[j + h];
                data[j] = x + y;
                data[j + h] = x - y;
            }
        }
        h *= 2;
    }
}

/// data.len() is assumed to be divisible by 4.
/// scratch.len() >= data.len()/4
pub fn fwht4(
    data: &mut [i64],
    scratch: &mut [Simd<i64, 4>],
) {
    for (i, chunk) in data.as_chunks_mut().0.iter_mut().enumerate() {
        wht4(chunk);
        scratch[i] = Simd::from(*chunk);
    }
    fwht(scratch);
    for i in 0..data.len() / 4 {
        for j in 0..4 {
            data[i * 4 + j] = scratch[i][j]
        }
    }
}

/// data.len() is assumed to be divisible by 8.
/// scratch.len() >= data.len()/8
pub fn fwht8(
    data: &mut [i64],
    scratch: &mut [Simd<i64, 8>],
) {
    for (i, chunk) in data.as_chunks_mut().0.iter_mut().enumerate() {
        wht8(chunk);
        scratch[i] = Simd::from(*chunk);
    }
    fwht(scratch);
    for i in 0..data.len() / 8 {
        for j in 0..8 {
            data[i * 8 + j] = scratch[i][j]
        }
    }
}

#[must_use]
pub fn binary_dot_product(
    i: usize,
    j: usize,
) -> usize {
    let prod = i & j;
    (0..usize::BITS).fold(0, |acc, k| prod >> k & 1 ^ acc)
}

pub struct Naive<T> {
    scratch: Vec<T>,
}

impl<T> Naive<T>
where
    T: AddAssign + SubAssign + Copy,
{
    pub fn new() -> Self {
        Self {
            scratch: Vec::new(),
        }
    }

    pub fn init(
        &mut self,
        data: &[T],
    ) {
        self.scratch.clear();
        self.scratch.reserve_exact(data.len());
    }

    /// Naive implementation of Walsh-Hadamard transform of slice `data`.
    ///
    /// The result is written to `data`.
    pub fn process(
        &mut self,
        data: &mut [T],
    ) {
        let l = data.len();
        self.scratch.clear();
        for i in 0..l {
            self.scratch.push(data[0]);
            for j in 1..l {
                if binary_dot_product(i, j) == 0 {
                    self.scratch[i] += data[j]
                } else {
                    self.scratch[i] -= data[j]
                }
            }
        }
        for i in 0..l {
            data[i] = self.scratch[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn naive_wht_01() {
        let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
        let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];

        let mut transform = Naive::new();
        transform.init(data);

        transform.process(data);
        assert_eq!(*data, expected);
    }

    #[test]
    fn fwht_01() {
        let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
        let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];

        fwht(data);
        assert_eq!(*data, expected);
    }

    #[test]
    fn inlined_wht8_01() {
        let data = &mut [1, 0, 1, 0, 0, 1, 1, 0];
        let expected = [4, 2, 0, -2, 0, 2, 0, 2];

        wht8(data);
        assert_eq!(*data, expected);
    }

    #[test]
    fn fwht8_01() {
        let data = &mut [1, 0, 1, 0, 0, 1, 1, 0];
        let expected = [4, 2, 0, -2, 0, 2, 0, 2];

        let frame = Simd::from([0; 8]);
        let mut scratch = vec![frame; 8];

        fwht8(data, &mut scratch);
        assert_eq!(*data, expected);
    }

    #[test]
    fn fwht4_01() {
        let mut arr = [0i32; 1024];
        rand::thread_rng().fill(&mut arr[..]);

        let data1 = &mut arr.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let data2 = &mut data1.to_owned();
        let data3 = &mut data1.to_owned();

        let mut naive = Naive::new();
        naive.init(data1);

        naive.process(data1);
        fwht(data2);
        let frame = Simd::from([0; 4]);
        let mut scratch = vec![frame; 256];
        fwht4(data3, &mut scratch);

        assert_eq!(data1, data2);
        assert_eq!(data1, data3);
    }

    #[test]
    fn fwht8_02() {
        let mut arr = [0i32; 1024];
        rand::thread_rng().fill(&mut arr[..]);

        let data1 = &mut arr.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let data2 = &mut data1.to_owned();
        let data3 = &mut data1.to_owned();

        let mut naive = Naive::new();
        naive.init(data1);

        naive.process(data1);
        fwht(data2);
        let frame = Simd::from([0; 8]);
        let mut scratch = vec![frame; 256];
        fwht8(data3, &mut scratch);

        assert_eq!(data1, data2);
        assert_eq!(data1, data3);
    }

    #[test]
    fn binary_dot_product_01() {
        assert_eq!(binary_dot_product(0b00, 0b00), 0);
        assert_eq!(binary_dot_product(0b00, 0b01), 0);
        assert_eq!(binary_dot_product(0b00, 0b10), 0);
        assert_eq!(binary_dot_product(0b00, 0b11), 0);

        assert_eq!(binary_dot_product(0b01, 0b00), 0);
        assert_eq!(binary_dot_product(0b01, 0b01), 1);
        assert_eq!(binary_dot_product(0b01, 0b10), 0);
        assert_eq!(binary_dot_product(0b01, 0b11), 1);

        assert_eq!(binary_dot_product(0b10, 0b00), 0);
        assert_eq!(binary_dot_product(0b10, 0b01), 0);
        assert_eq!(binary_dot_product(0b10, 0b10), 1);
        assert_eq!(binary_dot_product(0b10, 0b11), 1);

        assert_eq!(binary_dot_product(0b11, 0b00), 0);
        assert_eq!(binary_dot_product(0b11, 0b01), 1);
        assert_eq!(binary_dot_product(0b11, 0b10), 1);
        assert_eq!(binary_dot_product(0b11, 0b11), 0);
    }

    #[test]
    fn binary_dot_product_02() {
        assert_eq!(binary_dot_product(0b1, 0b1), 1);
        assert_eq!(binary_dot_product(0b11, 0b11), 0);
        assert_eq!(binary_dot_product(0b111, 0b111), 1);
        assert_eq!(binary_dot_product(0b1111, 0b1111), 0);
        assert_eq!(binary_dot_product(0b11111, 0b11111), 1);

        assert_eq!(binary_dot_product(0b1011, 0b1100), 1);
        assert_eq!(binary_dot_product(0b1011, 0b1010), 0);

        assert_eq!(binary_dot_product(0b10_1001, 0b10_1001), 1);
        assert_eq!(binary_dot_product(0b1010_0001, 0b1010_0001), 1);
        assert_eq!(binary_dot_product(0b00_1010_0001, 0b00_1010_0001), 1);

        assert_eq!(binary_dot_product(0b00_0010_0001, 0b00_1010_0001), 0);
    }
}
