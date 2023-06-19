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

pub fn fwht(input: &[i32]) -> Vec<i32> {
    input.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![1, 0, 1, 0, 0, 1, 1, 0];
        let expec = vec![4, 2, 0, -2, 0, 2, 0, 2];

        let result = fwht(&input);
        assert_eq!(result, expec);
    }
}
