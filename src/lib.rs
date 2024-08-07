//! # Ultimate Tic Tac Toe implementation, game agents, heuristics and benchmarking tools.
//!
//! This library provides a full implementation of the Ultimate Tic Tac Toe game.
//! The game can be played by two agents.
//!
//! ## Provided agents:
//! * [HumanAgent](agents::human_agent::HumanAgent): An agent that requires user input to play.
//! * [RandomAgent](random_agent::RandomAgent): An agent that plays random moves.
//! * [MiniMaxAgent](minimax_agent::MiniMaxAgent): An agent that uses the minimax algorithm to determine the best move.
//!
//! A custom agent can be implemented by implementing the [Agent](agent::Agent) trait.
//!
//! ## Provided heuristics:
//! * [CustomHeuristic](custom_heuristic::CustomHeuristic): A heuristic that uses a custom evaluation function.
//! * [MonteCarloGameSearchHeuristic](monte_carlo_game_search_heuristic::MonteCarloGameSearchHeuristic): A heuristic that uses Monte Carlo Tree Search to evaluate the best move.
//! * [ParameterizedHeuristic](parameterized_heuristic::ParameterizedHeuristic): A heuristic that uses a parameterized evaluation function.
//!
//! A custom heuristic can be implemented by implementing the [Heuristic](heuristic::Heuristic) and [MiniBoardHeuristic](heuristic::MiniBoardHeuristic) trait.
//!
//! ## Genetic algorithm
//! The library also contains a [GeneticAlgorithm](genetic_algorithm::GeneticAlgorithm) as well as various [Selection](genetic_algorithm::mutations), [Mutation](genetic_algorithm::mutations) and [Recombination](genetic_algorithm::recombinations) operators to optimize the weights of the [ParameterizedHeuristic](parameterized_heuristic::ParameterizedHeuristic).
//!
//! # Usage
//! Initialize a game with two agents and play it:
//! ```rust
//! use hausarbeit::heuristics::custom_heuristic::CustomHeuristic;
//! use hausarbeit::game::Game;
//! use hausarbeit::agents::minimax_agent::MiniMaxAgent;
//! use hausarbeit::game::player::Player;
//! use hausarbeit::agents::random_agent::RandomAgent;
//!
//! let agent_one = MiniMaxAgent::new(2, 1, CustomHeuristic::new(Player::One));
//! let agent_two = RandomAgent::new();
//!
//! let mut game = Game::new(Box::new(agent_one), Box::new(agent_two));
//!
//! game.play();
//! ```

pub mod agent;
pub mod agents;
pub mod game;
pub mod genetic_algorithm;
pub mod heuristic;
pub mod heuristics;
