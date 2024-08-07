//! # Ultimate Tic Tac Toe implementation, game agent, heuristic and benchmarking tools
//!
//! This library provides a full implementation of the Ultimate Tic Tac Toe game.
//! The game can be played by two agents.
//!
//! ## Provided agents:
//! * [HumanAgent](agent::human_agent::HumanAgent): An agent that requires user input to play.
//! * [MiniMaxAgent](agent::minimax_agent::MiniMaxAgent): An agent that uses the minimax algorithm to determine the best move.
//! * [MonteCarloTreeAgent](agent::monte_carlo_tree_agent::MonteCarloTreeAgent): An agent that uses the Monte Carlo Tree Search algorithm to determine the best move.
//! * [RandomAgent](agent::random_agent::RandomAgent): An agent that plays random moves.
//!
//! ## Utility agents:
//! * [BenchedAgent](agent::benched::BenchedAgent): An agent that logs the time it takes to make a move.
//! * [RandomStartAgent](agent::random_start::RandomStartAgent): An agent that uses a random agent for the first `depth` turns, then switches to another agent.
//!
//! A custom agent can be implemented by implementing the [Agent](agent::Agent) trait.
//!
//! ## Provided heuristics:
//! * [CustomHeuristic](heuristic::custom_heuristic::CustomHeuristic): A heuristic that uses a custom evaluation function.
//! * [MonteCarloGameSearchHeuristic](heuristic::monte_carlo_game_search_heuristic::MonteCarloGameSearchHeuristic): A heuristic that uses Monte Carlo Tree Search to evaluate the best move.
//! * [ParameterizedHeuristic](heuristic::parameterized_heuristic::ParameterizedHeuristic): A heuristic that uses a parameterized evaluation function.
//!
//! A custom heuristic can be implemented by implementing the [Heuristic](heuristic::Heuristic) and [MiniBoardHeuristic](heuristic::MiniBoardHeuristic) trait.
//!
//! ## Genetic algorithm
//! The library also contains a [GeneticAlgorithm](genetic_algorithm::GeneticAlgorithm) as well as various [Selection](genetic_algorithm::selection), [Mutation](genetic_algorithm::mutation), [Recombination](genetic_algorithm::recombination) and [Fitness](genetic_algorithm::fitness) operators to optimize the weights of the [ParameterizedHeuristic](heuristic::parameterized_heuristic::ParameterizedHeuristic).
//!
//! # Usage
//! Initialize a game with two agents and play it:
//! ```rust
//! use hausarbeit::agent::minimax_agent::MiniMaxAgent;
//! use hausarbeit::agent::random_agent::RandomAgent;
//! use hausarbeit::heuristic::custom_heuristic::CustomHeuristic;
//! use hausarbeit::game::Game;
//! use hausarbeit::game::player::Player;
//!
//! let agent_one = MiniMaxAgent::new(2, 1, CustomHeuristic::new(Player::One));
//! let agent_two = RandomAgent::new();
//!
//! let mut game = Game::new(Box::new(agent_one), Box::new(agent_two));
//!
//! game.play();
//! ```

pub mod agent;
pub mod game;
pub mod genetic_algorithm;
pub mod heuristic;
pub mod runtime_test;
