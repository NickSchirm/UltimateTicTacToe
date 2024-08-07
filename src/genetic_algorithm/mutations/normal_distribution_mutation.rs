//! # Contains the [NormalDistributionMutation] struct
use rand_distr::Distribution;
use rand_distr::Normal;

use crate::genetic_algorithm::gene::Gene;
use crate::genetic_algorithm::mutation::Mutation;

/// # Mutation that mutates a gene using a normal distribution
///
/// This mutation adds a random value from a normal distribution to each value of the gene.
/// The normal distribution is centered around 0.
/// The standard deviation of the normal distribution can be set.
pub struct NormalDistributionMutation {
    normal: Normal<f64>,
}

impl NormalDistributionMutation {
    /// Creates a new NormalDistributionMutation with the given standard deviation
    /// # Arguments
    /// * `std_dev` - The standard deviation of the normal distribution
    /// # Returns
    /// The created NormalDistributionMutation
    pub fn new(std_dev: f64) -> Self {
        NormalDistributionMutation {
            normal: Normal::new(0., std_dev).unwrap(),
        }
    }
}

impl Mutation for NormalDistributionMutation {
    fn mutate(&mut self, gene: Gene) -> Gene {
        let mut rng = rand::thread_rng();
        Gene::with_values(
            gene.clone()
                .get_values()
                .iter()
                .map(|value| value + self.normal.sample(&mut rng))
                .collect(),
        )
    }
}

impl Default for NormalDistributionMutation {
    fn default() -> Self {
        NormalDistributionMutation {
            normal: Normal::new(0.0, 1.0).unwrap(),
        }
    }
}
