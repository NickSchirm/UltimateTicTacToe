//! # Contains various modules for the genetic algorithm
//!
//! The genetic algorithm is used to optimize the weights of [ParameterizedHeuristic](crate::heuristic::parameterized_heuristic::ParameterizedHeuristic).

use crate::genetic_algorithm::fitness::full_ordering_fitness::FullOrderingFitness;
use crate::genetic_algorithm::fitness::FitnessFunction;
use crate::genetic_algorithm::gene::Gene;
use crate::genetic_algorithm::mutation::normal_distribution_mutation::NormalDistributionMutation;
use crate::genetic_algorithm::mutation::Mutation;
use crate::genetic_algorithm::recombination::two_point_crossover::TwoPointCrossover;
use crate::genetic_algorithm::recombination::Recombination;
use crate::genetic_algorithm::selection::roulette_wheel_selection::RouletteWheelSelection;
use crate::genetic_algorithm::selection::Selection;
use crate::heuristic::parameterized_heuristic::NUM_FEATURES;
use itertools::Itertools;
use std::time::Instant;

pub mod fitness;
pub mod gene;
pub mod mutation;
pub mod recombination;
pub mod selection;

/// # Struct representing a genetic algorithm
///
/// The genetic algorithm is used to optimize the weights of a heuristic.
///
/// The fitness, selection, mutation and recombination operators can be set.
/// Multiple implementations are available.
pub struct GeneticAlgorithm {
    generations: usize,
    genes: Vec<Gene>,
    fitness: Box<dyn FitnessFunction>,
    selection: Box<dyn Selection>,
    mutation: Box<dyn Mutation>,
    recombination: Box<dyn Recombination>,
}

impl GeneticAlgorithm {
    pub fn new(
        generations: usize,
        genes: Vec<Gene>,
        fitness: Box<dyn FitnessFunction>,
        selection: Box<dyn Selection>,
        mutation: Box<dyn Mutation>,
        recombination: Box<dyn Recombination>,
    ) -> Self {
        GeneticAlgorithm {
            generations,
            genes,
            fitness,
            selection,
            mutation,
            recombination,
        }
    }

    /// Runs the genetic algorithm
    ///
    /// This function runs the genetic algorithm for the given number of generations.
    pub fn run(&mut self) {
        let pre_run = Instant::now();
        let mut pre_gen = Instant::now();
        for i in 0..self.generations {
            let genes_with_fitness = self.fitness.calculate_fitness(self.genes.clone());

            let selected_genes = self.selection.select(genes_with_fitness);

            let mutated_genes = self.mutation.mutate_all(selected_genes);

            self.genes = self.recombination.recombine_all(mutated_genes);

            println!(
                "Generation {} done in {} seconds",
                i,
                pre_gen.elapsed().as_secs_f32()
            );
            pre_gen = Instant::now();
        }
        println!();
        println!("Genetic algorithm done in {:?}", pre_run.elapsed());
        println!();

        println!("Calculating best gene");
        let best = self
            .fitness
            .calculate_fitness(self.genes.clone())
            .into_iter()
            .sorted_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
            .next();
        println!("Best gene: {:?}", best)
    }
}

pub fn run() {
    let mut genes = vec![];

    for _ in 0..10 {
        genes.push(Gene::new(NUM_FEATURES));
    }

    let mut genetic_algorithm = GeneticAlgorithm::new(
        100,
        genes,
        Box::new(FullOrderingFitness::new(5, 1)),
        Box::new(RouletteWheelSelection {}),
        Box::new(NormalDistributionMutation::new(0.1)),
        Box::new(TwoPointCrossover {}),
    );

    genetic_algorithm.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genetic_algorithm::fitness::full_ordering_fitness::FullOrderingFitness;
    use crate::genetic_algorithm::gene::Gene;
    use crate::genetic_algorithm::mutation::normal_distribution_mutation::NormalDistributionMutation;
    use crate::genetic_algorithm::recombination::two_point_crossover::TwoPointCrossover;
    use crate::genetic_algorithm::selection::roulette_wheel_selection::RouletteWheelSelection;
    use crate::heuristic::parameterized_heuristic::NUM_FEATURES;

    #[test]
    fn test_genetic_algorithm() {
        let mut genes = vec![];

        for _ in 0..10 {
            genes.push(Gene::new(NUM_FEATURES));
        }

        let mut genetic_algorithm = GeneticAlgorithm::new(
            1,
            genes,
            Box::new(FullOrderingFitness::new(4, 1)),
            Box::new(RouletteWheelSelection {}),
            Box::new(NormalDistributionMutation::new(0.1)),
            Box::new(TwoPointCrossover {}),
        );

        genetic_algorithm.run();
    }
}
