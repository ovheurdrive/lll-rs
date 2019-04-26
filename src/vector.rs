use rug::*;
use std::fmt;
use std::ops::{Index, IndexMut};

/**
 * The `Vector` trait describes the general properties of an element in a vector space.
 */
pub trait Vector {
    /// Returns the vector's dimension
    fn dimension(&self) -> usize;

    /// Add two vectors together
    fn add(&self, other: &Self) -> Self;

    /// Substract two vectors
    fn sub(&self, other: &Self) -> Self;

    /// Initialise vector type
    fn init(dimension: usize) -> Self;

    /// Basis vector
    fn basis_vector(&self, position: usize) -> Self;
}

pub trait Dot<T> {
    fn dot(&self, other: &Self) -> T;
}
/**
 * Implementation of vectors in a vector space over the (field) `K`
 */
pub struct VectorF {
    /// Underlying representation of the vector as a list of coefficients
    coefficients: Vec<f64>,

    /// Dimension of the vector
    dimension: usize,
}

impl Vector for VectorF {
    /**
     * Return a basis vector for the vector space
     *  `position`: number of the basis vector (0..n)
     */
    fn basis_vector(&self, position: usize) -> Self {
        assert!(position < self.dimension);

        let mut coefficients = vec![0.0; self.dimension()];
        coefficients[position] = 1.0;

        Self {
            coefficients,
            dimension: self.dimension(),
        }
    }

    /**
     * Create a new `VectorF` with default values, of size `dimension`
     */
    fn init(dimension: usize) -> Self {
        Self {
            coefficients: vec![Default::default(); dimension],
            dimension,
        }
    }

    /**
     * Return the vector's dimension
     */
    fn dimension(&self) -> usize {
        self.dimension
    }

    /**
     * Add two vectors of the same size
     */
    fn add(&self, other: &Self) -> Self {
        let n = self.dimension();

        assert_eq!(n, other.dimension());

        Self::from_vector(
            (0..n)
                .map(|i| self.coefficients[i] + other.get_coefficient(i))
                .collect(),
        )
    }

    /**
     * Subtract the vector `other` from this vector
     */
    fn sub(&self, other: &Self) -> Self {
        let n = self.dimension();

        assert_eq!(n, other.dimension());

        Self::from_vector(
            (0..n)
                .map(|i| self.coefficients[i] - other.get_coefficient(i))
                .collect(),
        )
    }
}

impl Dot<f64> for VectorF {
    /**
     * Dot product between two vectors
     */
    fn dot(&self, other: &Self) -> f64 {
        let n = self.dimension();
        assert_eq!(n, other.dimension());

        (0..n)
            .map(|i| self.coefficients[i] * other.get_coefficient(i))
            .sum()
    }
}

impl VectorF {
    /**
     * Return vector coefficient
     */
    pub fn get_coefficient(&self, position: usize) -> f64 {
        assert!(position < self.dimension());
        self.coefficients[position]
    }

    /**
     * Set vector coefficient
     */
    pub fn set_coefficient(&mut self, position: usize, value: f64) {
        assert!(position < self.dimension());
        self.coefficients[position] = value;
    }

    /**
     * Create from a `Vec`
     */
    pub fn from_vector(coefficients: Vec<f64>) -> Self {
        Self {
            dimension: coefficients.len(),
            coefficients,
        }
    }

    /// Multiplication by a scalar
    pub fn mulf(&self, other: f64) -> Self {
        let n = self.dimension();

        Self::from_vector((0..n).map(|i| self.coefficients[i] * other).collect())
    }
}

impl Index<usize> for VectorF {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.coefficients[index]
    }
}

impl IndexMut<usize> for VectorF {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.coefficients[index]
    }
}

impl fmt::Debug for VectorF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.coefficients)
    }
}

pub struct BigVector {
    coefficients: Vec<Integer>,
    dimension: usize,
}

impl Vector for BigVector {
    fn basis_vector(&self, position: usize) -> Self {
        assert!(position < self.dimension);

        let mut coefficients = vec![Integer::from(0); self.dimension()];
        coefficients[position] = Integer::from(1);

        Self {
            coefficients,
            dimension: self.dimension(),
        }
    }

    fn init(dimension: usize) -> Self {
        Self {
            coefficients: vec![Default::default(); dimension],
            dimension,
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn add(&self, other: &Self) -> Self {
        let n = self.dimension();

        assert_eq!(n, other.dimension());

        Self::from_vector(
            (0..n)
                .map(|i| Integer::from(&self.coefficients[i]) + other.get_coefficient(i))
                .collect(),
        )
    }

    fn sub(&self, other: &Self) -> Self {
        let n = self.dimension();

        assert_eq!(n, other.dimension());

        Self::from_vector(
            (0..n)
                .map(|i| Integer::from(&self.coefficients[i]) - other.get_coefficient(i))
                .collect(),
        )
    }
}

impl BigVector {
    /**
     * Return vector coefficient
     */
    pub fn get_coefficient(&self, position: usize) -> Integer {
        assert!(position < self.dimension());
        Integer::from(&self.coefficients[position])
    }

    /**
     * Set vector coefficient
     */
    pub fn set_coefficient(&mut self, position: usize, value: Integer) {
        assert!(position < self.dimension());
        self.coefficients[position] = value;
    }

    /**
     * Create from a `Vec`
     */
    pub fn from_vector(coefficients: Vec<Integer>) -> Self {
        Self {
            dimension: coefficients.len(),
            coefficients,
        }
    }

    /// Multiplication by a scalar
    pub fn mulf(&self, other: Integer) -> Self {
        let n = self.dimension();

        Self::from_vector(
            (0..n)
                .map(|i| Integer::from(&self.coefficients[i]) * Integer::from(&other))
                .collect(),
        )
    }
}

impl Dot<Integer> for BigVector {
    fn dot(&self, other: &Self) -> Integer {
        let n = self.dimension();
        assert_eq!(n, other.dimension());

        (0..n)
            .map(|i| Integer::from(&self.coefficients[i]) * other.get_coefficient(i))
            .sum()
    }
}

impl Index<usize> for BigVector {
    type Output = Integer;

    fn index(&self, index: usize) -> &Integer {
        &self.coefficients[index]
    }
}

impl IndexMut<usize> for BigVector {
    fn index_mut(&mut self, index: usize) -> &mut Integer {
        &mut self.coefficients[index]
    }
}

impl fmt::Debug for BigVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.coefficients)
    }
}
