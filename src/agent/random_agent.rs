//! # Contains the [RandomAgent] struct
//! The RandomAgent struct represents an [Agent] that plays randomly.
//! The agent can be used to test other agent or to play against a human player.

use crate::agent::{Agent, AgentInfo};
use crate::game::player::Player;
use crate::game::ultimate_board::UltimateBoard;
use rand::Rng;

/// Struct representing an agent that plays randomly
pub struct RandomAgent {
    player: Player,
    turn: u32,
}

impl Default for RandomAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomAgent {
    pub fn new() -> Self {
        RandomAgent {
            player: Player::default(),
            turn: 0,
        }
    }
}

impl Agent for RandomAgent {
    fn act(&mut self, board: UltimateBoard, player: Player, turn: u32) -> Option<u8> {
        self.player = player;
        self.turn = turn;

        let possible_moves: Vec<_> = board.get_possible_moves().collect();

        Some(possible_moves[rand::thread_rng().gen_range(0..possible_moves.len())])
    }

    fn get_info(&self) -> AgentInfo {
        AgentInfo::new("RAND".to_string(), self.player, self.turn, "".to_string())
    }
}
