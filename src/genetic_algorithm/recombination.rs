//! # Contains the [Recombination] trait
use crate::genetic_algorithm::gene::Gene;

/// # Trait representing a recombination
pub trait Recombination {
    /// Recombines the given genes
    /// # Arguments
    /// * `genes` - The genes to recombine
    /// # Returns
    /// The recombined genes
    fn recombine_all(&self, genes: Vec<Gene>) -> Vec<Gene>;

    /// Recombines the given genes
    /// # Arguments
    /// * `lhs` - The first gene
    /// * `rhs` - The second gene
    /// # Returns
    /// The recombined genes
    fn recombine(&self, lhs: Gene, rhs: Gene) -> (Gene, Gene);
}
