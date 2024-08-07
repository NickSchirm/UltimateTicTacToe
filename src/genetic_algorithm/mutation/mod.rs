//! # Contains the [Mutation] trait and implementations
pub mod normal_distribution_mutation;
pub mod shift_mutation;

use crate::genetic_algorithm::gene::Gene;

/// # Trait representing a mutation
///
/// A mutation is a function that changes a gene.
/// The way the gene is changed is determined by the implementation.
pub trait Mutation {
    /// Mutates the given gene
    /// # Arguments
    /// * `gene` - The gene to mutate
    /// # Returns
    /// The mutated gene
    fn mutate(&mut self, gene: Gene) -> Gene;

    /// Mutates all the given genes
    ///
    /// This function calls [Self::mutate] for each gene in the list.
    ///
    /// # Arguments
    /// * `genes` - The genes to mutate
    /// # Returns
    /// The mutated genes
    fn mutate_all(&mut self, genes: Vec<Gene>) -> Vec<Gene> {
        genes.into_iter().map(|gene| self.mutate(gene)).collect()
    }
}
