//! # Contains the [Recombination] trait and implementations
pub mod one_point_crossover;
pub mod two_point_crossover;

use itertools::Itertools;
use crate::genetic_algorithm::gene::Gene;

/// # Trait representing a recombination
pub trait Recombination {
	/// Recombines the given genes
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
