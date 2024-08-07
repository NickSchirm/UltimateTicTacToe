use crate::genetic_algorithm::gene::Gene;
use crate::genetic_algorithm::recombination::Recombination;
use itertools::Itertools;
use rand::Rng;

pub struct OnePointCrossover {}

impl Recombination for OnePointCrossover {
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

    fn recombine(&self, lhs: Gene, rhs: Gene) -> (Gene, Gene) {
        let mut rng = rand::thread_rng();
        let crossover_point = rng.gen_range(0..lhs.get_values().len());
        let mut new_lhs = Vec::new();
        let mut new_rhs = Vec::new();
        new_lhs.extend_from_slice(&lhs.get_values()[0..crossover_point]);
        new_lhs.extend_from_slice(&rhs.get_values()[crossover_point..]);
        new_rhs.extend_from_slice(&rhs.get_values()[0..crossover_point]);
        new_rhs.extend_from_slice(&lhs.get_values()[crossover_point..]);
        (Gene::with_values(new_lhs), Gene::with_values(new_rhs))
    }
}
