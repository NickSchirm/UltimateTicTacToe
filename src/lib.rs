//! # Ultimate Tic Tac Toe implementation, game agents and benchmarking tools.
//!
//! This library provides a full implementation of the Ultimate Tic Tac Toe game.
//! The game can be played by two agents.
//!
//! Provided agents:
//! * [RandomAgent](random_agent::RandomAgent): An agent that plays random moves
//! * [MiniMaxAgent](minimax_agent::MiniMaxAgent): An agent that uses the minimax algorithm to determine the best move
//!
//! Provided heuristics:
//! * [CustomHeuristic](custom_heuristic::CustomHeuristic): A heuristic that uses a custom evaluation function
//! * [MonteCarloGameSearchHeuristic](monte_carlo_game_search_heuristic::MonteCarloGameSearchHeuristic): A heuristic that uses Monte Carlo Tree Search to evaluate the best move
//!
//! # Usage
//! Initialize a game with two agents and play it:
//! ```rust
//! use hausarbeit::game::Game;
//! use hausarbeit::random_agent::RandomAgent;
//!
//! let agent_one = RandomAgent::new();
//! let agent_two = RandomAgent::new();
//!
//! let mut game = Game::new(Box::new(agent_one), Box::new(agent_two));
//!
//! game.play();
//! ```

pub mod agent;
pub mod bitboard;
pub mod board;
pub mod custom_heuristic;
pub mod game;
pub mod game_result;
pub mod heuristic;
pub mod human_agent;
pub mod minimax_agent;
pub mod monte_carlo_game_search_heuristic;
pub mod player;
pub mod random_agent;
pub mod ultimate_board;
