//! See: https://doi.org/10.1109/TC.1976.1674569
//! Wikipedia: https://en.wikipedia.org/wiki/Fast_Walsh%E2%80%93Hadamard_transform
//!
//! Implementation in Python:
//!
//! def fwht(a) -> None:
//! """In-place Fast Walsh–Hadamard Transform of array a."""
//! h = 1
//! while h < len(a):
//!     # perform FWHT
//!     for i in range(0, len(a), h * 2):
//!         for j in range(i, i + h):
//!             x = a[j]
//!             y = a[j + h]
//!             a[j] = x + y
//!             a[j + h] = x - y
//!     # normalize and increment
//!     a /= 2
//!     h *= 2

/// In-place fast Walsh-Hadamard transform of slice `data`.
pub fn fwht(data: &mut [i32]) {
    let mut h = 1;
    let length = data.len();
    while h < length {
        // perform FWHT
        for i in (0..length).step_by(h * 2) {
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
}
