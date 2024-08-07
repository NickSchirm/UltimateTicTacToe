//! # Contains the [ShiftMutation] struct

use crate::genetic_algorithm::gene::Gene;
use crate::genetic_algorithm::mutation::Mutation;
use rand::{thread_rng, Rng};

/// # Mutation that shifts the values of a gene
///
/// This mutation shifts the values of a gene by a given amount.
pub struct ShiftMutation {}

impl Mutation for ShiftMutation {
    fn mutate(&self, gene: Gene) -> Gene {
        let len = gene.get_values().len();
        let mut res = Vec::from(vec![0.0; len]);

        let shift = thread_rng().gen_range(0..len);

        for (i, value) in gene.get_values().iter().enumerate() {
            res[(i + shift) % len] = value.clone();
        }

        Gene::with_values(res)
    }
}
