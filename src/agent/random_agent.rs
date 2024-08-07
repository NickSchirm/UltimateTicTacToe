//! # Contains the [RandomAgent] struct
//! The RandomAgent struct represents an [Agent] that plays randomly.
//! The agent can be used to test other agent or to play against a human player.

use crate::agent::Agent;
use crate::game::ultimate_board::UltimateBoard;
use rand::Rng;

/// Struct representing an agent that plays randomly
pub struct RandomAgent {}

impl Default for RandomAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomAgent {
    pub fn new() -> Self {
        RandomAgent {}
    }
}

impl Agent for RandomAgent {
    fn act(&mut self, board: UltimateBoard) -> Option<u8> {
        let possible_moves: Vec<_> = board.get_possible_moves().collect();

        Some(possible_moves[rand::thread_rng().gen_range(0..possible_moves.len())])
    }
}
