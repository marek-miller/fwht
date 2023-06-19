//! Fast Walsh–Hadamard transform
//!
//! Wikipedia: https://en.wikipedia.org/wiki/Fast_Walsh%E2%80%93Hadamard_transform
//! See also: https://doi.org/10.1109/TC.1976.1674569

use std::ops::{Add, Sub};

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

pub fn binary_dot_product(i: usize, j: usize) -> usize {
    (0..usize::BITS).fold(0, |acc, k| (i & j) >> k & 1 ^ acc)
}

pub fn naive_wht<T>(data: &[T], output: &mut [T])
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy,
{
    let l = data.len();
    assert_eq!(output.len(), l);
    for i in 0..l {
        for j in 0..l {
            if binary_dot_product(i, j) == 0 {
                output[i] = output[i] + data[j]
            } else {
                output[i] = output[i] - data[j]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(binary_dot_product(0b1011, 0b1100), 1);
        assert_eq!(binary_dot_product(0b1011, 0b1010), 0);

        assert_eq!(binary_dot_product(0b1, 0b1), 1);
        assert_eq!(binary_dot_product(0b11, 0b11), 0);
        assert_eq!(binary_dot_product(0b111, 0b111), 1);
        assert_eq!(binary_dot_product(0b1111, 0b1111), 0);
        assert_eq!(binary_dot_product(0b11111, 0b11111), 1);

        assert_eq!(binary_dot_product(0b101001, 0b101001), 1);
        assert_eq!(binary_dot_product(0b10100001, 0b10100001), 1);
        assert_eq!(binary_dot_product(0b0010100001, 0b0010100001), 1);

        assert_eq!(binary_dot_product(0b0000100001, 0b0010100001), 0);
    }

    #[test]
    fn naive_wht_01() {
        let data = &mut vec![1, 0, 1, 0, 0, 1, 1, 0];
        let expected = vec![4, 2, 0, -2, 0, 2, 0, 2];

        let mut output = vec![0; 8];
        naive_wht(data, &mut output);
        assert_eq!(output, expected);
    }
}
