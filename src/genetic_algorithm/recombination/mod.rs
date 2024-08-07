//! # Contains the [Recombination] trait and implementations
//!
//! A recombination is used to recombine two genes.
//! The recombination can be used to create new genes from two given genes.
//!
//! The way the genes are recombined is determined by the implementation.

pub mod one_point_crossover;
pub mod two_point_crossover;

use crate::genetic_algorithm::gene::Gene;
use itertools::Itertools;

/// # Trait representing a recombination
///
/// A recombination is used to recombine two genes.
/// The recombination can be used to create new genes from two given genes.
pub trait Recombination {
    /// Recombines the given genes
    ///
    /// Two genes are recombined to create two new genes.
    ///
    /// If the amount of genes is odd, then the last gene will be copied without modification.
    /// # Arguments
    /// * `genes` - The genes to recombine
    /// # Returns
    /// The recombined genes
    fn recombine_all(&self, genes: Vec<Gene>) -> Vec<Gene> {
        let mut new_genes = Vec::with_capacity(genes.len());

        let mut iter = genes.iter().tuples();
        // Iterates over the genes in pairs
        for (lhs, rhs) in iter.by_ref() {
            let (new_lhs, new_rhs) = self.recombine(lhs.clone(), rhs.clone());
            new_genes.push(new_lhs);
            new_genes.push(new_rhs);
        }
        // If there is an odd number of genes, the last one is left over
        for leftover in iter.into_buffer() {
            new_genes.push(leftover.clone());
        }

        new_genes
    }

    /// Recombines the given genes
    /// # Arguments
    /// * `lhs` - The first gene
    /// * `rhs` - The second gene
    /// # Returns
    /// The recombined genes
    fn recombine(&self, lhs: Gene, rhs: Gene) -> (Gene, Gene);
}
