//! Array
//!
//! This module provides a simple array type that is a wrapper around a `nalgebra` vector.
//!

use nalgebra::{DVector, Scalar};
use num::{One, Zero};
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

/// A simple array type that is a wrapper around a `nalgebra` vector.
#[derive(Debug, Clone)]
pub struct Array<T: Scalar = f64> {
    data: DVector<T>,
}

impl<T: Scalar + Zero + One> Array<T> {
    pub fn zeros(n: usize) -> Self {
        Array {
            data: DVector::zeros(n),
        }
    }

    pub fn ones(n: usize) -> Self {
        Array {
            data: DVector::from_element(n, T::one()),
        }
    }
}

impl<T: Scalar + fmt::Display> fmt::Display for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector with length: {}\n {}", self.len(), self.data)
    }
}

impl<T: Scalar> Deref for Array<T> {
    type Target = DVector<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Scalar> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Scalar> From<DVector<T>> for Array<T> {
    fn from(value: DVector<T>) -> Self {
        Array { data: value }
    }
}

impl<T: Scalar> From<Vec<T>> for Array<T> {
    fn from(value: Vec<T>) -> Self {
        Array {
            data: DVector::from_vec(value),
        }
    }
}

impl<T: Scalar, const N: usize> From<[T; N]> for Array<T> {
    fn from(value: [T; N]) -> Self {
        Array {
            data: DVector::from_vec(value.into_iter().collect()),
        }
    }
}

impl<T: Scalar> From<&[T]> for Array<T> {
    fn from(value: &[T]) -> Self {
        Array {
            data: DVector::from_vec(value.to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let a: Array<f64> = Array::zeros(3);
        assert_eq!(a.data, DVector::zeros(3));
        println!("{}", a);
    }

    #[test]
    fn test_ones() {
        let a: Array<f64> = Array::ones(3);
        assert_eq!(a.data, DVector::from_element(3, 1.0));
    }
}
