use rand::Rng;
use crate::agent::Agent;
use crate::ultimate_board::UltimateBoard;

/// Struct representing an agent that plays randomly
pub struct RandomAgent {}

impl RandomAgent {
	pub fn new() -> RandomAgent {
		RandomAgent {}
	}
}

impl Agent for RandomAgent {
	fn act(&mut self, board: UltimateBoard) -> u8 {
		let possible_moves: Vec<_> = board.get_possible_moves().collect();
		
		possible_moves[rand::thread_rng().gen_range(0..possible_moves.len())]
	}
	fn reset(&mut self) {}
}