//! # Contains the [Gene] struct

use rand::distributions::{Distribution, Uniform};
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Gene {
    values: Vec<f64>,
}

impl Gene {
    /// Creates a new gene with the given number of values
    ///
    /// The values are initialized with random values between [0, 1).
    /// # Arguments
    /// * `num_values` - The number of values to generate
    /// # Returns
    /// A new gene with the given number of values
    pub fn new(num_values: usize) -> Self {
        Self::with_range(num_values, 0.0..1.0)
    }

    /// Creates a new gene with the given number of values and range
    ///
    /// The values are initialized according to a uniform distribution with the given range.
    /// # Arguments
    /// * `num_values` - The number of values to generate
    /// * `range` - The range of the values
    /// # Returns
    /// A new gene with the given number of values and range
    pub fn with_range(num_values: usize, range: Range<f64>) -> Self {
        let mut values = Vec::with_capacity(num_values);
        let between = Uniform::from(range);
        let mut rng = rand::thread_rng();

        for _ in 0..num_values {
            values.push(between.sample(&mut rng));
        }

        Gene { values }
    }

    /// Creates a new gene with the given values
    /// # Arguments
    /// * `values` - The values of the gene
    /// # Returns
    /// A new gene with the given values
    pub fn with_values(values: Vec<f64>) -> Self {
        Gene { values }
    }

    /// Returns the values of the gene
    /// # Returns
    /// The values of the gene
    pub fn get_values(&self) -> Vec<f64> {
        self.values.clone()
    }
}
