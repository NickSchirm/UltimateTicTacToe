//! # Contains the [FullOrderingFitness] struct

use std::collections::HashMap;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::agent::minimax_agent::MiniMaxAgent;
use crate::game::game_result::GameResult;
use crate::game::game_result::GameResult::Win;
use crate::game::player::Player::{One, Two};
use crate::game::Game;
use crate::genetic_algorithm::fitness::FitnessFunction;
use crate::genetic_algorithm::gene::Gene;
use crate::heuristic::parameterized_heuristic::ParameterizedHeuristic;

/// # Struct representing a full ordering fitness function
///
/// The fitness function is used to calculate the fitness of the genes.
pub struct FullOrderingFitness {
    depth: u32,
    quiescence_depth: u32,
}

impl FullOrderingFitness {
    pub fn new(depth: u32, quiescence_depth: u32) -> Self {
        FullOrderingFitness {
            depth,
            quiescence_depth,
        }
    }
}

impl FitnessFunction for FullOrderingFitness {
    fn calculate_fitness(&self, genes: Vec<Gene>) -> Vec<(Gene, f64)> {
        let enriched_genes: Vec<Vec<(usize, Gene)>> = genes
            .clone()
            .into_iter()
            .enumerate()
            .combinations(2)
            .collect();
        let mut genes_with_fitness = HashMap::with_capacity(genes.len());

        enriched_genes
            .into_par_iter()
            .map(|pair| {
                let (lhs_index, lhs) = pair[0].clone();
                let (rhs_index, rhs) = pair[1].clone();

                let mut lhs_fitness = 0.;
                let mut rhs_fitness = 0.;

                match self.play_game_with(lhs.clone(), rhs.clone(), self.depth, self.quiescence_depth) {
                    Win(One) => {
                        lhs_fitness += 1.;
                        rhs_fitness -= 1.;
                    }
                    Win(Two) => {
                        lhs_fitness -= 1.;
                        rhs_fitness += 1.;
                    }
                    _ => (),
                }

                match self.play_game_with(rhs.clone(), lhs.clone(), self.depth, self.quiescence_depth) {
                    Win(One) => {
                        lhs_fitness -= 1.;
                        rhs_fitness += 1.;
                    }
                    Win(Two) => {
                        lhs_fitness += 1.;
                        rhs_fitness -= 1.;
                    }
                    _ => (),
                }

                ((lhs_index, lhs_fitness), (rhs_index, rhs_fitness))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|((lhs_index, lhs_fitness), (rhs_index, rhs_fitness))| {
                let lhs = genes_with_fitness.entry(lhs_index).or_insert(0.);
                *lhs += lhs_fitness;

                let rhs = genes_with_fitness.entry(rhs_index).or_insert(0.);
                *rhs += rhs_fitness;
            });

        genes_with_fitness
            .into_iter()
            .map(|(i, fitness)| (genes[i].clone(), fitness))
            .collect()
    }
}
