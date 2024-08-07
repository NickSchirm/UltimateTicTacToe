//! # Contains the [Gene] struct

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
        let mut values = Vec::with_capacity(num_values);

        for _ in 0..num_values {
            values.push(rand::random::<f64>());
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
