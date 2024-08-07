//! # Contains the [Mutation] trait
use crate::genetic_algorithm::gene::Gene;

/// # Trait representing a mutation
///
/// A mutation is a function that changes a gene.
pub trait Mutation {
    /// Mutates the given gene
    /// # Arguments
    /// * `gene` - The gene to mutate
    /// # Returns
    /// The mutated gene
    fn mutate(&self, gene: Gene) -> Gene;

    /// Mutates all the given genes
    ///
    /// This function calls [Self::mutate] for each gene in the list.
    ///
    /// # Arguments
    /// * `genes` - The genes to mutate
    /// # Returns
    /// The mutated genes
    fn mutate_all(&self, genes: Vec<Gene>) -> Vec<Gene> {
        genes.into_iter().map(|gene| self.mutate(gene)).collect()
    }
}
