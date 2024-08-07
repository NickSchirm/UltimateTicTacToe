//! # Contains various modules for the genetic algorithm

use crate::genetic_algorithm::fitness_function::FitnessFunction;
use itertools::Itertools;

pub mod fitness_function;
pub mod gene;
pub mod mutation;
pub mod mutations;
pub mod recombination;
pub mod recombinations;
pub mod selection;
pub mod selections;

/// # Struct representing a genetic algorithm
///
/// The genetic algorithm is used to optimize the weights of a heuristic.
pub struct GeneticAlgorithm {
    population_size: usize,
    generations: usize,
    genes: Vec<gene::Gene>,
    fitness_function: FitnessFunction,
    selection: Box<dyn selection::Selection>,
    mutation: Box<dyn mutation::Mutation>,
    recombination: Box<dyn recombination::Recombination>,
    depth: u32,
    quiescence_depth: u32,
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
            population_size: genes.len(),
            generations,
            genes,
            fitness_function: FitnessFunction::new(depth, quiescence_depth),
            selection,
            mutation,
            recombination,
            depth,
            quiescence_depth,
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
    use crate::genetic_algorithm::mutations::shift_mutation::ShiftMutation;
    use crate::genetic_algorithm::recombinations::one_point_crossover::OnePointCrossover;
    use crate::genetic_algorithm::selections::roulette_wheel_selection::RouletteWheelSelection;
    use crate::parameterized_heuristic::NUM_FEATURES;

    #[test]
    fn test_genetic_algorithm() {
        let genes = vec![
            Gene::new(NUM_FEATURES),
            Gene::new(NUM_FEATURES),
            Gene::new(NUM_FEATURES),
            Gene::new(NUM_FEATURES),
        ];

        let mut genetic_algorithm = GeneticAlgorithm::new(
            10,
            genes,
            Box::new(RouletteWheelSelection {}),
            Box::new(ShiftMutation {}),
            Box::new(OnePointCrossover {}),
            2,
            2,
        );

        genetic_algorithm.run();
    }
}
