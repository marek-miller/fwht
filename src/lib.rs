//! Fast Walsh–Hadamard transform
//!
//! Wikipedia: https://en.wikipedia.org/wiki/Fast_Walsh%E2%80%93Hadamard_transform
//! See also: https://doi.org/10.1109/TC.1976.1674569

use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};

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

#[must_use] pub fn binary_dot_product(
    i: usize,
    j: usize,
) -> usize {
    (0..usize::BITS).fold(0, |acc, k| (i & j) >> k & 1 ^ acc)
}

/// Naive implementation of Walsh-Hadamard transform of slice `data`.
///
/// The result is written to `data`.  Uses `scratch` as a working space.
/// Assumes `scratch.len() >= data.len()`.
pub fn naive_wht<T>(
    data: &mut [T],
    scratch: &mut [T],
) where
    T: AddAssign + SubAssign + Copy,
{
    let l = data.len();
    assert!(scratch.len() >= l);
    for i in 0..l {
        for j in 0..l {
            if binary_dot_product(i, j) == 0 {
                scratch[i] += data[j]
            } else {
                scratch[i] -= data[j]
            }
        }
    }
    for i in 0..l {
        data[i] = scratch[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_wht_01() {
        let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
        let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];

        naive_wht(data, &mut [0; 8]);
        assert_eq!(*data, expected);
    }

    #[test]
    fn it_works() {
        let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
        let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];

        fwht(data);
        assert_eq!(*data, expected);
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
