//! # Contains the [FitnessFunction] trait and implementations

use crate::agent::minimax_agent::MiniMaxAgent;
use crate::game::Game;
use crate::game::game_result::GameResult;
use crate::game::player::Player::{One, Two};
use crate::genetic_algorithm::gene::Gene;
use crate::heuristic::parameterized_heuristic::ParameterizedHeuristic;

pub mod full_ordering_fitness;

/// # Trait representing a fitness function
///
/// The fitness function is used to calculate the fitness of the genes.
///
/// The fitness is a value that represents how good the genes are.
/// The higher the fitness, the better the genes.
///
/// The fitness can be calculated in different ways.
///
/// The Fitness function may be multithreaded.
pub trait FitnessFunction {
	/// Calculates the fitness of the given genes
	///
	/// # Arguments
	/// * `genes` - The genes to calculate the fitness for
	/// # Returns
	/// The genes with their fitness
	fn calculate_fitness(&self, genes: Vec<Gene>) -> Vec<(Gene, f64)>;

	/// Plays a game with the given genes
	///
	/// # Arguments
	/// * `lhs` - The first gene
	/// * `rhs` - The second gene
	/// # Returns
	/// The result of the game
	fn play_game_with(&self, lhs: Gene, rhs: Gene, depth: u32, quiescence_depth: u32) -> GameResult {
		Game::new(
			Box::new(MiniMaxAgent::new(
				depth,
				quiescence_depth,
				ParameterizedHeuristic::new(One, lhs.get_values()),
			)),
			Box::new(MiniMaxAgent::new(
				depth,
				quiescence_depth,
				ParameterizedHeuristic::new(Two, rhs.get_values()),
			)),
		).play()
	}
}