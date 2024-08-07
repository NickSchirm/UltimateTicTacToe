//! # Contains various modules for the genetic algorithm
//!
//! The genetic algorithm is used to optimize the weights of [ParameterizedHeuristic](crate::heuristic::parameterized_heuristic::ParameterizedHeuristic).

use crate::genetic_algorithm::fitness_function::FitnessFunction;
use itertools::Itertools;

pub mod fitness_function;
pub mod gene;
pub mod mutation;
pub mod recombination;
pub mod selection;

/// # Struct representing a genetic algorithm
///
/// The genetic algorithm is used to optimize the weights of a heuristic.
pub struct GeneticAlgorithm {
    generations: usize,
    genes: Vec<gene::Gene>,
    fitness_function: FitnessFunction,
    selection: Box<dyn selection::Selection>,
    mutation: Box<dyn mutation::Mutation>,
    recombination: Box<dyn recombination::Recombination>,
}

impl GeneticAlgorithm {
    pub fn new(
        generations: usize,
        genes: Vec<gene::Gene>,
        selection: Box<dyn selection::Selection>,
        mutation: Box<dyn mutation::Mutation>,
        recombination: Box<dyn recombination::Recombination>,
        depth: u32,
        quiescence_depth: u32,
    ) -> Self {
        GeneticAlgorithm {
            generations,
            genes,
            fitness_function: FitnessFunction::new(depth, quiescence_depth),
            selection,
            mutation,
            recombination,
        }
    }

    /// Runs the genetic algorithm
    ///
    /// This function runs the genetic algorithm for the given number of generations.
    pub fn run(&mut self) {
        for _ in 0..self.generations {
            let genes_with_fitness = self.fitness_function.calculate_fitness(self.genes.clone());

            let selected_genes = self.selection.select(genes_with_fitness);

            let mutated_genes = self.mutation.mutate_all(selected_genes);

            self.genes = self.recombination.recombine_all(mutated_genes);
        }

        let best = self
            .fitness_function
            .calculate_fitness(self.genes.clone())
            .into_iter()
            .sorted_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
            .next();
        println!("Best gene: {:?}", best)
    }
}

mod tests {
    use super::*;
    use crate::genetic_algorithm::gene::Gene;
    use crate::genetic_algorithm::mutation::normal_distribution_mutation::NormalDistributionMutation;
    use crate::genetic_algorithm::recombination::two_point_crossover::TwoPointCrossover;
    use crate::genetic_algorithm::selection::roulette_wheel_selection::RouletteWheelSelection;
    use crate::heuristic::parameterized_heuristic::NUM_FEATURES;

    #[test]
    fn test_genetic_algorithm() {
        let genes = vec![
            Gene::new(NUM_FEATURES),
            Gene::new(NUM_FEATURES),
            Gene::new(NUM_FEATURES),
            Gene::new(NUM_FEATURES),
        ];

        let mut genetic_algorithm = GeneticAlgorithm::new(
            1,
            genes,
            Box::new(RouletteWheelSelection {}),
            Box::new(NormalDistributionMutation::new(0.1)),
            Box::new(TwoPointCrossover {}),
            2,
            2,
        );

        genetic_algorithm.run();
    }
}
