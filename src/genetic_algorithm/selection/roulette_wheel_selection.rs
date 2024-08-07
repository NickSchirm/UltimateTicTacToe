//! # Contains the [RouletteWheelSelection] struct
use crate::genetic_algorithm::gene::Gene;
use crate::genetic_algorithm::selection::Selection;

/// # Selection that selects genes using the roulette wheel selection
///
/// The roulette wheel selection selects genes based on their fitness.
/// The higher the fitness, the higher the chance of being selected.
///
/// The selection can be visualized as a roulette wheel where each gene has a slice of the wheel.
/// The size of the slice is proportional to the gene's fitness.
pub struct RouletteWheelSelection {}

impl Selection for RouletteWheelSelection {
    fn select(&self, genes: Vec<(Gene, f64)>) -> Vec<Gene> {
        let total_fitness: f64 = genes.iter().map(|(_, fitness)| fitness).sum();
        let mut selected_genes = Vec::new();
        for _ in 0..genes.len() {
            let mut random = rand::random::<f64>() * total_fitness;
            for (gene, fitness) in &genes {
                random -= fitness;
                if random <= 0.0 {
                    selected_genes.push(gene.clone());
                    break;
                }
            }
        }
        selected_genes
    }
}
