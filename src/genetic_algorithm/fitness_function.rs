//! # Contains the [FitnessFunction] struct

use crate::game::Game;
use crate::game_result::GameResult;
use crate::game_result::GameResult::Win;
use crate::genetic_algorithm::gene::Gene;
use crate::minimax_agent::MiniMaxAgent;
use crate::parameterized_heuristic::ParameterizedHeuristic;
use crate::player::Player::{One, Two};
use std::collections::HashMap;

/// # Struct representing a fitness function
///
/// The fitness function is used to calculate the fitness of the genes.
pub struct FitnessFunction {
    depth: u32,
    quiescence_depth: u32,
}

impl FitnessFunction {
    pub fn new(depth: u32, quiescence_depth: u32) -> Self {
        FitnessFunction {
            depth,
            quiescence_depth,
        }
    }

    /// Calculates the fitness of the given genes
    ///
    /// The fitness is calculated by playing games with the genes.
    /// The fitness is the number of games won minus the number of games lost.
    ///
    /// # Arguments
    /// * `genes` - The genes to calculate the fitness for
    /// # Returns
    /// The genes with their fitness
    pub(crate) fn calculate_fitness(&self, genes: Vec<Gene>) -> Vec<(Gene, f64)> {
        let len = genes.len();
        let mut genes_with_fitness: HashMap<usize, f64> = HashMap::with_capacity(len);

        for i in 0..len {
            let lhs = genes[i].clone();
            let mut lhs_fitness = genes_with_fitness.get(&i).unwrap_or(&0.0).clone();

            for j in i + 1..len {
                let rhs = genes[j].clone();

                let mut rhs_fitness = genes_with_fitness.get(&j).unwrap_or(&0.0).clone();

                match self.play_game_with(lhs.clone(), rhs.clone()) {
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

                match self.play_game_with(rhs.clone(), lhs.clone()) {
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

                genes_with_fitness.insert(j, rhs_fitness);
            }

            genes_with_fitness.insert(i, lhs_fitness);
        }

        genes_with_fitness
            .iter()
            .map(|(i, fitness)| (genes[*i].clone(), *fitness))
            .collect()
    }

    /// Plays a game with the given genes
    ///
    /// # Arguments
    /// * `lhs` - The first gene
    /// * `rhs` - The second gene
    /// # Returns
    /// The result of the game
    fn play_game_with(&self, lhs: Gene, rhs: Gene) -> GameResult {
        Game::new(
            Box::new(MiniMaxAgent::new(
                self.depth,
                self.quiescence_depth,
                ParameterizedHeuristic::new(One, lhs.get_values()),
            )),
            Box::new(MiniMaxAgent::new(
                self.depth,
                self.quiescence_depth,
                ParameterizedHeuristic::new(Two, rhs.get_values()),
            )),
        )
        .play()
    }
}
