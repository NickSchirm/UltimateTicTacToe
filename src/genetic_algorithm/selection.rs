//! # Contains the [Selection] trait
use crate::genetic_algorithm::gene::Gene;

/// # Trait representing a selection
///
/// A selection is a function that selects the best genes from a list of genes.
/// The genes are given as a tuple of the gene and its fitness.
/// The selected genes are returned as a list of genes.
pub trait Selection {
    /// Selects the best genes from the given genes
    /// # Arguments
    /// * `genes` - The genes to select from and their fitness
    /// # Returns
    /// The selected genes
    fn select(&self, genes: Vec<(Gene, f64)>) -> Vec<Gene>;
}
